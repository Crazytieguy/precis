Score(3000)=0.553 I=0.709 C=0.431 ns_rows≤3K=17/49 grid(1000/1442/2080/3000/4327/6240/9000)=0.691/0.635/0.609/0.553/0.575/0.582/0.592

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
| walker |  | 724 | 15 | python module doc src/requests/help.py |  |  | 0.456 |
| ns | 762 |  | 80 | Makefile: install / test / CI targets | 1.8 |  | 0.477 |
| ns | 885 |  | 123 | `docs/` tree listing (complete) | 1.9 |  | 0.522 |
| walker |  | 899 | 175 | README headline in README.md |  |  | 0.660 |
| walker |  | 939 | 40 | headings outline in README.md |  |  | 0.691 |
| ns | 1187 |  | 302 | pyproject: build backend, metadata, runtime dependencies | 1.10 |  | 0.627 |
| walker |  | 1249 | 310 | python names src/requests/__init__.py |  |  | 0.635 |
| walker |  | 1296 | 47 | python decl src/requests/__init__.py:60 |  |  | 0.635 |
| walker |  | 1330 | 34 | python names src/requests/help.py |  |  | 0.635 |
| ns | 1458 |  | 271 | `requests/__init__.py`: name → module re-export map | 2.1 |  | 0.641 |
| ns | 1532 |  | 74 | `api.py`: names of all eight module-level functions | 2.2 |  | 0.627 |
| walker |  | 1680 | 350 | python module doc src/requests/__init__.py |  |  | 0.627 |
| walker |  | 1722 | 42 | python module doc src/requests/structures.py |  |  | 0.627 |
| walker |  | 1766 | 44 | python module doc src/requests/auth.py |  |  | 0.627 |
| walker |  | 1811 | 45 | python module doc src/requests/exceptions.py |  |  | 0.627 |
| walker |  | 1857 | 46 | python module doc src/requests/models.py |  |  | 0.627 |
| ns | 1885 |  | 353 | `exceptions.py`: the complete class hierarchy | 2.3 |  | 0.586 |
| walker |  | 1900 | 43 | listing of '.github' |  |  | 0.600 |
| walker |  | 1941 | 41 | listing of '.github/workflows' |  |  | 0.633 |
| walker |  | 1999 | 58 | [package] in pyproject.toml |  |  | 0.636 |
| walker |  | 2049 | 50 | python names src/requests/status_codes.py |  |  | 0.636 |
| ns | 2061 |  | 176 | `Session` class declaration + docstring | 2.4 |  | 0.609 |
| walker |  | 2103 | 54 | python module doc src/requests/adapters.py |  |  | 0.609 |
| walker |  | 2250 | 147 | README.md section #0 |  |  | 0.609 |
| walker |  | 2264 | 14 | listing of '.github/ISSUE_TEMPLATE' |  |  | 0.626 |
| walker |  | 2323 | 59 | python module doc src/requests/sessions.py |  |  | 0.626 |
| ns | 2332 |  | 271 | `Session` attribute set: typed fields + `__attrs__` | 2.5 | 2.4 | 0.590 |
| walker |  | 2382 | 59 | python module doc src/requests/utils.py |  |  | 0.590 |
| walker |  | 2437 | 55 | python names src/requests/sessions.py |  |  | 0.590 |
| walker |  | 2468 | 31 | python decl src/requests/sessions.py:76 |  |  | 0.590 |
| walker |  | 2476 | 8 | python body src/requests/sessions.py:908 |  |  | 0.590 |
| walker |  | 2529 | 53 | python decl src/requests/sessions.py:108 |  |  | 0.590 |
| walker |  | 2606 | 77 | manifest config in pyproject.toml |  |  | 0.594 |
| ns | 2620 |  | 288 | `Session` method roster (complete, with line coordinates) | 2.6 | 2.4 | 0.572 |
| walker |  | 2663 | 57 | python names src/requests/hooks.py |  |  | 0.572 |
| walker |  | 2716 | 53 | python decl src/requests/hooks.py:32 |  |  | 0.572 |
| walker |  | 2780 | 64 | python module doc src/requests/_internal_utils.py |  |  | 0.572 |
| walker |  | 2862 | 82 | tool.setuptools config in pyproject.toml |  |  | 0.572 |
| ns | 2871 |  | 251 | `Session.request`: the complete keyword signature | 2.7 | 2.6 | 0.553 |
| walker |  | 2934 | 72 | python module doc src/requests/cookies.py |  |  | 0.553 |
| ns | 3054 |  | 183 | `Session.__init__`: every default value | 2.8 | 2.6 | 0.539 |
| walker |  | 3073 | 139 | [dependencies] in pyproject.toml |  |  | 0.550 |
| walker |  | 3147 | 74 | python module doc src/requests/compat.py |  |  | 0.550 |
| walker |  | 3210 | 63 | README.md section #1 |  |  | 0.550 |
| walker |  | 3281 | 71 | python names src/requests/structures.py |  |  | 0.550 |
| walker |  | 3358 | 77 | python module doc src/requests/_types.py |  |  | 0.550 |
| ns | 3407 |  | 353 | `models.py` module map: constants, all five classes, `Request` fields | 3.1 |  | 0.526 |
| walker |  | 3439 | 81 | python module doc src/requests/api.py |  |  | 0.526 |
| walker |  | 3522 | 83 | python module doc src/requests/hooks.py |  |  | 0.526 |
| walker |  | 3535 | 13 | python doc src/requests/help.py:69 |  |  | 0.526 |
| walker |  | 3549 | 14 | python doc src/requests/help.py:128 |  |  | 0.526 |
| walker |  | 3636 | 87 | python names src/requests/adapters.py |  |  | 0.527 |
| walker |  | 3676 | 40 | python decl src/requests/adapters.py:122 |  |  | 0.527 |
| walker |  | 3685 | 9 | python doc src/requests/adapters.py:122 |  |  | 0.527 |
| walker |  | 3692 | 7 | python body src/requests/adapters.py:125 |  |  | 0.527 |
| ns | 3728 |  | 321 | `PreparedRequest`: attributes + every `prepare_*` step | 3.2 | 3.1 | 0.510 |
| walker |  | 3961 | 269 | python names src/requests/__init__.py #1 |  |  | 0.527 |
| ns | 4031 |  | 303 | `Response` attribute set: typed fields + `__attrs__` | 3.3 | 3.1 | 0.503 |
| walker |  | 4086 | 125 | package metadata in pyproject.toml |  |  | 0.534 |
| walker |  | 4163 | 77 | listing of 'tests' |  |  | 0.563 |
| walker |  | 4177 | 14 | listing of 'tests/certs' |  |  | 0.580 |
| walker |  | 4189 | 12 | listing of 'tests/testserver' |  |  | 0.590 |
| walker |  | 4206 | 17 | python body src/requests/hooks.py:25 |  |  | 0.590 |
| walker |  | 4224 | 18 | python doc src/requests/hooks.py:32 |  |  | 0.590 |
| ns | 4313 |  | 282 | `Response`: `__init__`, dunders and every `@property` | 3.4 | 3.3 | 0.575 |
| walker |  | 4345 | 121 | python module doc src/requests/certs.py |  |  | 0.575 |
| walker |  | 4363 | 18 | python body src/requests/help.py:128 |  |  | 0.575 |
| walker |  | 4373 | 10 | python body src/requests/adapters.py:153 |  |  | 0.575 |
| ns | 4465 |  | 152 | `Response`: the remaining members (`iter_content` … `close`) | 3.5 | 3.4 | 0.566 |
| walker |  | 4497 | 124 | python names src/requests/auth.py |  |  | 0.567 |
| walker |  | 4519 | 22 | python decl src/requests/auth.py:78 |  |  | 0.567 |
| walker |  | 4541 | 22 | python decl src/requests/auth.py:116 |  |  | 0.567 |
| walker |  | 4554 | 13 | python doc src/requests/auth.py:78 |  |  | 0.567 |
| walker |  | 4569 | 15 | python doc src/requests/auth.py:116 |  |  | 0.567 |
| ns | 4680 |  | 215 | `adapters.py`: pool defaults + the `BaseAdapter` contract | 3.6 |  | 0.560 |
| walker |  | 4701 | 132 | python names src/requests/api.py |  |  | 0.574 |
| walker |  | 4736 | 35 | python decl src/requests/api.py:24 |  |  | 0.574 |
| walker |  | 4776 | 40 | python decl src/requests/api.py:74 |  |  | 0.574 |
| walker |  | 4816 | 40 | python decl src/requests/api.py:137 |  |  | 0.574 |
| walker |  | 4856 | 40 | python decl src/requests/api.py:154 |  |  | 0.574 |
| walker |  | 4920 | 64 | python decl src/requests/api.py:117 |  |  | 0.574 |
| ns | 5044 |  | 364 | `HTTPAdapter`: constructor knobs + complete method roster | 3.7 | 3.6 | 0.553 |
| walker |  | 5165 | 245 | python names src/requests/utils.py |  |  | 0.553 |
| walker |  | 5173 | 8 | python decl src/requests/utils.py:370 |  |  | 0.553 |
| walker |  | 5183 | 10 | python decl src/requests/utils.py:328 |  |  | 0.553 |
| walker |  | 5211 | 28 | python decl src/requests/utils.py:91 |  |  | 0.554 |
| walker |  | 5244 | 33 | python decl src/requests/utils.py:231 |  |  | 0.554 |
| walker |  | 5281 | 37 | python decl src/requests/utils.py:149 |  |  | 0.554 |
| ns | 5313 |  | 269 | `SessionRedirectMixin` + session-module merge helpers | 3.8 |  | 0.542 |
| walker |  | 5318 | 37 | python decl src/requests/utils.py:341 |  |  | 0.542 |
| walker |  | 5331 | 13 | python module doc tests/__init__.py |  |  | 0.542 |
| walker |  | 5486 | 155 | python names src/requests/compat.py |  |  | 0.542 |
| ns | 5549 |  | 236 | `cookies.py` module map: classes + free functions | 3.9 |  | 0.533 |
| walker |  | 5630 | 144 | python names src/requests/models.py |  |  | 0.539 |
| walker |  | 5689 | 59 | python decl src/requests/models.py:255 |  |  | 0.539 |
| walker |  | 5725 | 36 | python decl src/requests/models.py:258 |  |  | 0.539 |
| walker |  | 5799 | 74 | python decl src/requests/models.py:96 |  |  | 0.547 |
| ns | 5857 |  | 308 | `auth.py`: every auth handler and its methods | 3.10 |  | 0.542 |
| walker |  | 5923 | 124 | python decl src/requests/models.py:109 |  |  | 0.542 |
| walker |  | 5929 | 6 | python decl src/requests/models.py:112 |  |  | 0.542 |
| walker |  | 5944 | 15 | python decl src/requests/models.py:137 |  |  | 0.542 |
| walker |  | 5961 | 17 | python decl src/requests/models.py:133 |  |  | 0.542 |
| walker |  | 5978 | 17 | python decl src/requests/models.py:147 |  |  | 0.542 |
| walker |  | 6136 | 158 | python module doc src/requests/__version__.py |  |  | 0.598 |
| ns | 6229 |  | 372 | `_types.py`: public type-alias roster | 4.1 |  | 0.582 |
| walker |  | 6487 | 351 | python names src/requests/exceptions.py |  |  | 0.620 |
| walker |  | 6495 | 8 | python doc src/requests/exceptions.py:106 |  |  | 0.620 |
| ns | 6540 |  | 311 | `_types.py`: the `Unpack` kwargs TypedDicts | 4.2 | 4.1 | 0.605 |
| walker |  | 6543 | 48 | python decl src/requests/exceptions.py:42 |  |  | 0.605 |
| walker |  | 6552 | 9 | python doc src/requests/exceptions.py:38 |  |  | 0.605 |
| walker |  | 6561 | 9 | python doc src/requests/exceptions.py:66 |  |  | 0.605 |
| walker |  | 6617 | 56 | python decl src/requests/exceptions.py:20 |  |  | 0.605 |
| walker |  | 6626 | 9 | python doc src/requests/exceptions.py:70 |  |  | 0.605 |
| walker |  | 6635 | 9 | python doc src/requests/exceptions.py:74 |  |  | 0.605 |
| ns | 6739 |  | 199 | Environment-driven settings: the `merge_environment_settings` core | 4.3 | 2.6 | 0.596 |
| walker |  | 6797 | 162 | python names src/requests/_types.py |  |  | 0.597 |
| walker |  | 6826 | 29 | python decl src/requests/_types.py:27 |  |  | 0.597 |
| ns | 6846 |  | 107 | `utils.py` module-level constants | 5.1 |  | 0.601 |
| walker |  | 6856 | 30 | python decl src/requests/_types.py:32 |  |  | 0.601 |
| walker |  | 6871 | 15 | python doc src/requests/_types.py:42 |  |  | 0.601 |
| ns | 6994 |  | 148 | `utils.py` function roster 1/3: proxies shim, lengths, netrc, encoders | 5.2 |  | 0.598 |
| walker |  | 7028 | 157 | python decl src/requests/sessions.py:127 |  |  | 0.609 |
| walker |  | 7054 | 26 | python decl src/requests/sessions.py:309 |  |  | 0.609 |
| walker |  | 7080 | 26 | python decl src/requests/sessions.py:370 |  |  | 0.609 |
| walker |  | 7125 | 45 | python decl src/requests/sessions.py:334 |  |  | 0.609 |
| ns | 7161 |  | 167 | `utils.py` function roster 2/3: cookies, encoding detection, URI quoting, CIDR | 5.3 | 5.2 | 0.602 |
| walker |  | 7292 | 167 | python decl src/requests/auth.py:85 |  |  | 0.606 |
| walker |  | 7299 | 7 | python decl src/requests/auth.py:91 |  |  | 0.606 |
| walker |  | 7306 | 7 | python decl src/requests/auth.py:93 |  |  | 0.606 |
| ns | 7341 |  | 180 | `utils.py` function roster 3/3: proxy resolution, default headers, header validation | 5.4 | 5.3 | 0.598 |
| walker |  | 7474 | 168 | python decl src/requests/models.py:283 |  |  | 0.618 |
| walker |  | 7485 | 11 | python doc src/requests/adapters.py:153 |  |  | 0.619 |
| ns | 7602 |  | 261 | `structures.py`: `CaseInsensitiveDict` and `LookupDict` in full shape | 5.5 |  | 0.609 |
| walker |  | 7691 | 206 | python module doc src/requests/status_codes.py |  |  | 0.609 |
| ns | 7712 |  | 110 | `status_codes.py`: how `requests.codes` is built | 5.6 |  | 0.606 |
| walker |  | 7788 | 97 | python decl src/requests/adapters.py:128 |  |  | 0.620 |
| walker |  | 7803 | 15 | python body src/requests/api.py:90 |  |  | 0.620 |
| ns | 7954 |  | 242 | The five small modules: hooks, `_internal_utils`, compat, certs, packages | 5.7 |  | 0.615 |
| walker |  | 8081 | 278 | python names src/requests/cookies.py |  |  | 0.624 |
| walker |  | 8108 | 27 | python decl src/requests/cookies.py:135 |  |  | 0.624 |
| ns | 8131 |  | 177 | `help.py`: the bug-report payload | 5.8 |  | 0.618 |
| walker |  | 8141 | 33 | python decl src/requests/cookies.py:604 |  |  | 0.618 |
| walker |  | 8180 | 39 | python decl src/requests/cookies.py:164 |  |  | 0.618 |
| walker |  | 8231 | 51 | python decl src/requests/cookies.py:579 |  |  | 0.618 |
| walker |  | 8284 | 53 | python decl src/requests/cookies.py:114 |  |  | 0.618 |
| walker |  | 8340 | 56 | python decl src/requests/cookies.py:563 |  |  | 0.618 |
| walker |  | 8396 | 56 | python decl src/requests/cookies.py:571 |  |  | 0.618 |
| walker |  | 8405 | 9 | python doc src/requests/exceptions.py:78 |  |  | 0.618 |
| ns | 8464 |  | 333 | `resolve_redirects`: the rules the redirect loop enforces | 6.1 | 3.8 | 0.605 |
| walker |  | 8607 | 202 | python decl src/requests/structures.py:96 |  |  | 0.607 |
| walker |  | 8614 | 7 | python decl src/requests/structures.py:126 |  |  | 0.607 |
| walker |  | 8623 | 9 | python decl src/requests/structures.py:123 |  |  | 0.607 |
| walker |  | 8631 | 8 | python doc src/requests/structures.py:96 |  |  | 0.608 |
| ns | 8753 |  | 289 | `HTTPAdapter.send`: the urllib3 call and the `MaxRetryError` fan-out | 6.2 | 3.7 | 0.597 |
| walker |  | 8851 | 220 | python names src/requests/_internal_utils.py |  |  | 0.599 |
| walker |  | 8880 | 29 | python decl src/requests/_internal_utils.py:20 |  |  | 0.599 |
| ns | 8955 |  | 202 | `HTTPAdapter.send`: the remaining `except` arms and the return | 6.3 | 6.2 | 0.592 |
| ns | 9043 |  | 88 | Makefile: the remaining targets (coverage, publish, docs) | 7.1 | 1.8 | 0.596 |
| walker |  | 9090 | 210 | python decl src/requests/structures.py:20 |  |  | 0.610 |
| walker |  | 9144 | 54 | python decl src/requests/structures.py:49 |  |  | 0.610 |
| walker |  | 9157 | 13 | python doc src/requests/utils.py:149 |  |  | 0.610 |
| walker |  | 9169 | 12 | python doc src/requests/compat.py:37 |  |  | 0.611 |
| walker |  | 9239 | 70 | python decl src/requests/adapters.py:85 |  |  | 0.611 |
| ns | 9326 |  | 283 | pyproject: every remaining table, and the settings that change how you work here | 7.2 | 1.10 | 0.604 |
| walker |  | 9374 | 135 | python decl src/requests/sessions.py:186 |  |  | 0.606 |
| walker |  | 9383 | 9 | python doc src/requests/exceptions.py:142 |  |  | 0.606 |
| walker |  | 9398 | 15 | python body src/requests/api.py:171 |  |  | 0.606 |
| walker |  | 9439 | 41 | python decl src/requests/models.py:151 |  |  | 0.606 |
| ns | 9481 |  | 155 | `tests/conftest.py`: the fixture set every test builds on | 7.3 |  | 0.601 |
| ns | 9576 |  | 95 | `tests/test_requests.py`: top-level class roster | 7.4 |  | 0.598 |
| walker |  | 9715 | 276 | python decl src/requests/cookies.py:31 |  |  | 0.598 |
| walker |  | 9723 | 8 | python decl src/requests/cookies.py:101 |  |  | 0.598 |
| walker |  | 9731 | 8 | python decl src/requests/cookies.py:105 |  |  | 0.598 |
| walker |  | 9739 | 8 | python decl src/requests/cookies.py:109 |  |  | 0.598 |
| ns | 9758 |  | 182 | `tests/testserver/server.py`: the local socket server API (complete) | 7.5 |  | 0.592 |
| walker |  | 9782 | 43 | python decl src/requests/models.py:183 |  |  | 0.592 |
| walker |  | 9904 | 122 | python body src/requests/__init__.py:99 |  |  | 0.592 |
| ns | 9915 |  | 157 | `docs/api.rst`: every section of the developer interface | 7.6 |  | 0.585 |
| walker |  | 9950 | 46 | python decl src/requests/models.py:141 |  |  | 0.585 |
| walker |  | 9959 | 9 | python body src/requests/structures.py:73 |  |  | 0.585 |
| ns | 9963 |  | 48 | `HISTORY.md`: the changelog's head | 7.7 |  | 0.584 |
