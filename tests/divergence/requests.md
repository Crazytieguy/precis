Score(3000)=0.576 I=0.718 C=0.462 ns_rows≤3K=17/49 grid(1000/1442/2080/3000/4327/6240/9000)=0.692/0.639/0.618/0.576/0.579/0.625/0.599

| source | ns_cum | walker_cum | marginal | descriptor | id | predecessor | Score(B=cum) |
|:-------|-------:|-----------:|---------:|:-----------|:---|:------------|-------------:|
| ns | 47 |  | 47 | Identity lede: what the library is, and its version | 1.1 |  | 0.000 |
| walker |  | 74 | 74 | listing of '.' |  |  | 0.000 |
| walker |  | 77 | 3 | listing of 'src' |  |  | 0.000 |
| walker |  | 111 | 34 | listing of 'ext' |  |  | 0.000 |
| ns | 121 |  | 74 | Repository root listing (complete) | 1.2 |  | 0.671 |
| walker |  | 169 | 58 | [package] in pyproject.toml |  |  | 0.672 |
| ns | 216 |  | 95 | `src/requests/` module roster (complete) | 1.3 |  | 0.456 |
| walker |  | 223 | 54 | listing of 'docs' |  |  | 0.462 |
| walker |  | 227 | 4 | listing of 'docs/_templates' |  |  | 0.462 |
| walker |  | 236 | 9 | listing of 'docs/_static' |  |  | 0.462 |
| walker |  | 246 | 10 | listing of 'docs/dev' |  |  | 0.464 |
| walker |  | 260 | 14 | listing of 'docs/_themes' |  |  | 0.464 |
| walker |  | 281 | 21 | listing of 'docs/user' |  |  | 0.468 |
| walker |  | 319 | 38 | listing of 'docs/community' |  |  | 0.379 |
| ns | 319 |  | 103 | `tests/` roster (complete, incl. `testserver/` and `certs/`) | 1.4 |  | 0.379 |
| walker |  | 411 | 92 | listing of 'src/requests' |  |  | 0.607 |
| ns | 466 |  | 147 | README: the canonical `requests.get(...)` doctest + every `##` heading | 1.5 | 1.1 | 0.541 |
| ns | 584 |  | 118 | Full package metadata block (`__version__.py`) | 1.6 | 1.1 | 0.507 |
| ns | 682 |  | 98 | CI: `.github/` and workflow file roster (complete) | 1.7 |  | 0.450 |
| ns | 762 |  | 80 | Makefile: install / test / CI targets | 1.8 |  | 0.431 |
| walker |  | 767 | 356 | plaintext config Makefile |  |  | 0.477 |
| ns | 885 |  | 123 | `docs/` tree listing (complete) | 1.9 |  | 0.523 |
| walker |  | 942 | 175 | README headline in README.md |  |  | 0.660 |
| walker |  | 982 | 40 | headings outline in README.md |  |  | 0.692 |
| ns | 1187 |  | 302 | pyproject: build backend, metadata, runtime dependencies | 1.10 |  | 0.630 |
| walker |  | 1292 | 310 | python names src/requests/__init__.py |  |  | 0.639 |
| walker |  | 1339 | 47 | python decl src/requests/__init__.py:60 |  |  | 0.639 |
| walker |  | 1375 | 36 | python names src/requests/help.py |  |  | 0.639 |
| ns | 1458 |  | 271 | `requests/__init__.py`: name → module re-export map | 2.1 |  | 0.644 |
| ns | 1532 |  | 74 | `api.py`: names of all eight module-level functions | 2.2 |  | 0.630 |
| walker |  | 1725 | 350 | python module doc src/requests/__init__.py |  |  | 0.630 |
| walker |  | 1866 | 141 | [dependencies] in pyproject.toml |  |  | 0.640 |
| ns | 1885 |  | 353 | `exceptions.py`: the complete class hierarchy | 2.3 |  | 0.599 |
| walker |  | 1909 | 43 | listing of '.github' |  |  | 0.612 |
| walker |  | 1950 | 41 | listing of '.github/workflows' |  |  | 0.646 |
| walker |  | 2000 | 50 | python names src/requests/status_codes.py |  |  | 0.646 |
| ns | 2061 |  | 176 | `Session` class declaration + docstring | 2.4 |  | 0.618 |
| walker |  | 2147 | 147 | README.md section #0 |  |  | 0.618 |
| walker |  | 2161 | 14 | listing of '.github/ISSUE_TEMPLATE' |  |  | 0.635 |
| walker |  | 2218 | 57 | python names src/requests/hooks.py |  |  | 0.635 |
| walker |  | 2271 | 53 | python decl src/requests/hooks.py:32 |  |  | 0.635 |
| walker |  | 2328 | 57 | python names src/requests/sessions.py |  |  | 0.635 |
| ns | 2332 |  | 271 | `Session` attribute set: typed fields + `__attrs__` | 2.5 | 2.4 | 0.598 |
| walker |  | 2359 | 31 | python decl src/requests/sessions.py:76 |  |  | 0.598 |
| walker |  | 2367 | 8 | python body src/requests/sessions.py:908 |  |  | 0.598 |
| walker |  | 2420 | 53 | python decl src/requests/sessions.py:108 |  |  | 0.598 |
| walker |  | 2483 | 63 | README.md section #1 |  |  | 0.598 |
| walker |  | 2556 | 73 | python names src/requests/structures.py |  |  | 0.598 |
| walker |  | 2569 | 13 | python doc src/requests/help.py:69 |  |  | 0.598 |
| walker |  | 2583 | 14 | python doc src/requests/help.py:128 |  |  | 0.598 |
| ns | 2620 |  | 288 | `Session` method roster (complete, with line coordinates) | 2.6 | 2.4 | 0.576 |
| walker |  | 2672 | 89 | python names src/requests/adapters.py |  |  | 0.576 |
| walker |  | 2712 | 40 | python decl src/requests/adapters.py:122 |  |  | 0.576 |
| walker |  | 2721 | 9 | python doc src/requests/adapters.py:122 |  |  | 0.576 |
| walker |  | 2728 | 7 | python body src/requests/adapters.py:125 |  |  | 0.576 |
| ns | 2871 |  | 251 | `Session.request`: the complete keyword signature | 2.7 | 2.6 | 0.557 |
| walker |  | 2997 | 269 | python names src/requests/__init__.py #1 |  |  | 0.576 |
| ns | 3054 |  | 183 | `Session.__init__`: every default value | 2.8 | 2.6 | 0.562 |
| walker |  | 3122 | 125 | package metadata in pyproject.toml |  |  | 0.591 |
| walker |  | 3199 | 77 | listing of 'tests' |  |  | 0.624 |
| walker |  | 3213 | 14 | listing of 'tests/certs' |  |  | 0.643 |
| walker |  | 3225 | 12 | listing of 'tests/testserver' |  |  | 0.654 |
| walker |  | 3264 | 39 | plaintext config docs/requirements.txt |  |  | 0.654 |
| walker |  | 3281 | 17 | python body src/requests/hooks.py:25 |  |  | 0.654 |
| walker |  | 3299 | 18 | python doc src/requests/hooks.py:32 |  |  | 0.654 |
| walker |  | 3317 | 18 | python body src/requests/help.py:128 |  |  | 0.654 |
| walker |  | 3327 | 10 | python body src/requests/adapters.py:153 |  |  | 0.654 |
| ns | 3407 |  | 353 | `models.py` module map: constants, all five classes, `Request` fields | 3.1 |  | 0.626 |
| walker |  | 3453 | 126 | python names src/requests/auth.py |  |  | 0.626 |
| walker |  | 3475 | 22 | python decl src/requests/auth.py:78 |  |  | 0.626 |
| walker |  | 3497 | 22 | python decl src/requests/auth.py:116 |  |  | 0.627 |
| walker |  | 3510 | 13 | python doc src/requests/auth.py:78 |  |  | 0.627 |
| walker |  | 3525 | 15 | python doc src/requests/auth.py:116 |  |  | 0.627 |
| walker |  | 3659 | 134 | python names src/requests/api.py |  |  | 0.643 |
| walker |  | 3694 | 35 | python decl src/requests/api.py:24 |  |  | 0.643 |
| ns | 3728 |  | 321 | `PreparedRequest`: attributes + every `prepare_*` step | 3.2 | 3.1 | 0.622 |
| walker |  | 3734 | 40 | python decl src/requests/api.py:74 |  |  | 0.622 |
| walker |  | 3774 | 40 | python decl src/requests/api.py:137 |  |  | 0.622 |
| walker |  | 3814 | 40 | python decl src/requests/api.py:154 |  |  | 0.622 |
| walker |  | 3878 | 64 | python decl src/requests/api.py:117 |  |  | 0.622 |
| walker |  | 3891 | 13 | python module doc tests/__init__.py |  |  | 0.622 |
| ns | 4031 |  | 303 | `Response` attribute set: typed fields + `__attrs__` | 3.3 | 3.1 | 0.594 |
| walker |  | 4138 | 247 | python names src/requests/utils.py |  |  | 0.595 |
| walker |  | 4146 | 8 | python decl src/requests/utils.py:370 |  |  | 0.595 |
| walker |  | 4156 | 10 | python decl src/requests/utils.py:328 |  |  | 0.595 |
| walker |  | 4184 | 28 | python decl src/requests/utils.py:91 |  |  | 0.595 |
| walker |  | 4217 | 33 | python decl src/requests/utils.py:231 |  |  | 0.595 |
| walker |  | 4254 | 37 | python decl src/requests/utils.py:149 |  |  | 0.595 |
| walker |  | 4291 | 37 | python decl src/requests/utils.py:341 |  |  | 0.595 |
| ns | 4313 |  | 282 | `Response`: `__init__`, dunders and every `@property` | 3.4 | 3.3 | 0.579 |
| walker |  | 4448 | 157 | python names src/requests/compat.py |  |  | 0.580 |
| ns | 4465 |  | 152 | `Response`: the remaining members (`iter_content` … `close`) | 3.5 | 3.4 | 0.571 |
| walker |  | 4594 | 146 | python names src/requests/models.py |  |  | 0.577 |
| walker |  | 4653 | 59 | python decl src/requests/models.py:255 |  |  | 0.577 |
| ns | 4680 |  | 215 | `adapters.py`: pool defaults + the `BaseAdapter` contract | 3.6 |  | 0.570 |
| walker |  | 4689 | 36 | python decl src/requests/models.py:258 |  |  | 0.570 |
| walker |  | 4763 | 74 | python decl src/requests/models.py:96 |  |  | 0.578 |
| walker |  | 4887 | 124 | python decl src/requests/models.py:109 |  |  | 0.578 |
| walker |  | 4893 | 6 | python decl src/requests/models.py:112 |  |  | 0.578 |
| walker |  | 4908 | 15 | python decl src/requests/models.py:137 |  |  | 0.578 |
| walker |  | 4925 | 17 | python decl src/requests/models.py:133 |  |  | 0.578 |
| walker |  | 4942 | 17 | python decl src/requests/models.py:147 |  |  | 0.578 |
| ns | 5044 |  | 364 | `HTTPAdapter`: constructor knobs + complete method roster | 3.7 | 3.6 | 0.558 |
| walker |  | 5100 | 158 | python module doc src/requests/__version__.py |  |  | 0.617 |
| ns | 5313 |  | 269 | `SessionRedirectMixin` + session-module merge helpers | 3.8 |  | 0.604 |
| walker |  | 5453 | 353 | python names src/requests/exceptions.py |  |  | 0.645 |
| walker |  | 5461 | 8 | python doc src/requests/exceptions.py:106 |  |  | 0.645 |
| walker |  | 5509 | 48 | python decl src/requests/exceptions.py:42 |  |  | 0.645 |
| walker |  | 5518 | 9 | python doc src/requests/exceptions.py:38 |  |  | 0.645 |
| walker |  | 5527 | 9 | python doc src/requests/exceptions.py:66 |  |  | 0.645 |
| ns | 5549 |  | 236 | `cookies.py` module map: classes + free functions | 3.9 |  | 0.634 |
| walker |  | 5583 | 56 | python decl src/requests/exceptions.py:20 |  |  | 0.634 |
| walker |  | 5592 | 9 | python doc src/requests/exceptions.py:70 |  |  | 0.634 |
| walker |  | 5601 | 9 | python doc src/requests/exceptions.py:74 |  |  | 0.634 |
| walker |  | 5765 | 164 | python names src/requests/_types.py |  |  | 0.635 |
| walker |  | 5794 | 29 | python decl src/requests/_types.py:27 |  |  | 0.635 |
| walker |  | 5824 | 30 | python decl src/requests/_types.py:32 |  |  | 0.635 |
| walker |  | 5839 | 15 | python doc src/requests/_types.py:42 |  |  | 0.635 |
| ns | 5857 |  | 308 | `auth.py`: every auth handler and its methods | 3.10 |  | 0.627 |
| walker |  | 5996 | 157 | python decl src/requests/sessions.py:127 |  |  | 0.640 |
| walker |  | 6022 | 26 | python decl src/requests/sessions.py:309 |  |  | 0.640 |
| walker |  | 6048 | 26 | python decl src/requests/sessions.py:370 |  |  | 0.640 |
| walker |  | 6093 | 45 | python decl src/requests/sessions.py:334 |  |  | 0.640 |
| ns | 6229 |  | 372 | `_types.py`: public type-alias roster | 4.1 |  | 0.625 |
| walker |  | 6260 | 167 | python decl src/requests/auth.py:85 |  |  | 0.629 |
| walker |  | 6267 | 7 | python decl src/requests/auth.py:91 |  |  | 0.629 |
| walker |  | 6274 | 7 | python decl src/requests/auth.py:93 |  |  | 0.629 |
| walker |  | 6442 | 168 | python decl src/requests/models.py:283 |  |  | 0.651 |
| walker |  | 6515 | 73 | declaration surface of requirements-dev.txt |  |  | 0.651 |
| walker |  | 6526 | 11 | python doc src/requests/adapters.py:153 |  |  | 0.653 |
| ns | 6540 |  | 311 | `_types.py`: the `Unpack` kwargs TypedDicts | 4.2 | 4.1 | 0.637 |
| walker |  | 6623 | 97 | python decl src/requests/adapters.py:128 |  |  | 0.653 |
| walker |  | 6638 | 15 | python body src/requests/api.py:90 |  |  | 0.653 |
| ns | 6739 |  | 199 | Environment-driven settings: the `merge_environment_settings` core | 4.3 | 2.6 | 0.643 |
| ns | 6846 |  | 107 | `utils.py` module-level constants | 5.1 |  | 0.647 |
| walker |  | 6918 | 280 | python names src/requests/cookies.py |  |  | 0.657 |
| walker |  | 6945 | 27 | python decl src/requests/cookies.py:135 |  |  | 0.657 |
| walker |  | 6978 | 33 | python decl src/requests/cookies.py:604 |  |  | 0.657 |
| ns | 6994 |  | 148 | `utils.py` function roster 1/3: proxies shim, lengths, netrc, encoders | 5.2 |  | 0.653 |
| walker |  | 7017 | 39 | python decl src/requests/cookies.py:164 |  |  | 0.653 |
| walker |  | 7068 | 51 | python decl src/requests/cookies.py:579 |  |  | 0.653 |
| walker |  | 7121 | 53 | python decl src/requests/cookies.py:114 |  |  | 0.653 |
| ns | 7161 |  | 167 | `utils.py` function roster 2/3: cookies, encoding detection, URI quoting, CIDR | 5.3 | 5.2 | 0.645 |
| walker |  | 7177 | 56 | python decl src/requests/cookies.py:563 |  |  | 0.645 |
| walker |  | 7233 | 56 | python decl src/requests/cookies.py:571 |  |  | 0.645 |
| walker |  | 7242 | 9 | python doc src/requests/exceptions.py:78 |  |  | 0.645 |
| ns | 7341 |  | 180 | `utils.py` function roster 3/3: proxy resolution, default headers, header validation | 5.4 | 5.3 | 0.636 |
| walker |  | 7444 | 202 | python decl src/requests/structures.py:96 |  |  | 0.637 |
| walker |  | 7451 | 7 | python decl src/requests/structures.py:126 |  |  | 0.637 |
| walker |  | 7460 | 9 | python decl src/requests/structures.py:123 |  |  | 0.637 |
| walker |  | 7468 | 8 | python doc src/requests/structures.py:96 |  |  | 0.637 |
| ns | 7602 |  | 261 | `structures.py`: `CaseInsensitiveDict` and `LookupDict` in full shape | 5.5 |  | 0.629 |
| walker |  | 7678 | 210 | python decl src/requests/structures.py:20 |  |  | 0.644 |
| ns | 7712 |  | 110 | `status_codes.py`: how `requests.codes` is built | 5.6 |  | 0.640 |
| walker |  | 7732 | 54 | python decl src/requests/structures.py:49 |  |  | 0.640 |
| walker |  | 7954 | 222 | python names src/requests/_internal_utils.py |  |  | 0.635 |
| ns | 7954 |  | 242 | The five small modules: hooks, `_internal_utils`, compat, certs, packages | 5.7 |  | 0.635 |
| walker |  | 7983 | 29 | python decl src/requests/_internal_utils.py:20 |  |  | 0.635 |
| walker |  | 7996 | 13 | python doc src/requests/utils.py:149 |  |  | 0.635 |
| walker |  | 8008 | 12 | python doc src/requests/compat.py:37 |  |  | 0.637 |
| walker |  | 8078 | 70 | python decl src/requests/adapters.py:85 |  |  | 0.637 |
| ns | 8131 |  | 177 | `help.py`: the bug-report payload | 5.8 |  | 0.630 |
| walker |  | 8213 | 135 | python decl src/requests/sessions.py:186 |  |  | 0.632 |
| walker |  | 8222 | 9 | python doc src/requests/exceptions.py:142 |  |  | 0.632 |
| walker |  | 8237 | 15 | python body src/requests/api.py:171 |  |  | 0.632 |
| walker |  | 8278 | 41 | python decl src/requests/models.py:151 |  |  | 0.632 |
| walker |  | 8388 | 110 | declaration surface of tox.ini |  |  | 0.632 |
| ns | 8464 |  | 333 | `resolve_redirects`: the rules the redirect loop enforces | 6.1 | 3.8 | 0.619 |
| walker |  | 8664 | 276 | python decl src/requests/cookies.py:31 |  |  | 0.619 |
| walker |  | 8672 | 8 | python decl src/requests/cookies.py:101 |  |  | 0.619 |
| walker |  | 8680 | 8 | python decl src/requests/cookies.py:105 |  |  | 0.619 |
| walker |  | 8688 | 8 | python decl src/requests/cookies.py:109 |  |  | 0.619 |
| walker |  | 8731 | 43 | python decl src/requests/models.py:183 |  |  | 0.619 |
| ns | 8753 |  | 289 | `HTTPAdapter.send`: the urllib3 call and the `MaxRetryError` fan-out | 6.2 | 3.7 | 0.607 |
| walker |  | 8853 | 122 | python body src/requests/__init__.py:99 |  |  | 0.607 |
| walker |  | 8899 | 46 | python decl src/requests/models.py:141 |  |  | 0.607 |
| walker |  | 8908 | 9 | python body src/requests/structures.py:73 |  |  | 0.607 |
| ns | 8955 |  | 202 | `HTTPAdapter.send`: the remaining `except` arms and the return | 6.3 | 6.2 | 0.599 |
| walker |  | 8958 | 50 | python body src/requests/_types.py:42 |  |  | 0.599 |
| ns | 9043 |  | 88 | Makefile: the remaining targets (coverage, publish, docs) | 7.1 | 1.8 | 0.604 |
| walker |  | 9280 | 322 | python decl src/requests/models.py:376 |  |  | 0.624 |
| walker |  | 9288 | 8 | python decl src/requests/models.py:471 |  |  | 0.625 |
| walker |  | 9321 | 33 | python decl src/requests/models.py:697 |  |  | 0.625 |
| ns | 9326 |  | 283 | pyproject: every remaining table, and the settings that change how you work here | 7.2 | 1.10 | 0.613 |
| walker |  | 9360 | 39 | python decl src/requests/models.py:574 |  |  | 0.613 |
| walker |  | 9400 | 40 | python decl src/requests/models.py:481 |  |  | 0.613 |
| walker |  | 9441 | 41 | python decl src/requests/models.py:668 |  |  | 0.613 |
| ns | 9481 |  | 155 | `tests/conftest.py`: the fixture set every test builds on | 7.3 |  | 0.608 |
| walker |  | 9494 | 53 | python body src/requests/_internal_utils.py:26 |  |  | 0.608 |
| ns | 9576 |  | 95 | `tests/test_requests.py`: top-level class roster | 7.4 |  | 0.605 |
| walker |  | 9665 | 171 | python decl src/requests/models.py:321 |  |  | 0.605 |
| walker |  | 9721 | 56 | python doc src/requests/_internal_utils.py:26 |  |  | 0.605 |
| walker |  | 9730 | 9 | python doc src/requests/exceptions.py:153 |  |  | 0.605 |
| ns | 9758 |  | 182 | `tests/testserver/server.py`: the local socket server API (complete) | 7.5 |  | 0.599 |
| ns | 9915 |  | 157 | `docs/api.rst`: every section of the developer interface | 7.6 |  | 0.593 |
| ns | 9963 |  | 48 | `HISTORY.md`: the changelog's head | 7.7 |  | 0.591 |
