Score(3000)=0.684 I=0.796 C=0.587 ns_rows≤3K=17/49 (reached=9 partial=0 missing=8) grid(1000/1442/2080/3000/4327/6240/9000)=0.522/0.473/0.597/0.684/0.621/0.599/0.631

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
| ns | 471 |  | 147 | README: the canonical `requests.get(...)` doctest + every `##` heading | 1.5 | 1.1 | 0.337 |
| ns | 589 |  | 118 | Full package metadata block (`__version__.py`) | 1.6 | 1.1 | 0.316 |
| walker |  | 620 | 356 | plaintext config Makefile |  |  | 0.322 |
| ns | 688 |  | 99 | CI: `.github/` and workflow file roster (complete) | 1.7 |  | 0.286 |
| walker |  | 709 | 89 | listing of 'src/requests' |  |  | 0.456 |
| ns | 768 |  | 80 | Makefile: install / test / CI targets | 1.8 |  | 0.477 |
| ns | 896 |  | 128 | `docs/` tree listing (complete) | 1.9 |  | 0.522 |
| ns | 1198 |  | 302 | pyproject: build backend, metadata, runtime dependencies | 1.10 |  | 0.473 |
| ns | 1469 |  | 271 | `requests/__init__.py`: name → module re-export map | 2.1 |  | 0.431 |
| ns | 1543 |  | 74 | `api.py`: names of all eight module-level functions | 2.2 |  | 0.421 |
| walker |  | 1711 | 1002 | python imports in src/requests/__init__.py |  |  | 0.509 |
| walker |  | 1745 | 34 | python decl names surface in src/requests/__init__.py |  |  | 0.509 |
| walker |  | 1761 | 16 | python decl at src/requests/__init__.py:99 |  |  | 0.509 |
| ns | 1896 |  | 353 | `exceptions.py`: the complete class hierarchy | 2.3 |  | 0.476 |
| walker |  | 1936 | 175 | README headline in README.md |  |  | 0.588 |
| walker |  | 1976 | 40 | headings outline in README.md |  |  | 0.610 |
| walker |  | 2023 | 47 | python decl at src/requests/__init__.py:60 |  |  | 0.610 |
| walker |  | 2067 | 44 | listing of '.github' |  |  | 0.624 |
| ns | 2072 |  | 176 | `Session` class declaration + docstring | 2.4 |  | 0.597 |
| walker |  | 2107 | 40 | listing of '.github/workflows' |  |  | 0.629 |
| walker |  | 2165 | 58 | [package] in pyproject.toml |  |  | 0.631 |
| walker |  | 2312 | 147 | README.md section #0 |  |  | 0.631 |
| walker |  | 2325 | 13 | listing of '.github/ISSUE_TEMPLATE' |  |  | 0.648 |
| ns | 2343 |  | 271 | `Session` attribute set: typed fields + `__attrs__` | 2.5 | 2.4 | 0.610 |
| walker |  | 2402 | 77 | manifest config in pyproject.toml |  |  | 0.615 |
| walker |  | 2422 | 20 | python imports in setup.py |  |  | 0.615 |
| walker |  | 2504 | 82 | tool.setuptools config in pyproject.toml |  |  | 0.615 |
| ns | 2631 |  | 288 | `Session` method roster (complete, with line coordinates) | 2.6 | 2.4 | 0.592 |
| walker |  | 2643 | 139 | [dependencies] in pyproject.toml |  |  | 0.603 |
| walker |  | 2706 | 63 | README.md section #1 |  |  | 0.603 |
| walker |  | 2831 | 125 | package metadata in pyproject.toml |  |  | 0.640 |
| ns | 2882 |  | 251 | `Session.request`: the complete keyword signature | 2.7 | 2.6 | 0.619 |
| walker |  | 2909 | 78 | listing of 'tests' |  |  | 0.652 |
| walker |  | 2918 | 9 | listing of 'tests/testserver' |  |  | 0.662 |
| walker |  | 2934 | 16 | listing of 'tests/certs' |  |  | 0.684 |
| walker |  | 2960 | 26 | python imports in src/requests/packages.py |  |  | 0.684 |
| ns | 3065 |  | 183 | `Session.__init__`: every default value | 2.8 | 2.6 | 0.667 |
| walker |  | 3136 | 176 | README.md section #2 |  |  | 0.667 |
| walker |  | 3209 | 73 | python decl names surface in src/requests/utils.py |  |  | 0.667 |
| walker |  | 3209 | 0 | python decl at src/requests/utils.py:283 |  |  | 0.667 |
| walker |  | 3209 | 0 | python decl at src/requests/utils.py:290 |  |  | 0.667 |
| walker |  | 3230 | 21 | python decl at src/requests/utils.py:328 |  |  | 0.667 |
| walker |  | 3263 | 33 | python decl at src/requests/utils.py:231 |  |  | 0.667 |
| walker |  | 3280 | 17 | python decl doc at src/requests/utils.py:283 |  |  | 0.667 |
| walker |  | 3297 | 17 | python decl doc at src/requests/utils.py:328 |  |  | 0.667 |
| walker |  | 3334 | 37 | python decl at src/requests/utils.py:149 |  |  | 0.667 |
| walker |  | 3347 | 13 | python decl doc at src/requests/utils.py:149 |  |  | 0.667 |
| walker |  | 3366 | 19 | python decl doc at src/requests/utils.py:231 |  |  | 0.667 |
| ns | 3418 |  | 353 | `models.py` module map: constants, all five classes, `Request` fields | 3.1 |  | 0.638 |
| walker |  | 3425 | 59 | python decl doc at src/requests/utils.py:290 |  |  | 0.638 |
| walker |  | 3482 | 57 | python decl names surface in src/requests/sessions.py |  |  | 0.638 |
| walker |  | 3482 | 0 | python decl at src/requests/sessions.py:127 |  |  | 0.638 |
| walker |  | 3482 | 0 | python decl at src/requests/sessions.py:395 |  |  | 0.638 |
| walker |  | 3482 | 0 | python decl at src/requests/sessions.py:908 |  |  | 0.638 |
| walker |  | 3490 | 8 | python decl body at src/requests/sessions.py:908 body 920 |  |  | 0.638 |
| walker |  | 3521 | 31 | python decl at src/requests/sessions.py:76 |  |  | 0.638 |
| walker |  | 3574 | 53 | python decl at src/requests/sessions.py:108 |  |  | 0.638 |
| walker |  | 3606 | 32 | python class body at src/requests/sessions.py:127 |  |  | 0.638 |
| ns | 3739 |  | 321 | `PreparedRequest`: attributes + every `prepare_*` step | 3.2 | 3.1 | 0.617 |
| walker |  | 3879 | 273 | python class body at src/requests/sessions.py:395 |  |  | 0.668 |
| walker |  | 3936 | 57 | python decl doc at src/requests/sessions.py:108 |  |  | 0.668 |
| ns | 4042 |  | 303 | `Response` attribute set: typed fields + `__attrs__` | 3.3 | 3.1 | 0.638 |
| ns | 4324 |  | 282 | `Response`: `__init__`, dunders and every `@property` | 3.4 | 3.3 | 0.621 |
| walker |  | 4410 | 474 | python method sigs in src/requests/sessions.py |  |  | 0.654 |
| walker |  | 4410 | 0 | python method at src/requests/sessions.py:132 |  |  | 0.654 |
| walker |  | 4410 | 0 | python method at src/requests/sessions.py:134 |  |  | 0.654 |
| walker |  | 4410 | 0 | python method at src/requests/sessions.py:154 |  |  | 0.654 |
| walker |  | 4410 | 0 | python method at src/requests/sessions.py:505 |  |  | 0.654 |
| walker |  | 4410 | 0 | python method at src/requests/sessions.py:508 |  |  | 0.654 |
| walker |  | 4410 | 0 | python method at src/requests/sessions.py:511 |  |  | 0.654 |
| walker |  | 4410 | 0 | python method at src/requests/sessions.py:673 |  |  | 0.654 |
| walker |  | 4410 | 0 | python method at src/requests/sessions.py:684 |  |  | 0.654 |
| walker |  | 4410 | 0 | python method at src/requests/sessions.py:742 |  |  | 0.654 |
| walker |  | 4410 | 0 | python method at src/requests/sessions.py:752 |  |  | 0.654 |
| walker |  | 4410 | 0 | python method at src/requests/sessions.py:870 |  |  | 0.654 |
| walker |  | 4410 | 0 | python method at src/requests/sessions.py:883 |  |  | 0.654 |
| walker |  | 4410 | 0 | python method at src/requests/sessions.py:888 |  |  | 0.654 |
| walker |  | 4410 | 0 | python method at src/requests/sessions.py:899 |  |  | 0.654 |
| walker |  | 4410 | 0 | python method at src/requests/sessions.py:903 |  |  | 0.654 |
| walker |  | 4410 | 0 | python method body at src/requests/sessions.py:132 body 132 |  |  | 0.654 |
| walker |  | 4436 | 26 | python method at src/requests/sessions.py:309 |  |  | 0.654 |
| walker |  | 4462 | 26 | python method at src/requests/sessions.py:370 |  |  | 0.654 |
| ns | 4476 |  | 152 | `Response`: the remaining members (`iter_content` … `close`) | 3.5 | 3.4 | 0.644 |
| walker |  | 4478 | 16 | python method doc at src/requests/sessions.py:883 |  |  | 0.644 |
| walker |  | 4496 | 18 | python method doc at src/requests/sessions.py:154 |  |  | 0.645 |
| walker |  | 4516 | 20 | python method doc at src/requests/sessions.py:134 |  |  | 0.645 |
| walker |  | 4559 | 43 | python method at src/requests/sessions.py:714 |  |  | 0.645 |
| walker |  | 4602 | 43 | python method at src/requests/sessions.py:728 |  |  | 0.645 |
| walker |  | 4647 | 45 | python method at src/requests/sessions.py:334 |  |  | 0.645 |
| ns | 4691 |  | 215 | `adapters.py`: pool defaults + the `BaseAdapter` contract | 3.6 |  | 0.630 |
| walker |  | 4697 | 50 | python method at src/requests/sessions.py:442 |  |  | 0.630 |
| walker |  | 4755 | 58 | python method at src/requests/sessions.py:655 |  |  | 0.630 |
| walker |  | 4789 | 34 | python method doc at src/requests/sessions.py:752 |  |  | 0.630 |
| walker |  | 4856 | 67 | python decl doc at src/requests/sessions.py:76 |  |  | 0.630 |
| walker |  | 4895 | 39 | python method doc at src/requests/sessions.py:370 |  |  | 0.630 |
| walker |  | 4936 | 41 | python method doc at src/requests/sessions.py:888 |  |  | 0.630 |
| walker |  | 5008 | 72 | python method at src/requests/sessions.py:695 |  |  | 0.630 |
| walker |  | 5044 | 36 | python decl names surface in src/requests/help.py |  |  | 0.630 |
| walker |  | 5044 | 0 | python decl at src/requests/help.py:37 |  |  | 0.630 |
| walker |  | 5044 | 0 | python decl at src/requests/help.py:69 |  |  | 0.630 |
| walker |  | 5044 | 0 | python decl at src/requests/help.py:128 |  |  | 0.630 |
| ns | 5055 |  | 364 | `HTTPAdapter`: constructor knobs + complete method roster | 3.7 | 3.6 | 0.607 |
| walker |  | 5057 | 13 | python decl doc at src/requests/help.py:69 |  |  | 0.607 |
| walker |  | 5071 | 14 | python decl doc at src/requests/help.py:128 |  |  | 0.607 |
| walker |  | 5089 | 18 | python decl body at src/requests/help.py:128 body 130 |  |  | 0.607 |
| walker |  | 5134 | 45 | python method doc at src/requests/sessions.py:870 |  |  | 0.607 |
| walker |  | 5215 | 81 | python method at src/requests/sessions.py:831 |  |  | 0.607 |
| walker |  | 5257 | 42 | python method doc at src/requests/sessions.py:831 |  |  | 0.607 |
| ns | 5324 |  | 269 | `SessionRedirectMixin` + session-module merge helpers | 3.8 |  | 0.611 |
| walker |  | 5346 | 89 | python decl names surface in src/requests/adapters.py |  |  | 0.613 |
| walker |  | 5346 | 0 | python decl at src/requests/adapters.py:122 |  |  | 0.613 |
| walker |  | 5346 | 0 | python decl at src/requests/adapters.py:158 |  |  | 0.613 |
| walker |  | 5357 | 11 | python decl doc at src/requests/adapters.py:122 |  |  | 0.614 |
| walker |  | 5372 | 15 | python decl doc at src/requests/adapters.py:158 |  |  | 0.614 |
| walker |  | 5526 | 154 | python class body at src/requests/adapters.py:158 |  |  | 0.614 |
| ns | 5560 |  | 236 | `cookies.py` module map: classes + free functions | 3.9 |  | 0.604 |
| walker |  | 5811 | 285 | python method sigs in src/requests/adapters.py |  |  | 0.617 |
| walker |  | 5811 | 0 | python method at src/requests/adapters.py:125 |  |  | 0.617 |
| walker |  | 5811 | 0 | python method at src/requests/adapters.py:153 |  |  | 0.617 |
| walker |  | 5811 | 0 | python method at src/requests/adapters.py:223 |  |  | 0.617 |
| walker |  | 5811 | 0 | python method at src/requests/adapters.py:269 |  |  | 0.617 |
| walker |  | 5811 | 0 | python method at src/requests/adapters.py:365 |  |  | 0.617 |
| walker |  | 5811 | 0 | python method at src/requests/adapters.py:555 |  |  | 0.617 |
| walker |  | 5811 | 0 | python method at src/requests/adapters.py:599 |  |  | 0.617 |
| walker |  | 5811 | 0 | python method at src/requests/adapters.py:613 |  |  | 0.617 |
| walker |  | 5824 | 13 | python method doc at src/requests/adapters.py:153 |  |  | 0.619 |
| walker |  | 5855 | 31 | python method at src/requests/adapters.py:565 |  |  | 0.619 |
| ns | 5868 |  | 308 | `auth.py`: every auth handler and its methods | 3.10 |  | 0.605 |
| walker |  | 5889 | 34 | python method at src/requests/adapters.py:512 |  |  | 0.605 |
| walker |  | 5928 | 39 | python method at src/requests/adapters.py:307 |  |  | 0.605 |
| walker |  | 5968 | 40 | python method at src/requests/adapters.py:226 |  |  | 0.605 |
| walker |  | 6016 | 48 | python method at src/requests/adapters.py:403 |  |  | 0.605 |
| walker |  | 6075 | 59 | python method at src/requests/adapters.py:239 |  |  | 0.605 |
| walker |  | 6147 | 72 | python method at src/requests/adapters.py:455 |  |  | 0.605 |
| walker |  | 6223 | 76 | python method at src/requests/adapters.py:201 |  |  | 0.615 |
| ns | 6240 |  | 372 | `_types.py`: public type-alias roster | 4.1 |  | 0.599 |
| walker |  | 6276 | 53 | python method doc at src/requests/adapters.py:555 |  |  | 0.599 |
| walker |  | 6386 | 110 | declaration surface of tox.ini |  |  | 0.599 |
| walker |  | 6543 | 157 | python decl names surface in src/requests/compat.py |  |  | 0.599 |
| walker |  | 6543 | 0 | python decl at src/requests/compat.py:37 |  |  | 0.599 |
| ns | 6551 |  | 311 | `_types.py`: the `Unpack` kwargs TypedDicts | 4.2 | 4.1 | 0.584 |
| walker |  | 6555 | 12 | python decl doc at src/requests/compat.py:37 |  |  | 0.584 |
| walker |  | 6611 | 56 | python method doc at src/requests/sessions.py:309 |  |  | 0.584 |
| walker |  | 6617 | 6 | listing of 'tests/certs/valid' |  |  | 0.584 |
| walker |  | 6714 | 97 | python method at src/requests/adapters.py:128 |  |  | 0.600 |
| ns | 6750 |  | 199 | Environment-driven settings: the `merge_environment_settings` core | 4.3 | 2.6 | 0.591 |
| walker |  | 6811 | 97 | python method at src/requests/adapters.py:634 |  |  | 0.608 |
| walker |  | 6836 | 25 | python imports in tests/__init__.py |  |  | 0.608 |
| ns | 6857 |  | 107 | `utils.py` module-level constants | 5.1 |  | 0.604 |
| ns | 7005 |  | 148 | `utils.py` function roster 1/3: proxies shim, lengths, netrc, encoders | 5.2 |  | 0.599 |
| walker |  | 7092 | 256 | README.md section #3 |  |  | 0.599 |
| walker |  | 7142 | 50 | python decl names surface in src/requests/status_codes.py |  |  | 0.599 |
| walker |  | 7142 | 0 | python decl at src/requests/status_codes.py:109 |  |  | 0.599 |
| ns | 7172 |  | 167 | `utils.py` function roster 2/3: cookies, encoding detection, URI quoting, CIDR | 5.3 | 5.2 | 0.591 |
| walker |  | 7288 | 146 | python decl names surface in src/requests/models.py |  |  | 0.596 |
| walker |  | 7288 | 0 | python decl at src/requests/models.py:109 |  |  | 0.596 |
| walker |  | 7288 | 0 | python decl at src/requests/models.py:255 |  |  | 0.596 |
| walker |  | 7288 | 0 | python decl at src/requests/models.py:283 |  |  | 0.596 |
| walker |  | 7288 | 0 | python decl at src/requests/models.py:376 |  |  | 0.596 |
| walker |  | 7288 | 0 | python decl at src/requests/models.py:730 |  |  | 0.596 |
| walker |  | 7306 | 18 | python decl doc at src/requests/models.py:283 |  |  | 0.596 |
| walker |  | 7345 | 39 | python decl doc at src/requests/models.py:730 |  |  | 0.596 |
| ns | 7352 |  | 180 | `utils.py` function roster 3/3: proxy resolution, default headers, header validation | 5.4 | 5.3 | 0.588 |
| walker |  | 7356 | 11 | python class body at src/requests/models.py:109 |  |  | 0.588 |
| ns | 7613 |  | 261 | `structures.py`: `CaseInsensitiveDict` and `LookupDict` in full shape | 5.5 |  | 0.578 |
| walker |  | 7615 | 259 | python class body at src/requests/models.py:730 |  |  | 0.611 |
| ns | 7723 |  | 110 | `status_codes.py`: how `requests.codes` is built | 5.6 |  | 0.607 |
| walker |  | 7738 | 123 | python class body at src/requests/models.py:283 |  |  | 0.618 |
| walker |  | 7837 | 99 | python class body at src/requests/models.py:376 |  |  | 0.620 |
| walker |  | 7911 | 74 | python decl at src/requests/models.py:96 |  |  | 0.630 |
| walker |  | 7928 | 17 | python class body at src/requests/models.py:255 |  |  | 0.630 |
| ns | 7965 |  | 242 | The five small modules: hooks, `_internal_utils`, compat, certs, packages | 5.7 |  | 0.621 |
| ns | 8142 |  | 177 | `help.py`: the bug-report payload | 5.8 |  | 0.614 |
| ns | 8475 |  | 333 | `resolve_redirects`: the rules the redirect loop enforces | 6.1 | 3.8 | 0.602 |
| walker |  | 8629 | 701 | python method sigs in src/requests/models.py |  |  | 0.640 |
| walker |  | 8629 | 0 | python method at src/requests/models.py:271 |  |  | 0.640 |
| walker |  | 8629 | 0 | python method at src/requests/models.py:355 |  |  | 0.640 |
| walker |  | 8629 | 0 | python method at src/requests/models.py:358 |  |  | 0.640 |
| walker |  | 8629 | 0 | python method at src/requests/models.py:451 |  |  | 0.640 |
| walker |  | 8629 | 0 | python method at src/requests/models.py:454 |  |  | 0.640 |
| walker |  | 8629 | 0 | python method at src/requests/models.py:465 |  |  | 0.640 |
| walker |  | 8629 | 0 | python method at src/requests/models.py:563 |  |  | 0.640 |
| walker |  | 8629 | 0 | python method at src/requests/models.py:652 |  |  | 0.640 |
| walker |  | 8629 | 0 | python method at src/requests/models.py:720 |  |  | 0.640 |
| walker |  | 8629 | 0 | python method at src/requests/models.py:763 |  |  | 0.640 |
| walker |  | 8629 | 0 | python method at src/requests/models.py:810 |  |  | 0.640 |
| walker |  | 8629 | 0 | python method at src/requests/models.py:813 |  |  | 0.640 |
| walker |  | 8629 | 0 | python method at src/requests/models.py:824 |  |  | 0.640 |
| walker |  | 8629 | 0 | python method at src/requests/models.py:832 |  |  | 0.640 |
| walker |  | 8629 | 0 | python method at src/requests/models.py:835 |  |  | 0.640 |
| walker |  | 8629 | 0 | python method at src/requests/models.py:845 |  |  | 0.640 |
| walker |  | 8629 | 0 | python method at src/requests/models.py:855 |  |  | 0.640 |
| walker |  | 8629 | 0 | python method at src/requests/models.py:1087 |  |  | 0.640 |
| walker |  | 8629 | 0 | python method at src/requests/models.py:1140 |  |  | 0.640 |
| walker |  | 8629 | 0 | python method at src/requests/models.py:1169 |  |  | 0.640 |
| walker |  | 8635 | 6 | python method at src/requests/models.py:112 |  |  | 0.640 |
| walker |  | 8643 | 8 | python method at src/requests/models.py:859 |  |  | 0.641 |
| walker |  | 8651 | 8 | python method at src/requests/models.py:874 |  |  | 0.642 |
| walker |  | 8659 | 8 | python method at src/requests/models.py:881 |  |  | 0.644 |
| walker |  | 8667 | 8 | python method at src/requests/models.py:889 |  |  | 0.645 |
| walker |  | 8675 | 8 | python method at src/requests/models.py:894 |  |  | 0.646 |
| walker |  | 8684 | 9 | python method at src/requests/models.py:1030 |  |  | 0.647 |
| walker |  | 8693 | 9 | python method at src/requests/models.py:1049 |  |  | 0.649 |
| walker |  | 8702 | 9 | python method at src/requests/models.py:1122 |  |  | 0.650 |
| walker |  | 8716 | 14 | python method at src/requests/models.py:405 |  |  | 0.650 |
| walker |  | 8724 | 8 | python method at src/requests/models.py:471 |  |  | 0.652 |
| walker |  | 8736 | 12 | python method doc at src/requests/models.py:720 |  |  | 0.652 |
| walker |  | 8749 | 13 | python method doc at src/requests/models.py:112 |  |  | 0.652 |
| walker |  | 8762 | 13 | python method doc at src/requests/models.py:465 |  |  | 0.652 |
| ns | 8764 |  | 289 | `HTTPAdapter.send`: the urllib3 call and the `MaxRetryError` fan-out | 6.2 | 3.7 | 0.639 |
| walker |  | 8775 | 13 | python method doc at src/requests/models.py:563 |  |  | 0.639 |
| walker |  | 8804 | 29 | python method at src/requests/models.py:816 |  |  | 0.639 |
| walker |  | 8819 | 15 | python method doc at src/requests/models.py:1030 |  |  | 0.639 |
| walker |  | 8852 | 33 | python method at src/requests/models.py:697 |  |  | 0.639 |
| walker |  | 8868 | 16 | python method doc at src/requests/models.py:855 |  |  | 0.639 |
| walker |  | 8885 | 17 | python method doc at src/requests/models.py:652 |  |  | 0.639 |
| walker |  | 8921 | 36 | python method at src/requests/models.py:258 |  |  | 0.639 |
| walker |  | 8933 | 12 | python method doc at src/requests/models.py:258 |  |  | 0.639 |
| ns | 8966 |  | 202 | `HTTPAdapter.send`: the remaining `except` arms and the return | 6.3 | 6.2 | 0.631 |
| walker |  | 8969 | 36 | python method at src/requests/models.py:912 |  |  | 0.631 |
| walker |  | 8987 | 18 | python method doc at src/requests/models.py:881 |  |  | 0.631 |
| walker |  | 9005 | 18 | python method doc at src/requests/models.py:1140 |  |  | 0.631 |
| walker |  | 9024 | 19 | python method doc at src/requests/models.py:1122 |  |  | 0.631 |
| ns | 9054 |  | 88 | Makefile: the remaining targets (coverage, publish, docs) | 7.1 | 1.8 | 0.634 |
| walker |  | 9063 | 39 | python method at src/requests/models.py:574 |  |  | 0.634 |
| walker |  | 9077 | 14 | python method doc at src/requests/models.py:574 |  |  | 0.634 |
| walker |  | 9117 | 40 | python method at src/requests/models.py:481 |  |  | 0.634 |
| walker |  | 9130 | 13 | python method doc at src/requests/models.py:481 |  |  | 0.634 |
| walker |  | 9171 | 41 | python method at src/requests/models.py:668 |  |  | 0.634 |
| walker |  | 9185 | 14 | python method doc at src/requests/models.py:668 |  |  | 0.634 |
| walker |  | 9207 | 22 | python method doc at src/requests/models.py:894 |  |  | 0.634 |
| walker |  | 9231 | 24 | python method doc at src/requests/models.py:358 |  |  | 0.634 |
| walker |  | 9255 | 24 | python method doc at src/requests/models.py:889 |  |  | 0.634 |
| walker |  | 9311 | 56 | python method at src/requests/models.py:908 |  |  | 0.634 |
| walker |  | 9311 | 0 | python method body at src/requests/models.py:908 body 911 |  |  | 0.634 |
| ns | 9337 |  | 283 | pyproject: every remaining table, and the settings that change how you work here | 7.2 | 1.10 | 0.626 |
| walker |  | 9368 | 57 | python method at src/requests/models.py:904 |  |  | 0.626 |
| walker |  | 9368 | 0 | python method body at src/requests/models.py:904 body 907 |  |  | 0.626 |
| walker |  | 9429 | 61 | python method at src/requests/models.py:990 |  |  | 0.626 |
| walker |  | 9464 | 35 | python method doc at src/requests/models.py:271 |  |  | 0.626 |
| ns | 9492 |  | 155 | `tests/conftest.py`: the fixture set every test builds on | 7.3 |  | 0.621 |
| walker |  | 9510 | 46 | python method doc at src/requests/models.py:874 |  |  | 0.621 |
| ns | 9587 |  | 95 | `tests/test_requests.py`: top-level class roster | 7.4 |  | 0.617 |
| walker |  | 9592 | 82 | python method at src/requests/models.py:975 |  |  | 0.617 |
| walker |  | 9592 | 0 | python method body at src/requests/models.py:975 body 981 |  |  | 0.617 |
| walker |  | 9676 | 84 | python method at src/requests/models.py:982 |  |  | 0.617 |
| walker |  | 9676 | 0 | python method body at src/requests/models.py:982 body 989 |  |  | 0.617 |
| walker |  | 9712 | 36 | python method at src/requests/models.py:133 |  |  | 0.617 |
| walker |  | 9712 | 0 | python method body at src/requests/models.py:133 body 135 |  |  | 0.617 |
| walker |  | 9746 | 34 | python method at src/requests/models.py:137 |  |  | 0.617 |
| walker |  | 9746 | 0 | python method body at src/requests/models.py:137 body 139 |  |  | 0.617 |
| ns | 9769 |  | 182 | `tests/testserver/server.py`: the local socket server API (complete) | 7.5 |  | 0.611 |
| walker |  | 9787 | 41 | python method at src/requests/models.py:147 |  |  | 0.611 |
| walker |  | 9787 | 0 | python method body at src/requests/models.py:147 body 149 |  |  | 0.611 |
| walker |  | 9828 | 41 | python method at src/requests/models.py:151 |  |  | 0.611 |
| walker |  | 9871 | 43 | python method at src/requests/models.py:183 |  |  | 0.611 |
| ns | 9926 |  | 157 | `docs/api.rst`: every section of the developer interface | 7.6 |  | 0.605 |
| ns | 9974 |  | 48 | `HISTORY.md`: the changelog's head | 7.7 |  | 0.603 |
