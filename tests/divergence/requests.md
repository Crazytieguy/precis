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
| walker |  | 2973 | 39 | plaintext config docs/requirements.txt |  |  | 0.684 |
| walker |  | 2999 | 26 | python imports in src/requests/packages.py |  |  | 0.684 |
| ns | 3065 |  | 183 | `Session.__init__`: every default value | 2.8 | 2.6 | 0.667 |
| walker |  | 3175 | 176 | README.md section #2 |  |  | 0.667 |
| walker |  | 3248 | 73 | declaration surface of requirements-dev.txt |  |  | 0.667 |
| walker |  | 3305 | 57 | python decl names surface in src/requests/sessions.py |  |  | 0.667 |
| walker |  | 3305 | 0 | python decl at src/requests/sessions.py:127 |  |  | 0.667 |
| walker |  | 3305 | 0 | python decl at src/requests/sessions.py:395 |  |  | 0.667 |
| walker |  | 3305 | 0 | python decl at src/requests/sessions.py:908 |  |  | 0.667 |
| walker |  | 3313 | 8 | python decl body at src/requests/sessions.py:908 body 920 |  |  | 0.667 |
| walker |  | 3344 | 31 | python decl at src/requests/sessions.py:76 |  |  | 0.667 |
| walker |  | 3397 | 53 | python decl at src/requests/sessions.py:108 |  |  | 0.667 |
| ns | 3418 |  | 353 | `models.py` module map: constants, all five classes, `Request` fields | 3.1 |  | 0.638 |
| walker |  | 3429 | 32 | python class body at src/requests/sessions.py:127 |  |  | 0.638 |
| walker |  | 3702 | 273 | python class body at src/requests/sessions.py:395 |  |  | 0.690 |
| ns | 3739 |  | 321 | `PreparedRequest`: attributes + every `prepare_*` step | 3.2 | 3.1 | 0.668 |
| walker |  | 3759 | 57 | python decl doc at src/requests/sessions.py:108 |  |  | 0.668 |
| ns | 4042 |  | 303 | `Response` attribute set: typed fields + `__attrs__` | 3.3 | 3.1 | 0.638 |
| walker |  | 4233 | 474 | python method sigs in src/requests/sessions.py |  |  | 0.671 |
| walker |  | 4233 | 0 | python method at src/requests/sessions.py:132 |  |  | 0.671 |
| walker |  | 4233 | 0 | python method at src/requests/sessions.py:134 |  |  | 0.671 |
| walker |  | 4233 | 0 | python method at src/requests/sessions.py:154 |  |  | 0.671 |
| walker |  | 4233 | 0 | python method at src/requests/sessions.py:505 |  |  | 0.671 |
| walker |  | 4233 | 0 | python method at src/requests/sessions.py:508 |  |  | 0.671 |
| walker |  | 4233 | 0 | python method at src/requests/sessions.py:511 |  |  | 0.671 |
| walker |  | 4233 | 0 | python method at src/requests/sessions.py:673 |  |  | 0.671 |
| walker |  | 4233 | 0 | python method at src/requests/sessions.py:684 |  |  | 0.671 |
| walker |  | 4233 | 0 | python method at src/requests/sessions.py:742 |  |  | 0.671 |
| walker |  | 4233 | 0 | python method at src/requests/sessions.py:752 |  |  | 0.671 |
| walker |  | 4233 | 0 | python method at src/requests/sessions.py:870 |  |  | 0.671 |
| walker |  | 4233 | 0 | python method at src/requests/sessions.py:883 |  |  | 0.671 |
| walker |  | 4233 | 0 | python method at src/requests/sessions.py:888 |  |  | 0.671 |
| walker |  | 4233 | 0 | python method at src/requests/sessions.py:899 |  |  | 0.671 |
| walker |  | 4233 | 0 | python method at src/requests/sessions.py:903 |  |  | 0.671 |
| walker |  | 4233 | 0 | python method body at src/requests/sessions.py:132 body 132 |  |  | 0.671 |
| walker |  | 4259 | 26 | python method at src/requests/sessions.py:309 |  |  | 0.671 |
| walker |  | 4285 | 26 | python method at src/requests/sessions.py:370 |  |  | 0.671 |
| walker |  | 4301 | 16 | python method doc at src/requests/sessions.py:883 |  |  | 0.671 |
| walker |  | 4319 | 18 | python method doc at src/requests/sessions.py:154 |  |  | 0.671 |
| ns | 4324 |  | 282 | `Response`: `__init__`, dunders and every `@property` | 3.4 | 3.3 | 0.654 |
| walker |  | 4339 | 20 | python method doc at src/requests/sessions.py:134 |  |  | 0.654 |
| walker |  | 4382 | 43 | python method at src/requests/sessions.py:714 |  |  | 0.654 |
| walker |  | 4425 | 43 | python method at src/requests/sessions.py:728 |  |  | 0.654 |
| walker |  | 4470 | 45 | python method at src/requests/sessions.py:334 |  |  | 0.654 |
| ns | 4476 |  | 152 | `Response`: the remaining members (`iter_content` … `close`) | 3.5 | 3.4 | 0.645 |
| walker |  | 4520 | 50 | python method at src/requests/sessions.py:442 |  |  | 0.645 |
| walker |  | 4578 | 58 | python method at src/requests/sessions.py:655 |  |  | 0.645 |
| walker |  | 4612 | 34 | python method doc at src/requests/sessions.py:752 |  |  | 0.645 |
| walker |  | 4679 | 67 | python decl doc at src/requests/sessions.py:76 |  |  | 0.645 |
| ns | 4691 |  | 215 | `adapters.py`: pool defaults + the `BaseAdapter` contract | 3.6 |  | 0.630 |
| walker |  | 4718 | 39 | python method doc at src/requests/sessions.py:370 |  |  | 0.630 |
| walker |  | 4759 | 41 | python method doc at src/requests/sessions.py:888 |  |  | 0.630 |
| walker |  | 4831 | 72 | python method at src/requests/sessions.py:695 |  |  | 0.630 |
| walker |  | 4867 | 36 | python decl names surface in src/requests/help.py |  |  | 0.630 |
| walker |  | 4867 | 0 | python decl at src/requests/help.py:37 |  |  | 0.630 |
| walker |  | 4867 | 0 | python decl at src/requests/help.py:69 |  |  | 0.630 |
| walker |  | 4867 | 0 | python decl at src/requests/help.py:128 |  |  | 0.630 |
| walker |  | 4880 | 13 | python decl doc at src/requests/help.py:69 |  |  | 0.630 |
| walker |  | 4894 | 14 | python decl doc at src/requests/help.py:128 |  |  | 0.630 |
| walker |  | 4912 | 18 | python decl body at src/requests/help.py:128 body 130 |  |  | 0.630 |
| walker |  | 4957 | 45 | python method doc at src/requests/sessions.py:870 |  |  | 0.630 |
| walker |  | 5030 | 73 | python decl names surface in src/requests/utils.py |  |  | 0.630 |
| walker |  | 5030 | 0 | python decl at src/requests/utils.py:283 |  |  | 0.630 |
| walker |  | 5030 | 0 | python decl at src/requests/utils.py:290 |  |  | 0.630 |
| walker |  | 5051 | 21 | python decl at src/requests/utils.py:328 |  |  | 0.630 |
| ns | 5055 |  | 364 | `HTTPAdapter`: constructor knobs + complete method roster | 3.7 | 3.6 | 0.607 |
| walker |  | 5084 | 33 | python decl at src/requests/utils.py:231 |  |  | 0.607 |
| walker |  | 5101 | 17 | python decl doc at src/requests/utils.py:283 |  |  | 0.607 |
| walker |  | 5118 | 17 | python decl doc at src/requests/utils.py:328 |  |  | 0.607 |
| walker |  | 5155 | 37 | python decl at src/requests/utils.py:149 |  |  | 0.607 |
| walker |  | 5168 | 13 | python decl doc at src/requests/utils.py:149 |  |  | 0.607 |
| walker |  | 5187 | 19 | python decl doc at src/requests/utils.py:231 |  |  | 0.607 |
| walker |  | 5246 | 59 | python decl doc at src/requests/utils.py:290 |  |  | 0.607 |
| ns | 5324 |  | 269 | `SessionRedirectMixin` + session-module merge helpers | 3.8 |  | 0.611 |
| walker |  | 5327 | 81 | python method at src/requests/sessions.py:831 |  |  | 0.611 |
| walker |  | 5369 | 42 | python method doc at src/requests/sessions.py:831 |  |  | 0.611 |
| walker |  | 5458 | 89 | python decl names surface in src/requests/adapters.py |  |  | 0.613 |
| walker |  | 5458 | 0 | python decl at src/requests/adapters.py:122 |  |  | 0.613 |
| walker |  | 5458 | 0 | python decl at src/requests/adapters.py:158 |  |  | 0.613 |
| walker |  | 5469 | 11 | python decl doc at src/requests/adapters.py:122 |  |  | 0.614 |
| walker |  | 5484 | 15 | python decl doc at src/requests/adapters.py:158 |  |  | 0.614 |
| ns | 5560 |  | 236 | `cookies.py` module map: classes + free functions | 3.9 |  | 0.604 |
| walker |  | 5638 | 154 | python class body at src/requests/adapters.py:158 |  |  | 0.604 |
| ns | 5868 |  | 308 | `auth.py`: every auth handler and its methods | 3.10 |  | 0.590 |
| walker |  | 5923 | 285 | python method sigs in src/requests/adapters.py |  |  | 0.603 |
| walker |  | 5923 | 0 | python method at src/requests/adapters.py:125 |  |  | 0.603 |
| walker |  | 5923 | 0 | python method at src/requests/adapters.py:153 |  |  | 0.603 |
| walker |  | 5923 | 0 | python method at src/requests/adapters.py:223 |  |  | 0.603 |
| walker |  | 5923 | 0 | python method at src/requests/adapters.py:269 |  |  | 0.603 |
| walker |  | 5923 | 0 | python method at src/requests/adapters.py:365 |  |  | 0.603 |
| walker |  | 5923 | 0 | python method at src/requests/adapters.py:555 |  |  | 0.603 |
| walker |  | 5923 | 0 | python method at src/requests/adapters.py:599 |  |  | 0.603 |
| walker |  | 5923 | 0 | python method at src/requests/adapters.py:613 |  |  | 0.603 |
| walker |  | 5936 | 13 | python method doc at src/requests/adapters.py:153 |  |  | 0.605 |
| walker |  | 5967 | 31 | python method at src/requests/adapters.py:565 |  |  | 0.605 |
| walker |  | 6001 | 34 | python method at src/requests/adapters.py:512 |  |  | 0.605 |
| walker |  | 6040 | 39 | python method at src/requests/adapters.py:307 |  |  | 0.605 |
| walker |  | 6080 | 40 | python method at src/requests/adapters.py:226 |  |  | 0.605 |
| walker |  | 6128 | 48 | python method at src/requests/adapters.py:403 |  |  | 0.605 |
| walker |  | 6187 | 59 | python method at src/requests/adapters.py:239 |  |  | 0.605 |
| ns | 6240 |  | 372 | `_types.py`: public type-alias roster | 4.1 |  | 0.589 |
| walker |  | 6259 | 72 | python method at src/requests/adapters.py:455 |  |  | 0.589 |
| walker |  | 6335 | 76 | python method at src/requests/adapters.py:201 |  |  | 0.599 |
| walker |  | 6388 | 53 | python method doc at src/requests/adapters.py:555 |  |  | 0.599 |
| walker |  | 6498 | 110 | declaration surface of tox.ini |  |  | 0.599 |
| ns | 6551 |  | 311 | `_types.py`: the `Unpack` kwargs TypedDicts | 4.2 | 4.1 | 0.584 |
| walker |  | 6655 | 157 | python decl names surface in src/requests/compat.py |  |  | 0.584 |
| walker |  | 6655 | 0 | python decl at src/requests/compat.py:37 |  |  | 0.584 |
| walker |  | 6667 | 12 | python decl doc at src/requests/compat.py:37 |  |  | 0.584 |
| walker |  | 6723 | 56 | python method doc at src/requests/sessions.py:309 |  |  | 0.584 |
| walker |  | 6729 | 6 | listing of 'tests/certs/valid' |  |  | 0.584 |
| ns | 6750 |  | 199 | Environment-driven settings: the `merge_environment_settings` core | 4.3 | 2.6 | 0.576 |
| walker |  | 6826 | 97 | python method at src/requests/adapters.py:128 |  |  | 0.591 |
| ns | 6857 |  | 107 | `utils.py` module-level constants | 5.1 |  | 0.588 |
| walker |  | 6923 | 97 | python method at src/requests/adapters.py:634 |  |  | 0.604 |
| walker |  | 6948 | 25 | python imports in tests/__init__.py |  |  | 0.604 |
| ns | 7005 |  | 148 | `utils.py` function roster 1/3: proxies shim, lengths, netrc, encoders | 5.2 |  | 0.599 |
| ns | 7172 |  | 167 | `utils.py` function roster 2/3: cookies, encoding detection, URI quoting, CIDR | 5.3 | 5.2 | 0.591 |
| walker |  | 7204 | 256 | README.md section #3 |  |  | 0.591 |
| walker |  | 7254 | 50 | python decl names surface in src/requests/status_codes.py |  |  | 0.591 |
| walker |  | 7254 | 0 | python decl at src/requests/status_codes.py:109 |  |  | 0.591 |
| ns | 7352 |  | 180 | `utils.py` function roster 3/3: proxy resolution, default headers, header validation | 5.4 | 5.3 | 0.584 |
| walker |  | 7400 | 146 | python decl names surface in src/requests/models.py |  |  | 0.588 |
| walker |  | 7400 | 0 | python decl at src/requests/models.py:109 |  |  | 0.588 |
| walker |  | 7400 | 0 | python decl at src/requests/models.py:255 |  |  | 0.588 |
| walker |  | 7400 | 0 | python decl at src/requests/models.py:283 |  |  | 0.588 |
| walker |  | 7400 | 0 | python decl at src/requests/models.py:376 |  |  | 0.588 |
| walker |  | 7400 | 0 | python decl at src/requests/models.py:730 |  |  | 0.588 |
| walker |  | 7418 | 18 | python decl doc at src/requests/models.py:283 |  |  | 0.588 |
| walker |  | 7457 | 39 | python decl doc at src/requests/models.py:730 |  |  | 0.588 |
| walker |  | 7468 | 11 | python class body at src/requests/models.py:109 |  |  | 0.588 |
| ns | 7613 |  | 261 | `structures.py`: `CaseInsensitiveDict` and `LookupDict` in full shape | 5.5 |  | 0.578 |
| ns | 7723 |  | 110 | `status_codes.py`: how `requests.codes` is built | 5.6 |  | 0.575 |
| walker |  | 7727 | 259 | python class body at src/requests/models.py:730 |  |  | 0.607 |
| walker |  | 7850 | 123 | python class body at src/requests/models.py:283 |  |  | 0.618 |
| walker |  | 7949 | 99 | python class body at src/requests/models.py:376 |  |  | 0.620 |
| ns | 7965 |  | 242 | The five small modules: hooks, `_internal_utils`, compat, certs, packages | 5.7 |  | 0.612 |
| walker |  | 8023 | 74 | python decl at src/requests/models.py:96 |  |  | 0.621 |
| walker |  | 8040 | 17 | python class body at src/requests/models.py:255 |  |  | 0.621 |
| ns | 8142 |  | 177 | `help.py`: the bug-report payload | 5.8 |  | 0.614 |
| ns | 8475 |  | 333 | `resolve_redirects`: the rules the redirect loop enforces | 6.1 | 3.8 | 0.602 |
| walker |  | 8741 | 701 | python method sigs in src/requests/models.py |  |  | 0.640 |
| walker |  | 8741 | 0 | python method at src/requests/models.py:271 |  |  | 0.640 |
| walker |  | 8741 | 0 | python method at src/requests/models.py:355 |  |  | 0.640 |
| walker |  | 8741 | 0 | python method at src/requests/models.py:358 |  |  | 0.640 |
| walker |  | 8741 | 0 | python method at src/requests/models.py:451 |  |  | 0.640 |
| walker |  | 8741 | 0 | python method at src/requests/models.py:454 |  |  | 0.640 |
| walker |  | 8741 | 0 | python method at src/requests/models.py:465 |  |  | 0.640 |
| walker |  | 8741 | 0 | python method at src/requests/models.py:563 |  |  | 0.640 |
| walker |  | 8741 | 0 | python method at src/requests/models.py:652 |  |  | 0.640 |
| walker |  | 8741 | 0 | python method at src/requests/models.py:720 |  |  | 0.640 |
| walker |  | 8741 | 0 | python method at src/requests/models.py:763 |  |  | 0.640 |
| walker |  | 8741 | 0 | python method at src/requests/models.py:810 |  |  | 0.640 |
| walker |  | 8741 | 0 | python method at src/requests/models.py:813 |  |  | 0.640 |
| walker |  | 8741 | 0 | python method at src/requests/models.py:824 |  |  | 0.640 |
| walker |  | 8741 | 0 | python method at src/requests/models.py:832 |  |  | 0.640 |
| walker |  | 8741 | 0 | python method at src/requests/models.py:835 |  |  | 0.640 |
| walker |  | 8741 | 0 | python method at src/requests/models.py:845 |  |  | 0.640 |
| walker |  | 8741 | 0 | python method at src/requests/models.py:855 |  |  | 0.640 |
| walker |  | 8741 | 0 | python method at src/requests/models.py:1087 |  |  | 0.640 |
| walker |  | 8741 | 0 | python method at src/requests/models.py:1140 |  |  | 0.640 |
| walker |  | 8741 | 0 | python method at src/requests/models.py:1169 |  |  | 0.640 |
| walker |  | 8747 | 6 | python method at src/requests/models.py:112 |  |  | 0.640 |
| walker |  | 8755 | 8 | python method at src/requests/models.py:859 |  |  | 0.641 |
| walker |  | 8763 | 8 | python method at src/requests/models.py:874 |  |  | 0.642 |
| ns | 8764 |  | 289 | `HTTPAdapter.send`: the urllib3 call and the `MaxRetryError` fan-out | 6.2 | 3.7 | 0.630 |
| walker |  | 8771 | 8 | python method at src/requests/models.py:881 |  |  | 0.632 |
| walker |  | 8779 | 8 | python method at src/requests/models.py:889 |  |  | 0.633 |
| walker |  | 8787 | 8 | python method at src/requests/models.py:894 |  |  | 0.634 |
| walker |  | 8796 | 9 | python method at src/requests/models.py:1030 |  |  | 0.635 |
| walker |  | 8805 | 9 | python method at src/requests/models.py:1049 |  |  | 0.637 |
| walker |  | 8814 | 9 | python method at src/requests/models.py:1122 |  |  | 0.638 |
| walker |  | 8828 | 14 | python method at src/requests/models.py:405 |  |  | 0.638 |
| walker |  | 8836 | 8 | python method at src/requests/models.py:471 |  |  | 0.639 |
| walker |  | 8848 | 12 | python method doc at src/requests/models.py:720 |  |  | 0.639 |
| walker |  | 8861 | 13 | python method doc at src/requests/models.py:112 |  |  | 0.639 |
| walker |  | 8874 | 13 | python method doc at src/requests/models.py:465 |  |  | 0.639 |
| walker |  | 8887 | 13 | python method doc at src/requests/models.py:563 |  |  | 0.639 |
| walker |  | 8916 | 29 | python method at src/requests/models.py:816 |  |  | 0.639 |
| walker |  | 8931 | 15 | python method doc at src/requests/models.py:1030 |  |  | 0.639 |
| walker |  | 8964 | 33 | python method at src/requests/models.py:697 |  |  | 0.639 |
| ns | 8966 |  | 202 | `HTTPAdapter.send`: the remaining `except` arms and the return | 6.3 | 6.2 | 0.631 |
| walker |  | 8980 | 16 | python method doc at src/requests/models.py:855 |  |  | 0.631 |
| walker |  | 8997 | 17 | python method doc at src/requests/models.py:652 |  |  | 0.631 |
| walker |  | 9033 | 36 | python method at src/requests/models.py:258 |  |  | 0.631 |
| walker |  | 9045 | 12 | python method doc at src/requests/models.py:258 |  |  | 0.631 |
| ns | 9054 |  | 88 | Makefile: the remaining targets (coverage, publish, docs) | 7.1 | 1.8 | 0.634 |
| walker |  | 9081 | 36 | python method at src/requests/models.py:912 |  |  | 0.634 |
| walker |  | 9099 | 18 | python method doc at src/requests/models.py:881 |  |  | 0.634 |
| walker |  | 9117 | 18 | python method doc at src/requests/models.py:1140 |  |  | 0.634 |
| walker |  | 9136 | 19 | python method doc at src/requests/models.py:1122 |  |  | 0.634 |
| walker |  | 9175 | 39 | python method at src/requests/models.py:574 |  |  | 0.634 |
| walker |  | 9189 | 14 | python method doc at src/requests/models.py:574 |  |  | 0.634 |
| walker |  | 9229 | 40 | python method at src/requests/models.py:481 |  |  | 0.634 |
| walker |  | 9242 | 13 | python method doc at src/requests/models.py:481 |  |  | 0.634 |
| walker |  | 9283 | 41 | python method at src/requests/models.py:668 |  |  | 0.634 |
| walker |  | 9297 | 14 | python method doc at src/requests/models.py:668 |  |  | 0.634 |
| walker |  | 9319 | 22 | python method doc at src/requests/models.py:894 |  |  | 0.634 |
| ns | 9337 |  | 283 | pyproject: every remaining table, and the settings that change how you work here | 7.2 | 1.10 | 0.626 |
| walker |  | 9343 | 24 | python method doc at src/requests/models.py:358 |  |  | 0.626 |
| walker |  | 9367 | 24 | python method doc at src/requests/models.py:889 |  |  | 0.626 |
| walker |  | 9423 | 56 | python method at src/requests/models.py:908 |  |  | 0.626 |
| walker |  | 9423 | 0 | python method body at src/requests/models.py:908 body 911 |  |  | 0.626 |
| walker |  | 9480 | 57 | python method at src/requests/models.py:904 |  |  | 0.626 |
| walker |  | 9480 | 0 | python method body at src/requests/models.py:904 body 907 |  |  | 0.626 |
| ns | 9492 |  | 155 | `tests/conftest.py`: the fixture set every test builds on | 7.3 |  | 0.621 |
| walker |  | 9541 | 61 | python method at src/requests/models.py:990 |  |  | 0.621 |
| walker |  | 9576 | 35 | python method doc at src/requests/models.py:271 |  |  | 0.621 |
| ns | 9587 |  | 95 | `tests/test_requests.py`: top-level class roster | 7.4 |  | 0.617 |
| walker |  | 9622 | 46 | python method doc at src/requests/models.py:874 |  |  | 0.617 |
| walker |  | 9704 | 82 | python method at src/requests/models.py:975 |  |  | 0.617 |
| walker |  | 9704 | 0 | python method body at src/requests/models.py:975 body 981 |  |  | 0.617 |
| ns | 9769 |  | 182 | `tests/testserver/server.py`: the local socket server API (complete) | 7.5 |  | 0.611 |
| walker |  | 9788 | 84 | python method at src/requests/models.py:982 |  |  | 0.611 |
| walker |  | 9788 | 0 | python method body at src/requests/models.py:982 body 989 |  |  | 0.611 |
| walker |  | 9824 | 36 | python method at src/requests/models.py:133 |  |  | 0.611 |
| walker |  | 9824 | 0 | python method body at src/requests/models.py:133 body 135 |  |  | 0.611 |
| walker |  | 9858 | 34 | python method at src/requests/models.py:137 |  |  | 0.611 |
| walker |  | 9858 | 0 | python method body at src/requests/models.py:137 body 139 |  |  | 0.611 |
| walker |  | 9899 | 41 | python method at src/requests/models.py:147 |  |  | 0.611 |
| walker |  | 9899 | 0 | python method body at src/requests/models.py:147 body 149 |  |  | 0.611 |
| ns | 9926 |  | 157 | `docs/api.rst`: every section of the developer interface | 7.6 |  | 0.605 |
| walker |  | 9940 | 41 | python method at src/requests/models.py:151 |  |  | 0.605 |
| ns | 9974 |  | 48 | `HISTORY.md`: the changelog's head | 7.7 |  | 0.603 |
| walker |  | 9983 | 43 | python method at src/requests/models.py:183 |  |  | 0.603 |
