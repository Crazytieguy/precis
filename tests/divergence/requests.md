Score(3000)=0.684 I=0.796 C=0.587 ns_rows≤3K=17/49 grid(1000/1442/2080/3000/4327/6240/9000)=0.522/0.473/0.597/0.684/0.621/0.599/0.631

| source | ns_cum | walker_cum | marginal | descriptor | id | predecessor | Score(B=cum) |
|:-------|-------:|-----------:|---------:|:-----------|:---|:------------|-------------:|
| ns | 47 |  | 47 | Identity lede: what the library is, and its version | 1.1 |  | 0.000 |
| walker |  | 74 | 74 | listing of '.' |  |  | 0.000 |
| walker |  | 77 | 3 | listing of 'src' |  |  | 0.000 |
| walker |  | 111 | 34 | listing of 'ext' |  |  | 0.000 |
| ns | 121 |  | 74 | Repository root listing (complete) | 1.2 |  | 0.671 |
| walker |  | 165 | 54 | listing of 'docs' |  |  | 0.680 |
| walker |  | 169 | 4 | listing of 'docs/_templates' |  |  | 0.680 |
| walker |  | 178 | 9 | listing of 'docs/_static' |  |  | 0.680 |
| walker |  | 188 | 10 | listing of 'docs/dev' |  |  | 0.683 |
| walker |  | 202 | 14 | listing of 'docs/_themes' |  |  | 0.683 |
| ns | 216 |  | 95 | `src/requests/` module roster (complete) | 1.3 |  | 0.463 |
| walker |  | 223 | 21 | listing of 'docs/user' |  |  | 0.467 |
| walker |  | 261 | 38 | listing of 'docs/community' |  |  | 0.478 |
| ns | 319 |  | 103 | `tests/` roster (complete, incl. `testserver/` and `certs/`) | 1.4 |  | 0.378 |
| ns | 466 |  | 147 | README: the canonical `requests.get(...)` doctest + every `##` heading | 1.5 | 1.1 | 0.337 |
| ns | 584 |  | 118 | Full package metadata block (`__version__.py`) | 1.6 | 1.1 | 0.316 |
| walker |  | 617 | 356 | plaintext config Makefile |  |  | 0.322 |
| ns | 682 |  | 98 | CI: `.github/` and workflow file roster (complete) | 1.7 |  | 0.286 |
| walker |  | 709 | 92 | listing of 'src/requests' |  |  | 0.456 |
| ns | 762 |  | 80 | Makefile: install / test / CI targets | 1.8 |  | 0.477 |
| ns | 885 |  | 123 | `docs/` tree listing (complete) | 1.9 |  | 0.522 |
| ns | 1187 |  | 302 | pyproject: build backend, metadata, runtime dependencies | 1.10 |  | 0.473 |
| ns | 1458 |  | 271 | `requests/__init__.py`: name → module re-export map | 2.1 |  | 0.431 |
| ns | 1532 |  | 74 | `api.py`: names of all eight module-level functions | 2.2 |  | 0.421 |
| walker |  | 1711 | 1002 | python imports in src/requests/__init__.py |  |  | 0.509 |
| walker |  | 1745 | 34 | python decl names surface in src/requests/__init__.py |  |  | 0.509 |
| walker |  | 1761 | 16 | python decl at src/requests/__init__.py:99 |  |  | 0.509 |
| ns | 1885 |  | 353 | `exceptions.py`: the complete class hierarchy | 2.3 |  | 0.476 |
| walker |  | 1936 | 175 | README headline in README.md |  |  | 0.588 |
| walker |  | 1976 | 40 | headings outline in README.md |  |  | 0.610 |
| walker |  | 2023 | 47 | python decl at src/requests/__init__.py:60 |  |  | 0.610 |
| ns | 2061 |  | 176 | `Session` class declaration + docstring | 2.4 |  | 0.584 |
| walker |  | 2066 | 43 | listing of '.github' |  |  | 0.597 |
| walker |  | 2107 | 41 | listing of '.github/workflows' |  |  | 0.629 |
| walker |  | 2165 | 58 | [package] in pyproject.toml |  |  | 0.631 |
| walker |  | 2312 | 147 | README.md section #0 |  |  | 0.631 |
| walker |  | 2326 | 14 | listing of '.github/ISSUE_TEMPLATE' |  |  | 0.648 |
| ns | 2332 |  | 271 | `Session` attribute set: typed fields + `__attrs__` | 2.5 | 2.4 | 0.610 |
| walker |  | 2403 | 77 | manifest config in pyproject.toml |  |  | 0.615 |
| walker |  | 2423 | 20 | python imports in setup.py |  |  | 0.615 |
| walker |  | 2505 | 82 | tool.setuptools config in pyproject.toml |  |  | 0.615 |
| ns | 2620 |  | 288 | `Session` method roster (complete, with line coordinates) | 2.6 | 2.4 | 0.592 |
| walker |  | 2644 | 139 | [dependencies] in pyproject.toml |  |  | 0.603 |
| walker |  | 2707 | 63 | README.md section #1 |  |  | 0.603 |
| walker |  | 2832 | 125 | package metadata in pyproject.toml |  |  | 0.640 |
| ns | 2871 |  | 251 | `Session.request`: the complete keyword signature | 2.7 | 2.6 | 0.619 |
| walker |  | 2909 | 77 | listing of 'tests' |  |  | 0.652 |
| walker |  | 2923 | 14 | listing of 'tests/certs' |  |  | 0.672 |
| walker |  | 2935 | 12 | listing of 'tests/testserver' |  |  | 0.684 |
| walker |  | 2961 | 26 | python imports in src/requests/packages.py |  |  | 0.684 |
| ns | 3054 |  | 183 | `Session.__init__`: every default value | 2.8 | 2.6 | 0.667 |
| walker |  | 3137 | 176 | README.md section #2 |  |  | 0.667 |
| walker |  | 3210 | 73 | python decl names surface in src/requests/utils.py |  |  | 0.667 |
| walker |  | 3210 | 0 | python decl at src/requests/utils.py:283 |  |  | 0.667 |
| walker |  | 3210 | 0 | python decl at src/requests/utils.py:290 |  |  | 0.667 |
| walker |  | 3231 | 21 | python decl at src/requests/utils.py:328 |  |  | 0.667 |
| walker |  | 3264 | 33 | python decl at src/requests/utils.py:231 |  |  | 0.667 |
| walker |  | 3281 | 17 | python decl doc at src/requests/utils.py:283 |  |  | 0.667 |
| walker |  | 3298 | 17 | python decl doc at src/requests/utils.py:328 |  |  | 0.667 |
| walker |  | 3335 | 37 | python decl at src/requests/utils.py:149 |  |  | 0.667 |
| walker |  | 3348 | 13 | python decl doc at src/requests/utils.py:149 |  |  | 0.667 |
| walker |  | 3367 | 19 | python decl doc at src/requests/utils.py:231 |  |  | 0.667 |
| ns | 3407 |  | 353 | `models.py` module map: constants, all five classes, `Request` fields | 3.1 |  | 0.638 |
| walker |  | 3426 | 59 | python decl doc at src/requests/utils.py:290 |  |  | 0.638 |
| walker |  | 3483 | 57 | python decl names surface in src/requests/sessions.py |  |  | 0.638 |
| walker |  | 3483 | 0 | python decl at src/requests/sessions.py:127 |  |  | 0.638 |
| walker |  | 3483 | 0 | python decl at src/requests/sessions.py:395 |  |  | 0.638 |
| walker |  | 3483 | 0 | python decl at src/requests/sessions.py:908 |  |  | 0.638 |
| walker |  | 3491 | 8 | python decl body at src/requests/sessions.py:908 body 920 |  |  | 0.638 |
| walker |  | 3522 | 31 | python decl at src/requests/sessions.py:76 |  |  | 0.638 |
| walker |  | 3575 | 53 | python decl at src/requests/sessions.py:108 |  |  | 0.638 |
| walker |  | 3607 | 32 | python class body at src/requests/sessions.py:127 |  |  | 0.638 |
| ns | 3728 |  | 321 | `PreparedRequest`: attributes + every `prepare_*` step | 3.2 | 3.1 | 0.617 |
| walker |  | 3880 | 273 | python class body at src/requests/sessions.py:395 |  |  | 0.668 |
| walker |  | 3937 | 57 | python decl doc at src/requests/sessions.py:108 |  |  | 0.668 |
| ns | 4031 |  | 303 | `Response` attribute set: typed fields + `__attrs__` | 3.3 | 3.1 | 0.638 |
| ns | 4313 |  | 282 | `Response`: `__init__`, dunders and every `@property` | 3.4 | 3.3 | 0.621 |
| walker |  | 4411 | 474 | python method sigs in src/requests/sessions.py |  |  | 0.654 |
| walker |  | 4411 | 0 | python method at src/requests/sessions.py:132 |  |  | 0.654 |
| walker |  | 4411 | 0 | python method at src/requests/sessions.py:134 |  |  | 0.654 |
| walker |  | 4411 | 0 | python method at src/requests/sessions.py:154 |  |  | 0.654 |
| walker |  | 4411 | 0 | python method at src/requests/sessions.py:505 |  |  | 0.654 |
| walker |  | 4411 | 0 | python method at src/requests/sessions.py:508 |  |  | 0.654 |
| walker |  | 4411 | 0 | python method at src/requests/sessions.py:511 |  |  | 0.654 |
| walker |  | 4411 | 0 | python method at src/requests/sessions.py:673 |  |  | 0.654 |
| walker |  | 4411 | 0 | python method at src/requests/sessions.py:684 |  |  | 0.654 |
| walker |  | 4411 | 0 | python method at src/requests/sessions.py:742 |  |  | 0.654 |
| walker |  | 4411 | 0 | python method at src/requests/sessions.py:752 |  |  | 0.654 |
| walker |  | 4411 | 0 | python method at src/requests/sessions.py:870 |  |  | 0.654 |
| walker |  | 4411 | 0 | python method at src/requests/sessions.py:883 |  |  | 0.654 |
| walker |  | 4411 | 0 | python method at src/requests/sessions.py:888 |  |  | 0.654 |
| walker |  | 4411 | 0 | python method at src/requests/sessions.py:899 |  |  | 0.654 |
| walker |  | 4411 | 0 | python method at src/requests/sessions.py:903 |  |  | 0.654 |
| walker |  | 4411 | 0 | python method body at src/requests/sessions.py:132 body 132 |  |  | 0.654 |
| walker |  | 4437 | 26 | python method at src/requests/sessions.py:309 |  |  | 0.654 |
| walker |  | 4463 | 26 | python method at src/requests/sessions.py:370 |  |  | 0.654 |
| ns | 4465 |  | 152 | `Response`: the remaining members (`iter_content` … `close`) | 3.5 | 3.4 | 0.644 |
| walker |  | 4479 | 16 | python method doc at src/requests/sessions.py:883 |  |  | 0.644 |
| walker |  | 4497 | 18 | python method doc at src/requests/sessions.py:154 |  |  | 0.645 |
| walker |  | 4517 | 20 | python method doc at src/requests/sessions.py:134 |  |  | 0.645 |
| walker |  | 4560 | 43 | python method at src/requests/sessions.py:714 |  |  | 0.645 |
| walker |  | 4603 | 43 | python method at src/requests/sessions.py:728 |  |  | 0.645 |
| walker |  | 4648 | 45 | python method at src/requests/sessions.py:334 |  |  | 0.645 |
| ns | 4680 |  | 215 | `adapters.py`: pool defaults + the `BaseAdapter` contract | 3.6 |  | 0.630 |
| walker |  | 4698 | 50 | python method at src/requests/sessions.py:442 |  |  | 0.630 |
| walker |  | 4756 | 58 | python method at src/requests/sessions.py:655 |  |  | 0.630 |
| walker |  | 4790 | 34 | python method doc at src/requests/sessions.py:752 |  |  | 0.630 |
| walker |  | 4857 | 67 | python decl doc at src/requests/sessions.py:76 |  |  | 0.630 |
| walker |  | 4896 | 39 | python method doc at src/requests/sessions.py:370 |  |  | 0.630 |
| walker |  | 4937 | 41 | python method doc at src/requests/sessions.py:888 |  |  | 0.630 |
| walker |  | 5009 | 72 | python method at src/requests/sessions.py:695 |  |  | 0.630 |
| ns | 5044 |  | 364 | `HTTPAdapter`: constructor knobs + complete method roster | 3.7 | 3.6 | 0.607 |
| walker |  | 5045 | 36 | python decl names surface in src/requests/help.py |  |  | 0.607 |
| walker |  | 5045 | 0 | python decl at src/requests/help.py:37 |  |  | 0.607 |
| walker |  | 5045 | 0 | python decl at src/requests/help.py:69 |  |  | 0.607 |
| walker |  | 5045 | 0 | python decl at src/requests/help.py:128 |  |  | 0.607 |
| walker |  | 5058 | 13 | python decl doc at src/requests/help.py:69 |  |  | 0.607 |
| walker |  | 5072 | 14 | python decl doc at src/requests/help.py:128 |  |  | 0.607 |
| walker |  | 5090 | 18 | python decl body at src/requests/help.py:128 body 130 |  |  | 0.607 |
| walker |  | 5135 | 45 | python method doc at src/requests/sessions.py:870 |  |  | 0.607 |
| walker |  | 5216 | 81 | python method at src/requests/sessions.py:831 |  |  | 0.607 |
| walker |  | 5258 | 42 | python method doc at src/requests/sessions.py:831 |  |  | 0.607 |
| ns | 5313 |  | 269 | `SessionRedirectMixin` + session-module merge helpers | 3.8 |  | 0.611 |
| walker |  | 5347 | 89 | python decl names surface in src/requests/adapters.py |  |  | 0.613 |
| walker |  | 5347 | 0 | python decl at src/requests/adapters.py:122 |  |  | 0.613 |
| walker |  | 5347 | 0 | python decl at src/requests/adapters.py:158 |  |  | 0.613 |
| walker |  | 5358 | 11 | python decl doc at src/requests/adapters.py:122 |  |  | 0.614 |
| walker |  | 5373 | 15 | python decl doc at src/requests/adapters.py:158 |  |  | 0.614 |
| walker |  | 5527 | 154 | python class body at src/requests/adapters.py:158 |  |  | 0.614 |
| ns | 5549 |  | 236 | `cookies.py` module map: classes + free functions | 3.9 |  | 0.604 |
| walker |  | 5812 | 285 | python method sigs in src/requests/adapters.py |  |  | 0.617 |
| walker |  | 5812 | 0 | python method at src/requests/adapters.py:125 |  |  | 0.617 |
| walker |  | 5812 | 0 | python method at src/requests/adapters.py:153 |  |  | 0.617 |
| walker |  | 5812 | 0 | python method at src/requests/adapters.py:223 |  |  | 0.617 |
| walker |  | 5812 | 0 | python method at src/requests/adapters.py:269 |  |  | 0.617 |
| walker |  | 5812 | 0 | python method at src/requests/adapters.py:365 |  |  | 0.617 |
| walker |  | 5812 | 0 | python method at src/requests/adapters.py:555 |  |  | 0.617 |
| walker |  | 5812 | 0 | python method at src/requests/adapters.py:599 |  |  | 0.617 |
| walker |  | 5812 | 0 | python method at src/requests/adapters.py:613 |  |  | 0.617 |
| walker |  | 5825 | 13 | python method doc at src/requests/adapters.py:153 |  |  | 0.619 |
| walker |  | 5856 | 31 | python method at src/requests/adapters.py:565 |  |  | 0.619 |
| ns | 5857 |  | 308 | `auth.py`: every auth handler and its methods | 3.10 |  | 0.605 |
| walker |  | 5890 | 34 | python method at src/requests/adapters.py:512 |  |  | 0.605 |
| walker |  | 5929 | 39 | python method at src/requests/adapters.py:307 |  |  | 0.605 |
| walker |  | 5969 | 40 | python method at src/requests/adapters.py:226 |  |  | 0.605 |
| walker |  | 6017 | 48 | python method at src/requests/adapters.py:403 |  |  | 0.605 |
| walker |  | 6076 | 59 | python method at src/requests/adapters.py:239 |  |  | 0.605 |
| walker |  | 6148 | 72 | python method at src/requests/adapters.py:455 |  |  | 0.605 |
| walker |  | 6224 | 76 | python method at src/requests/adapters.py:201 |  |  | 0.615 |
| ns | 6229 |  | 372 | `_types.py`: public type-alias roster | 4.1 |  | 0.599 |
| walker |  | 6277 | 53 | python method doc at src/requests/adapters.py:555 |  |  | 0.599 |
| walker |  | 6387 | 110 | declaration surface of tox.ini |  |  | 0.599 |
| ns | 6540 |  | 311 | `_types.py`: the `Unpack` kwargs TypedDicts | 4.2 | 4.1 | 0.584 |
| walker |  | 6544 | 157 | python decl names surface in src/requests/compat.py |  |  | 0.584 |
| walker |  | 6544 | 0 | python decl at src/requests/compat.py:37 |  |  | 0.584 |
| walker |  | 6556 | 12 | python decl doc at src/requests/compat.py:37 |  |  | 0.584 |
| walker |  | 6612 | 56 | python method doc at src/requests/sessions.py:309 |  |  | 0.584 |
| walker |  | 6618 | 6 | listing of 'tests/certs/valid' |  |  | 0.584 |
| walker |  | 6715 | 97 | python method at src/requests/adapters.py:128 |  |  | 0.600 |
| ns | 6739 |  | 199 | Environment-driven settings: the `merge_environment_settings` core | 4.3 | 2.6 | 0.591 |
| walker |  | 6812 | 97 | python method at src/requests/adapters.py:634 |  |  | 0.608 |
| walker |  | 6837 | 25 | python imports in tests/__init__.py |  |  | 0.608 |
| ns | 6846 |  | 107 | `utils.py` module-level constants | 5.1 |  | 0.604 |
| ns | 6994 |  | 148 | `utils.py` function roster 1/3: proxies shim, lengths, netrc, encoders | 5.2 |  | 0.599 |
| walker |  | 7093 | 256 | README.md section #3 |  |  | 0.599 |
| walker |  | 7143 | 50 | python decl names surface in src/requests/status_codes.py |  |  | 0.599 |
| walker |  | 7143 | 0 | python decl at src/requests/status_codes.py:109 |  |  | 0.599 |
| ns | 7161 |  | 167 | `utils.py` function roster 2/3: cookies, encoding detection, URI quoting, CIDR | 5.3 | 5.2 | 0.591 |
| walker |  | 7289 | 146 | python decl names surface in src/requests/models.py |  |  | 0.596 |
| walker |  | 7289 | 0 | python decl at src/requests/models.py:109 |  |  | 0.596 |
| walker |  | 7289 | 0 | python decl at src/requests/models.py:255 |  |  | 0.596 |
| walker |  | 7289 | 0 | python decl at src/requests/models.py:283 |  |  | 0.596 |
| walker |  | 7289 | 0 | python decl at src/requests/models.py:376 |  |  | 0.596 |
| walker |  | 7289 | 0 | python decl at src/requests/models.py:730 |  |  | 0.596 |
| walker |  | 7307 | 18 | python decl doc at src/requests/models.py:283 |  |  | 0.596 |
| ns | 7341 |  | 180 | `utils.py` function roster 3/3: proxy resolution, default headers, header validation | 5.4 | 5.3 | 0.588 |
| walker |  | 7346 | 39 | python decl doc at src/requests/models.py:730 |  |  | 0.588 |
| walker |  | 7357 | 11 | python class body at src/requests/models.py:109 |  |  | 0.588 |
| ns | 7602 |  | 261 | `structures.py`: `CaseInsensitiveDict` and `LookupDict` in full shape | 5.5 |  | 0.578 |
| walker |  | 7616 | 259 | python class body at src/requests/models.py:730 |  |  | 0.611 |
| ns | 7712 |  | 110 | `status_codes.py`: how `requests.codes` is built | 5.6 |  | 0.607 |
| walker |  | 7739 | 123 | python class body at src/requests/models.py:283 |  |  | 0.618 |
| walker |  | 7838 | 99 | python class body at src/requests/models.py:376 |  |  | 0.620 |
| walker |  | 7912 | 74 | python decl at src/requests/models.py:96 |  |  | 0.630 |
| walker |  | 7929 | 17 | python class body at src/requests/models.py:255 |  |  | 0.630 |
| ns | 7954 |  | 242 | The five small modules: hooks, `_internal_utils`, compat, certs, packages | 5.7 |  | 0.621 |
| ns | 8131 |  | 177 | `help.py`: the bug-report payload | 5.8 |  | 0.614 |
| ns | 8464 |  | 333 | `resolve_redirects`: the rules the redirect loop enforces | 6.1 | 3.8 | 0.602 |
| walker |  | 8630 | 701 | python method sigs in src/requests/models.py |  |  | 0.640 |
| walker |  | 8630 | 0 | python method at src/requests/models.py:271 |  |  | 0.640 |
| walker |  | 8630 | 0 | python method at src/requests/models.py:355 |  |  | 0.640 |
| walker |  | 8630 | 0 | python method at src/requests/models.py:358 |  |  | 0.640 |
| walker |  | 8630 | 0 | python method at src/requests/models.py:451 |  |  | 0.640 |
| walker |  | 8630 | 0 | python method at src/requests/models.py:454 |  |  | 0.640 |
| walker |  | 8630 | 0 | python method at src/requests/models.py:465 |  |  | 0.640 |
| walker |  | 8630 | 0 | python method at src/requests/models.py:563 |  |  | 0.640 |
| walker |  | 8630 | 0 | python method at src/requests/models.py:652 |  |  | 0.640 |
| walker |  | 8630 | 0 | python method at src/requests/models.py:720 |  |  | 0.640 |
| walker |  | 8630 | 0 | python method at src/requests/models.py:763 |  |  | 0.640 |
| walker |  | 8630 | 0 | python method at src/requests/models.py:810 |  |  | 0.640 |
| walker |  | 8630 | 0 | python method at src/requests/models.py:813 |  |  | 0.640 |
| walker |  | 8630 | 0 | python method at src/requests/models.py:824 |  |  | 0.640 |
| walker |  | 8630 | 0 | python method at src/requests/models.py:832 |  |  | 0.640 |
| walker |  | 8630 | 0 | python method at src/requests/models.py:835 |  |  | 0.640 |
| walker |  | 8630 | 0 | python method at src/requests/models.py:845 |  |  | 0.640 |
| walker |  | 8630 | 0 | python method at src/requests/models.py:855 |  |  | 0.640 |
| walker |  | 8630 | 0 | python method at src/requests/models.py:1087 |  |  | 0.640 |
| walker |  | 8630 | 0 | python method at src/requests/models.py:1140 |  |  | 0.640 |
| walker |  | 8630 | 0 | python method at src/requests/models.py:1169 |  |  | 0.640 |
| walker |  | 8636 | 6 | python method at src/requests/models.py:112 |  |  | 0.640 |
| walker |  | 8644 | 8 | python method at src/requests/models.py:859 |  |  | 0.641 |
| walker |  | 8652 | 8 | python method at src/requests/models.py:874 |  |  | 0.642 |
| walker |  | 8660 | 8 | python method at src/requests/models.py:881 |  |  | 0.644 |
| walker |  | 8668 | 8 | python method at src/requests/models.py:889 |  |  | 0.645 |
| walker |  | 8676 | 8 | python method at src/requests/models.py:894 |  |  | 0.646 |
| walker |  | 8685 | 9 | python method at src/requests/models.py:1030 |  |  | 0.647 |
| walker |  | 8694 | 9 | python method at src/requests/models.py:1049 |  |  | 0.649 |
| walker |  | 8703 | 9 | python method at src/requests/models.py:1122 |  |  | 0.650 |
| walker |  | 8717 | 14 | python method at src/requests/models.py:405 |  |  | 0.650 |
| walker |  | 8725 | 8 | python method at src/requests/models.py:471 |  |  | 0.652 |
| walker |  | 8737 | 12 | python method doc at src/requests/models.py:720 |  |  | 0.652 |
| walker |  | 8750 | 13 | python method doc at src/requests/models.py:112 |  |  | 0.652 |
| ns | 8753 |  | 289 | `HTTPAdapter.send`: the urllib3 call and the `MaxRetryError` fan-out | 6.2 | 3.7 | 0.639 |
| walker |  | 8763 | 13 | python method doc at src/requests/models.py:465 |  |  | 0.639 |
| walker |  | 8776 | 13 | python method doc at src/requests/models.py:563 |  |  | 0.639 |
| walker |  | 8805 | 29 | python method at src/requests/models.py:816 |  |  | 0.639 |
| walker |  | 8820 | 15 | python method doc at src/requests/models.py:1030 |  |  | 0.639 |
| walker |  | 8853 | 33 | python method at src/requests/models.py:697 |  |  | 0.639 |
| walker |  | 8869 | 16 | python method doc at src/requests/models.py:855 |  |  | 0.639 |
| walker |  | 8886 | 17 | python method doc at src/requests/models.py:652 |  |  | 0.639 |
| walker |  | 8922 | 36 | python method at src/requests/models.py:258 |  |  | 0.639 |
| walker |  | 8934 | 12 | python method doc at src/requests/models.py:258 |  |  | 0.639 |
| ns | 8955 |  | 202 | `HTTPAdapter.send`: the remaining `except` arms and the return | 6.3 | 6.2 | 0.631 |
| walker |  | 8970 | 36 | python method at src/requests/models.py:912 |  |  | 0.631 |
| walker |  | 8988 | 18 | python method doc at src/requests/models.py:881 |  |  | 0.631 |
| walker |  | 9006 | 18 | python method doc at src/requests/models.py:1140 |  |  | 0.631 |
| walker |  | 9025 | 19 | python method doc at src/requests/models.py:1122 |  |  | 0.631 |
| ns | 9043 |  | 88 | Makefile: the remaining targets (coverage, publish, docs) | 7.1 | 1.8 | 0.634 |
| walker |  | 9064 | 39 | python method at src/requests/models.py:574 |  |  | 0.634 |
| walker |  | 9078 | 14 | python method doc at src/requests/models.py:574 |  |  | 0.634 |
| walker |  | 9118 | 40 | python method at src/requests/models.py:481 |  |  | 0.634 |
| walker |  | 9131 | 13 | python method doc at src/requests/models.py:481 |  |  | 0.634 |
| walker |  | 9172 | 41 | python method at src/requests/models.py:668 |  |  | 0.634 |
| walker |  | 9186 | 14 | python method doc at src/requests/models.py:668 |  |  | 0.634 |
| walker |  | 9208 | 22 | python method doc at src/requests/models.py:894 |  |  | 0.634 |
| walker |  | 9232 | 24 | python method doc at src/requests/models.py:358 |  |  | 0.634 |
| walker |  | 9256 | 24 | python method doc at src/requests/models.py:889 |  |  | 0.634 |
| walker |  | 9312 | 56 | python method at src/requests/models.py:908 |  |  | 0.634 |
| walker |  | 9312 | 0 | python method body at src/requests/models.py:908 body 911 |  |  | 0.634 |
| ns | 9326 |  | 283 | pyproject: every remaining table, and the settings that change how you work here | 7.2 | 1.10 | 0.626 |
| walker |  | 9369 | 57 | python method at src/requests/models.py:904 |  |  | 0.626 |
| walker |  | 9369 | 0 | python method body at src/requests/models.py:904 body 907 |  |  | 0.626 |
| walker |  | 9430 | 61 | python method at src/requests/models.py:990 |  |  | 0.626 |
| walker |  | 9465 | 35 | python method doc at src/requests/models.py:271 |  |  | 0.626 |
| ns | 9481 |  | 155 | `tests/conftest.py`: the fixture set every test builds on | 7.3 |  | 0.621 |
| walker |  | 9511 | 46 | python method doc at src/requests/models.py:874 |  |  | 0.621 |
| ns | 9576 |  | 95 | `tests/test_requests.py`: top-level class roster | 7.4 |  | 0.617 |
| walker |  | 9593 | 82 | python method at src/requests/models.py:975 |  |  | 0.617 |
| walker |  | 9593 | 0 | python method body at src/requests/models.py:975 body 981 |  |  | 0.617 |
| walker |  | 9677 | 84 | python method at src/requests/models.py:982 |  |  | 0.617 |
| walker |  | 9677 | 0 | python method body at src/requests/models.py:982 body 989 |  |  | 0.617 |
| walker |  | 9713 | 36 | python method at src/requests/models.py:133 |  |  | 0.617 |
| walker |  | 9713 | 0 | python method body at src/requests/models.py:133 body 135 |  |  | 0.617 |
| walker |  | 9747 | 34 | python method at src/requests/models.py:137 |  |  | 0.617 |
| walker |  | 9747 | 0 | python method body at src/requests/models.py:137 body 139 |  |  | 0.617 |
| ns | 9758 |  | 182 | `tests/testserver/server.py`: the local socket server API (complete) | 7.5 |  | 0.611 |
| walker |  | 9788 | 41 | python method at src/requests/models.py:147 |  |  | 0.611 |
| walker |  | 9788 | 0 | python method body at src/requests/models.py:147 body 149 |  |  | 0.611 |
| walker |  | 9829 | 41 | python method at src/requests/models.py:151 |  |  | 0.611 |
| walker |  | 9872 | 43 | python method at src/requests/models.py:183 |  |  | 0.611 |
| ns | 9915 |  | 157 | `docs/api.rst`: every section of the developer interface | 7.6 |  | 0.605 |
| ns | 9963 |  | 48 | `HISTORY.md`: the changelog's head | 7.7 |  | 0.603 |
