Score(3000)=0.664 I=0.784 C=0.562 ns_rows≤3K=17/49 (reached=8 partial=0 missing=9)

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
| walker |  | 2459 | 109 | tool.pytest+pyright config in pyproject.toml |  |  | 0.605 |
| walker |  | 2584 | 125 | package metadata in pyproject.toml |  |  | 0.644 |
| ns | 2631 |  | 288 | `Session` method roster (complete, with line coordinates) | 2.6 | 2.4 | 0.619 |
| walker |  | 2662 | 78 | listing of 'tests' |  |  | 0.654 |
| walker |  | 2671 | 9 | listing of 'tests/testserver' |  |  | 0.664 |
| walker |  | 2687 | 16 | listing of 'tests/certs' |  |  | 0.687 |
| ns | 2882 |  | 251 | `Session.request`: the complete keyword signature | 2.7 | 2.6 | 0.664 |
| walker |  | 3043 | 356 | plaintext config Makefile |  |  | 0.684 |
| ns | 3065 |  | 183 | `Session.__init__`: every default value | 2.8 | 2.6 | 0.667 |
| walker |  | 3183 | 140 | dev/build/target dependencies in pyproject.toml |  |  | 0.668 |
| walker |  | 3222 | 39 | plaintext config docs/requirements.txt |  |  | 0.668 |
| ns | 3418 |  | 353 | `models.py` module map: constants, all five classes, `Request` fields | 3.1 |  | 0.639 |
| walker |  | 3478 | 256 | README.md section #3 |  |  | 0.639 |
| ns | 3739 |  | 321 | `PreparedRequest`: attributes + every `prepare_*` step | 3.2 | 3.1 | 0.618 |
| walker |  | 3792 | 314 | tool.ruff config in pyproject.toml |  |  | 0.619 |
| walker |  | 3818 | 26 | python imports in src/requests/packages.py |  |  | 0.619 |
| walker |  | 3994 | 176 | README.md section #2 |  |  | 0.619 |
| ns | 4042 |  | 303 | `Response` attribute set: typed fields + `__attrs__` | 3.3 | 3.1 | 0.591 |
| walker |  | 4067 | 73 | declaration surface of requirements-dev.txt |  |  | 0.591 |
| walker |  | 4073 | 6 | listing of 'tests/certs/valid' |  |  | 0.591 |
| walker |  | 4130 | 57 | python decl names surface in src/requests/sessions.py |  |  | 0.591 |
| walker |  | 4130 | 0 | python decl at src/requests/sessions.py:127 |  |  | 0.591 |
| walker |  | 4130 | 0 | python decl at src/requests/sessions.py:395 |  |  | 0.591 |
| walker |  | 4130 | 0 | python decl at src/requests/sessions.py:908 |  |  | 0.591 |
| walker |  | 4138 | 8 | python decl body at src/requests/sessions.py:908 body 920 |  |  | 0.591 |
| walker |  | 4169 | 31 | python decl at src/requests/sessions.py:76 |  |  | 0.591 |
| walker |  | 4222 | 53 | python decl at src/requests/sessions.py:108 |  |  | 0.591 |
| walker |  | 4254 | 32 | python class body at src/requests/sessions.py:127 |  |  | 0.591 |
| ns | 4324 |  | 282 | `Response`: `__init__`, dunders and every `@property` | 3.4 | 3.3 | 0.576 |
| ns | 4476 |  | 152 | `Response`: the remaining members (`iter_content` … `close`) | 3.5 | 3.4 | 0.567 |
| walker |  | 4527 | 273 | python class body at src/requests/sessions.py:395 |  |  | 0.614 |
| walker |  | 4584 | 57 | python decl doc at src/requests/sessions.py:108 |  |  | 0.614 |
| ns | 4691 |  | 215 | `adapters.py`: pool defaults + the `BaseAdapter` contract | 3.6 |  | 0.600 |
| ns | 5055 |  | 364 | `HTTPAdapter`: constructor knobs + complete method roster | 3.7 | 3.6 | 0.578 |
| walker |  | 5058 | 474 | python method sigs in src/requests/sessions.py |  |  | 0.608 |
| walker |  | 5058 | 0 | python method at src/requests/sessions.py:132 |  |  | 0.608 |
| walker |  | 5058 | 0 | python method at src/requests/sessions.py:134 |  |  | 0.608 |
| walker |  | 5058 | 0 | python method at src/requests/sessions.py:154 |  |  | 0.608 |
| walker |  | 5058 | 0 | python method at src/requests/sessions.py:505 |  |  | 0.608 |
| walker |  | 5058 | 0 | python method at src/requests/sessions.py:508 |  |  | 0.608 |
| walker |  | 5058 | 0 | python method at src/requests/sessions.py:511 |  |  | 0.608 |
| walker |  | 5058 | 0 | python method at src/requests/sessions.py:673 |  |  | 0.608 |
| walker |  | 5058 | 0 | python method at src/requests/sessions.py:684 |  |  | 0.608 |
| walker |  | 5058 | 0 | python method at src/requests/sessions.py:742 |  |  | 0.608 |
| walker |  | 5058 | 0 | python method at src/requests/sessions.py:752 |  |  | 0.608 |
| walker |  | 5058 | 0 | python method at src/requests/sessions.py:870 |  |  | 0.608 |
| walker |  | 5058 | 0 | python method at src/requests/sessions.py:883 |  |  | 0.608 |
| walker |  | 5058 | 0 | python method at src/requests/sessions.py:888 |  |  | 0.608 |
| walker |  | 5058 | 0 | python method at src/requests/sessions.py:899 |  |  | 0.608 |
| walker |  | 5058 | 0 | python method at src/requests/sessions.py:903 |  |  | 0.608 |
| walker |  | 5058 | 0 | python method body at src/requests/sessions.py:132 body 132 |  |  | 0.608 |
| walker |  | 5084 | 26 | python method at src/requests/sessions.py:309 |  |  | 0.608 |
| walker |  | 5110 | 26 | python method at src/requests/sessions.py:370 |  |  | 0.608 |
| walker |  | 5126 | 16 | python method doc at src/requests/sessions.py:883 |  |  | 0.608 |
| walker |  | 5144 | 18 | python method doc at src/requests/sessions.py:154 |  |  | 0.608 |
| walker |  | 5164 | 20 | python method doc at src/requests/sessions.py:134 |  |  | 0.609 |
| walker |  | 5207 | 43 | python method at src/requests/sessions.py:714 |  |  | 0.609 |
| walker |  | 5250 | 43 | python method at src/requests/sessions.py:728 |  |  | 0.609 |
| walker |  | 5295 | 45 | python method at src/requests/sessions.py:334 |  |  | 0.609 |
| ns | 5324 |  | 269 | `SessionRedirectMixin` + session-module merge helpers | 3.8 |  | 0.613 |
| walker |  | 5345 | 50 | python method at src/requests/sessions.py:442 |  |  | 0.613 |
| walker |  | 5403 | 58 | python method at src/requests/sessions.py:655 |  |  | 0.613 |
| walker |  | 5437 | 34 | python method doc at src/requests/sessions.py:752 |  |  | 0.613 |
| walker |  | 5504 | 67 | python decl doc at src/requests/sessions.py:76 |  |  | 0.613 |
| walker |  | 5543 | 39 | python method doc at src/requests/sessions.py:370 |  |  | 0.613 |
| ns | 5560 |  | 236 | `cookies.py` module map: classes + free functions | 3.9 |  | 0.603 |
| walker |  | 5579 | 36 | python decl names surface in src/requests/help.py |  |  | 0.603 |
| walker |  | 5579 | 0 | python decl at src/requests/help.py:37 |  |  | 0.603 |
| walker |  | 5579 | 0 | python decl at src/requests/help.py:69 |  |  | 0.603 |
| walker |  | 5579 | 0 | python decl at src/requests/help.py:128 |  |  | 0.603 |
| walker |  | 5592 | 13 | python decl doc at src/requests/help.py:69 |  |  | 0.603 |
| walker |  | 5606 | 14 | python decl doc at src/requests/help.py:128 |  |  | 0.603 |
| walker |  | 5624 | 18 | python decl body at src/requests/help.py:128 body 130 |  |  | 0.603 |
| walker |  | 5665 | 41 | python method doc at src/requests/sessions.py:888 |  |  | 0.603 |
| walker |  | 5737 | 72 | python method at src/requests/sessions.py:695 |  |  | 0.603 |
| walker |  | 5826 | 89 | python decl names surface in src/requests/adapters.py |  |  | 0.605 |
| walker |  | 5826 | 0 | python decl at src/requests/adapters.py:122 |  |  | 0.605 |
| walker |  | 5826 | 0 | python decl at src/requests/adapters.py:158 |  |  | 0.605 |
| walker |  | 5837 | 11 | python decl doc at src/requests/adapters.py:122 |  |  | 0.606 |
| walker |  | 5852 | 15 | python decl doc at src/requests/adapters.py:158 |  |  | 0.606 |
| ns | 5868 |  | 308 | `auth.py`: every auth handler and its methods | 3.10 |  | 0.592 |
| walker |  | 6006 | 154 | python class body at src/requests/adapters.py:158 |  |  | 0.592 |
| ns | 6240 |  | 372 | `_types.py`: public type-alias roster | 4.1 |  | 0.576 |
| walker |  | 6291 | 285 | python method sigs in src/requests/adapters.py |  |  | 0.588 |
| walker |  | 6291 | 0 | python method at src/requests/adapters.py:125 |  |  | 0.588 |
| walker |  | 6291 | 0 | python method at src/requests/adapters.py:153 |  |  | 0.588 |
| walker |  | 6291 | 0 | python method at src/requests/adapters.py:223 |  |  | 0.588 |
| walker |  | 6291 | 0 | python method at src/requests/adapters.py:269 |  |  | 0.588 |
| walker |  | 6291 | 0 | python method at src/requests/adapters.py:365 |  |  | 0.588 |
| walker |  | 6291 | 0 | python method at src/requests/adapters.py:555 |  |  | 0.588 |
| walker |  | 6291 | 0 | python method at src/requests/adapters.py:599 |  |  | 0.588 |
| walker |  | 6291 | 0 | python method at src/requests/adapters.py:613 |  |  | 0.588 |
| walker |  | 6304 | 13 | python method doc at src/requests/adapters.py:153 |  |  | 0.590 |
| walker |  | 6335 | 31 | python method at src/requests/adapters.py:565 |  |  | 0.590 |
| walker |  | 6369 | 34 | python method at src/requests/adapters.py:512 |  |  | 0.590 |
| walker |  | 6408 | 39 | python method at src/requests/adapters.py:307 |  |  | 0.590 |
| walker |  | 6448 | 40 | python method at src/requests/adapters.py:226 |  |  | 0.590 |
| walker |  | 6496 | 48 | python method at src/requests/adapters.py:403 |  |  | 0.590 |
| ns | 6551 |  | 311 | `_types.py`: the `Unpack` kwargs TypedDicts | 4.2 | 4.1 | 0.576 |
| walker |  | 6555 | 59 | python method at src/requests/adapters.py:239 |  |  | 0.576 |
| walker |  | 6627 | 72 | python method at src/requests/adapters.py:455 |  |  | 0.576 |
| walker |  | 6672 | 45 | python method doc at src/requests/sessions.py:870 |  |  | 0.576 |
| walker |  | 6748 | 76 | python method at src/requests/adapters.py:201 |  |  | 0.586 |
| ns | 6750 |  | 199 | Environment-driven settings: the `merge_environment_settings` core | 4.3 | 2.6 | 0.577 |
| ns | 6857 |  | 107 | `utils.py` module-level constants | 5.1 |  | 0.574 |
| walker |  | 6905 | 157 | python decl names surface in src/requests/compat.py |  |  | 0.574 |
| walker |  | 6905 | 0 | python decl at src/requests/compat.py:37 |  |  | 0.574 |
| walker |  | 6917 | 12 | python decl doc at src/requests/compat.py:37 |  |  | 0.574 |
| walker |  | 6998 | 81 | python method at src/requests/sessions.py:831 |  |  | 0.574 |
| ns | 7005 |  | 148 | `utils.py` function roster 1/3: proxies shim, lengths, netrc, encoders | 5.2 |  | 0.567 |
| walker |  | 7040 | 42 | python method doc at src/requests/sessions.py:831 |  |  | 0.567 |
| walker |  | 7093 | 53 | python method doc at src/requests/adapters.py:555 |  |  | 0.567 |
| ns | 7172 |  | 167 | `utils.py` function roster 2/3: cookies, encoding detection, URI quoting, CIDR | 5.3 | 5.2 | 0.560 |
| walker |  | 7203 | 110 | declaration surface of tox.ini |  |  | 0.560 |
| walker |  | 7259 | 56 | python method doc at src/requests/sessions.py:309 |  |  | 0.560 |
| walker |  | 7309 | 50 | python decl names surface in src/requests/status_codes.py |  |  | 0.560 |
| walker |  | 7309 | 0 | python decl at src/requests/status_codes.py:109 |  |  | 0.560 |
| ns | 7352 |  | 180 | `utils.py` function roster 3/3: proxy resolution, default headers, header validation | 5.4 | 5.3 | 0.552 |
| ns | 7613 |  | 261 | `structures.py`: `CaseInsensitiveDict` and `LookupDict` in full shape | 5.5 |  | 0.543 |
| walker |  | 7662 | 353 | python decl names surface in src/requests/exceptions.py |  |  | 0.574 |
| walker |  | 7662 | 0 | python decl at src/requests/exceptions.py:20 |  |  | 0.574 |
| walker |  | 7662 | 0 | python decl at src/requests/exceptions.py:38 |  |  | 0.574 |
| walker |  | 7662 | 0 | python decl at src/requests/exceptions.py:42 |  |  | 0.574 |
| walker |  | 7662 | 0 | python decl at src/requests/exceptions.py:66 |  |  | 0.574 |
| walker |  | 7662 | 0 | python decl at src/requests/exceptions.py:70 |  |  | 0.574 |
| walker |  | 7662 | 0 | python decl at src/requests/exceptions.py:74 |  |  | 0.574 |
| walker |  | 7662 | 0 | python decl at src/requests/exceptions.py:78 |  |  | 0.574 |
| walker |  | 7662 | 0 | python decl at src/requests/exceptions.py:82 |  |  | 0.574 |
| walker |  | 7662 | 0 | python decl at src/requests/exceptions.py:91 |  |  | 0.574 |
| walker |  | 7662 | 0 | python decl at src/requests/exceptions.py:98 |  |  | 0.574 |
| walker |  | 7662 | 0 | python decl at src/requests/exceptions.py:102 |  |  | 0.574 |
| walker |  | 7662 | 0 | python decl at src/requests/exceptions.py:106 |  |  | 0.574 |
| walker |  | 7662 | 0 | python decl at src/requests/exceptions.py:110 |  |  | 0.574 |
| walker |  | 7662 | 0 | python decl at src/requests/exceptions.py:114 |  |  | 0.574 |
| walker |  | 7662 | 0 | python decl at src/requests/exceptions.py:118 |  |  | 0.574 |
| walker |  | 7662 | 0 | python decl at src/requests/exceptions.py:122 |  |  | 0.574 |
| walker |  | 7662 | 0 | python decl at src/requests/exceptions.py:126 |  |  | 0.574 |
| walker |  | 7662 | 0 | python decl at src/requests/exceptions.py:130 |  |  | 0.574 |
| walker |  | 7662 | 0 | python decl at src/requests/exceptions.py:134 |  |  | 0.574 |
| walker |  | 7662 | 0 | python decl at src/requests/exceptions.py:138 |  |  | 0.574 |
| walker |  | 7662 | 0 | python decl at src/requests/exceptions.py:142 |  |  | 0.574 |
| walker |  | 7662 | 0 | python decl at src/requests/exceptions.py:146 |  |  | 0.574 |
| walker |  | 7662 | 0 | python decl at src/requests/exceptions.py:153 |  |  | 0.574 |
| walker |  | 7662 | 0 | python decl at src/requests/exceptions.py:157 |  |  | 0.574 |
| walker |  | 7662 | 0 | python decl at src/requests/exceptions.py:161 |  |  | 0.574 |
| walker |  | 7670 | 8 | python decl doc at src/requests/exceptions.py:106 |  |  | 0.574 |
| walker |  | 7679 | 9 | python decl doc at src/requests/exceptions.py:38 |  |  | 0.574 |
| walker |  | 7688 | 9 | python decl doc at src/requests/exceptions.py:66 |  |  | 0.574 |
| walker |  | 7697 | 9 | python decl doc at src/requests/exceptions.py:70 |  |  | 0.574 |
| walker |  | 7706 | 9 | python decl doc at src/requests/exceptions.py:74 |  |  | 0.574 |
| walker |  | 7715 | 9 | python decl doc at src/requests/exceptions.py:78 |  |  | 0.574 |
| ns | 7723 |  | 110 | `status_codes.py`: how `requests.codes` is built | 5.6 |  | 0.571 |
| walker |  | 7724 | 9 | python decl doc at src/requests/exceptions.py:142 |  |  | 0.571 |
| walker |  | 7733 | 9 | python decl doc at src/requests/exceptions.py:153 |  |  | 0.571 |
| walker |  | 7743 | 10 | python decl doc at src/requests/exceptions.py:134 |  |  | 0.571 |
| walker |  | 7754 | 11 | python decl doc at src/requests/exceptions.py:118 |  |  | 0.571 |
| walker |  | 7765 | 11 | python decl doc at src/requests/exceptions.py:126 |  |  | 0.571 |
| walker |  | 7777 | 12 | python decl doc at src/requests/exceptions.py:122 |  |  | 0.571 |
| walker |  | 7790 | 13 | python decl doc at src/requests/exceptions.py:42 |  |  | 0.571 |
| walker |  | 7803 | 13 | python decl doc at src/requests/exceptions.py:138 |  |  | 0.571 |
| walker |  | 7817 | 14 | python decl doc at src/requests/exceptions.py:102 |  |  | 0.571 |
| walker |  | 7831 | 14 | python decl doc at src/requests/exceptions.py:114 |  |  | 0.571 |
| walker |  | 7845 | 14 | python decl doc at src/requests/exceptions.py:161 |  |  | 0.571 |
| walker |  | 7861 | 16 | python decl doc at src/requests/exceptions.py:130 |  |  | 0.571 |
| walker |  | 7878 | 17 | python decl doc at src/requests/exceptions.py:146 |  |  | 0.571 |
| walker |  | 7896 | 18 | python decl doc at src/requests/exceptions.py:98 |  |  | 0.571 |
| walker |  | 7914 | 18 | python decl doc at src/requests/exceptions.py:110 |  |  | 0.571 |
| walker |  | 7933 | 19 | python decl doc at src/requests/exceptions.py:157 |  |  | 0.571 |
| walker |  | 7963 | 30 | python decl doc at src/requests/exceptions.py:20 |  |  | 0.571 |
| ns | 7965 |  | 242 | The five small modules: hooks, `_internal_utils`, compat, certs, packages | 5.7 |  | 0.563 |
| walker |  | 8006 | 43 | python decl doc at src/requests/exceptions.py:91 |  |  | 0.563 |
| walker |  | 8031 | 25 | python class body at src/requests/exceptions.py:20 |  |  | 0.563 |
| walker |  | 8096 | 65 | python decl doc at src/requests/exceptions.py:82 |  |  | 0.563 |
| ns | 8142 |  | 177 | `help.py`: the bug-report payload | 5.8 |  | 0.557 |
| walker |  | 8166 | 70 | python method sigs in src/requests/exceptions.py |  |  | 0.557 |
| walker |  | 8166 | 0 | python method at src/requests/exceptions.py:28 |  |  | 0.557 |
| walker |  | 8166 | 0 | python method at src/requests/exceptions.py:45 |  |  | 0.557 |
| walker |  | 8166 | 0 | python method at src/requests/exceptions.py:55 |  |  | 0.557 |
| walker |  | 8185 | 19 | python method doc at src/requests/exceptions.py:28 |  |  | 0.557 |
| walker |  | 8331 | 146 | python decl names surface in src/requests/models.py |  |  | 0.561 |
| walker |  | 8331 | 0 | python decl at src/requests/models.py:109 |  |  | 0.561 |
| walker |  | 8331 | 0 | python decl at src/requests/models.py:255 |  |  | 0.561 |
| walker |  | 8331 | 0 | python decl at src/requests/models.py:283 |  |  | 0.561 |
| walker |  | 8331 | 0 | python decl at src/requests/models.py:376 |  |  | 0.561 |
| walker |  | 8331 | 0 | python decl at src/requests/models.py:730 |  |  | 0.561 |
| walker |  | 8349 | 18 | python decl doc at src/requests/models.py:283 |  |  | 0.561 |
| walker |  | 8388 | 39 | python decl doc at src/requests/models.py:730 |  |  | 0.562 |
| walker |  | 8399 | 11 | python class body at src/requests/models.py:109 |  |  | 0.562 |
| ns | 8475 |  | 333 | `resolve_redirects`: the rules the redirect loop enforces | 6.1 | 3.8 | 0.550 |
| walker |  | 8658 | 259 | python class body at src/requests/models.py:730 |  |  | 0.581 |
| ns | 8764 |  | 289 | `HTTPAdapter.send`: the urllib3 call and the `MaxRetryError` fan-out | 6.2 | 3.7 | 0.570 |
| walker |  | 8781 | 123 | python class body at src/requests/models.py:283 |  |  | 0.580 |
| walker |  | 8880 | 99 | python class body at src/requests/models.py:376 |  |  | 0.582 |
| walker |  | 8954 | 74 | python decl at src/requests/models.py:96 |  |  | 0.591 |
| ns | 8966 |  | 202 | `HTTPAdapter.send`: the remaining `except` arms and the return | 6.3 | 6.2 | 0.584 |
| walker |  | 8971 | 17 | python class body at src/requests/models.py:255 |  |  | 0.584 |
| ns | 9054 |  | 88 | Makefile: the remaining targets (coverage, publish, docs) | 7.1 | 1.8 | 0.588 |
| ns | 9337 |  | 283 | pyproject: every remaining table, and the settings that change how you work here | 7.2 | 1.10 | 0.599 |
| ns | 9492 |  | 155 | `tests/conftest.py`: the fixture set every test builds on | 7.3 |  | 0.594 |
| ns | 9587 |  | 95 | `tests/test_requests.py`: top-level class roster | 7.4 |  | 0.591 |
| walker |  | 9672 | 701 | python method sigs in src/requests/models.py |  |  | 0.625 |
| walker |  | 9672 | 0 | python method at src/requests/models.py:271 |  |  | 0.625 |
| walker |  | 9672 | 0 | python method at src/requests/models.py:355 |  |  | 0.625 |
| walker |  | 9672 | 0 | python method at src/requests/models.py:358 |  |  | 0.625 |
| walker |  | 9672 | 0 | python method at src/requests/models.py:451 |  |  | 0.625 |
| walker |  | 9672 | 0 | python method at src/requests/models.py:454 |  |  | 0.625 |
| walker |  | 9672 | 0 | python method at src/requests/models.py:465 |  |  | 0.625 |
| walker |  | 9672 | 0 | python method at src/requests/models.py:563 |  |  | 0.625 |
| walker |  | 9672 | 0 | python method at src/requests/models.py:652 |  |  | 0.625 |
| walker |  | 9672 | 0 | python method at src/requests/models.py:720 |  |  | 0.625 |
| walker |  | 9672 | 0 | python method at src/requests/models.py:763 |  |  | 0.625 |
| walker |  | 9672 | 0 | python method at src/requests/models.py:810 |  |  | 0.625 |
| walker |  | 9672 | 0 | python method at src/requests/models.py:813 |  |  | 0.625 |
| walker |  | 9672 | 0 | python method at src/requests/models.py:824 |  |  | 0.625 |
| walker |  | 9672 | 0 | python method at src/requests/models.py:832 |  |  | 0.625 |
| walker |  | 9672 | 0 | python method at src/requests/models.py:835 |  |  | 0.625 |
| walker |  | 9672 | 0 | python method at src/requests/models.py:845 |  |  | 0.625 |
| walker |  | 9672 | 0 | python method at src/requests/models.py:855 |  |  | 0.625 |
| walker |  | 9672 | 0 | python method at src/requests/models.py:1087 |  |  | 0.625 |
| walker |  | 9672 | 0 | python method at src/requests/models.py:1140 |  |  | 0.625 |
| walker |  | 9672 | 0 | python method at src/requests/models.py:1169 |  |  | 0.625 |
| walker |  | 9678 | 6 | python method at src/requests/models.py:112 |  |  | 0.625 |
| walker |  | 9686 | 8 | python method at src/requests/models.py:859 |  |  | 0.626 |
| walker |  | 9694 | 8 | python method at src/requests/models.py:874 |  |  | 0.628 |
| walker |  | 9702 | 8 | python method at src/requests/models.py:881 |  |  | 0.629 |
| walker |  | 9710 | 8 | python method at src/requests/models.py:889 |  |  | 0.630 |
| walker |  | 9718 | 8 | python method at src/requests/models.py:894 |  |  | 0.631 |
| walker |  | 9727 | 9 | python method at src/requests/models.py:1030 |  |  | 0.632 |
| walker |  | 9736 | 9 | python method at src/requests/models.py:1049 |  |  | 0.633 |
| walker |  | 9745 | 9 | python method at src/requests/models.py:1122 |  |  | 0.635 |
| walker |  | 9759 | 14 | python method at src/requests/models.py:405 |  |  | 0.635 |
| walker |  | 9767 | 8 | python method at src/requests/models.py:471 |  |  | 0.636 |
| ns | 9769 |  | 182 | `tests/testserver/server.py`: the local socket server API (complete) | 7.5 |  | 0.629 |
| walker |  | 9779 | 12 | python method doc at src/requests/models.py:720 |  |  | 0.629 |
| walker |  | 9792 | 13 | python method doc at src/requests/models.py:112 |  |  | 0.629 |
| walker |  | 9805 | 13 | python method doc at src/requests/models.py:465 |  |  | 0.629 |
| walker |  | 9818 | 13 | python method doc at src/requests/models.py:563 |  |  | 0.629 |
| walker |  | 9847 | 29 | python method at src/requests/models.py:816 |  |  | 0.629 |
| walker |  | 9862 | 15 | python method doc at src/requests/models.py:1030 |  |  | 0.629 |
| walker |  | 9895 | 33 | python method at src/requests/models.py:697 |  |  | 0.629 |
| walker |  | 9911 | 16 | python method doc at src/requests/models.py:855 |  |  | 0.629 |
| ns | 9926 |  | 157 | `docs/api.rst`: every section of the developer interface | 7.6 |  | 0.623 |
| walker |  | 9928 | 17 | python method doc at src/requests/models.py:652 |  |  | 0.623 |
| walker |  | 9964 | 36 | python method at src/requests/models.py:258 |  |  | 0.623 |
| ns | 9974 |  | 48 | `HISTORY.md`: the changelog's head | 7.7 |  | 0.621 |
| walker |  | 9976 | 12 | python method doc at src/requests/models.py:258 |  |  | 0.621 |
