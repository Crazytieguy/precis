Score(3000)=0.565 I=0.715 C=0.446 ns_rows≤3K=17/49 grid(1000/1442/2080/3000/4327/6240/9000)=0.691/0.635/0.626/0.565/0.591/0.634/0.607

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
| walker |  | 884 | 175 | README headline in README.md |  |  | 0.619 |
| ns | 885 |  | 123 | `docs/` tree listing (complete) | 1.9 |  | 0.660 |
| walker |  | 924 | 40 | headings outline in README.md |  |  | 0.691 |
| ns | 1187 |  | 302 | pyproject: build backend, metadata, runtime dependencies | 1.10 |  | 0.627 |
| walker |  | 1234 | 310 | python names src/requests/__init__.py |  |  | 0.635 |
| walker |  | 1281 | 47 | python decl src/requests/__init__.py:60 |  |  | 0.635 |
| walker |  | 1317 | 36 | python names src/requests/help.py |  |  | 0.635 |
| ns | 1458 |  | 271 | `requests/__init__.py`: name → module re-export map | 2.1 |  | 0.641 |
| ns | 1532 |  | 74 | `api.py`: names of all eight module-level functions | 2.2 |  | 0.627 |
| walker |  | 1667 | 350 | python module doc src/requests/__init__.py |  |  | 0.627 |
| walker |  | 1710 | 43 | listing of '.github' |  |  | 0.641 |
| walker |  | 1751 | 41 | listing of '.github/workflows' |  |  | 0.677 |
| walker |  | 1809 | 58 | [package] in pyproject.toml |  |  | 0.680 |
| walker |  | 1859 | 50 | python names src/requests/status_codes.py |  |  | 0.680 |
| ns | 1885 |  | 353 | `exceptions.py`: the complete class hierarchy | 2.3 |  | 0.636 |
| walker |  | 2006 | 147 | README.md section #0 |  |  | 0.636 |
| walker |  | 2020 | 14 | listing of '.github/ISSUE_TEMPLATE' |  |  | 0.654 |
| ns | 2061 |  | 176 | `Session` class declaration + docstring | 2.4 |  | 0.626 |
| walker |  | 2097 | 77 | manifest config in pyproject.toml |  |  | 0.631 |
| walker |  | 2154 | 57 | python names src/requests/hooks.py |  |  | 0.631 |
| walker |  | 2207 | 53 | python decl src/requests/hooks.py:32 |  |  | 0.631 |
| walker |  | 2264 | 57 | python names src/requests/sessions.py |  |  | 0.631 |
| walker |  | 2295 | 31 | python decl src/requests/sessions.py:76 |  |  | 0.631 |
| walker |  | 2303 | 8 | python body src/requests/sessions.py:908 |  |  | 0.631 |
| ns | 2332 |  | 271 | `Session` attribute set: typed fields + `__attrs__` | 2.5 | 2.4 | 0.594 |
| walker |  | 2356 | 53 | python decl src/requests/sessions.py:108 |  |  | 0.594 |
| walker |  | 2438 | 82 | tool.setuptools config in pyproject.toml |  |  | 0.595 |
| walker |  | 2577 | 139 | [dependencies] in pyproject.toml |  |  | 0.607 |
| ns | 2620 |  | 288 | `Session` method roster (complete, with line coordinates) | 2.6 | 2.4 | 0.584 |
| walker |  | 2640 | 63 | README.md section #1 |  |  | 0.584 |
| walker |  | 2713 | 73 | python names src/requests/structures.py |  |  | 0.584 |
| walker |  | 2726 | 13 | python doc src/requests/help.py:69 |  |  | 0.584 |
| walker |  | 2740 | 14 | python doc src/requests/help.py:128 |  |  | 0.584 |
| walker |  | 2829 | 89 | python names src/requests/adapters.py |  |  | 0.584 |
| walker |  | 2869 | 40 | python decl src/requests/adapters.py:122 |  |  | 0.584 |
| ns | 2871 |  | 251 | `Session.request`: the complete keyword signature | 2.7 | 2.6 | 0.565 |
| walker |  | 2878 | 9 | python doc src/requests/adapters.py:122 |  |  | 0.565 |
| walker |  | 2885 | 7 | python body src/requests/adapters.py:125 |  |  | 0.565 |
| ns | 3054 |  | 183 | `Session.__init__`: every default value | 2.8 | 2.6 | 0.551 |
| walker |  | 3154 | 269 | python names src/requests/__init__.py #1 |  |  | 0.569 |
| walker |  | 3279 | 125 | package metadata in pyproject.toml |  |  | 0.604 |
| walker |  | 3356 | 77 | listing of 'tests' |  |  | 0.637 |
| walker |  | 3370 | 14 | listing of 'tests/certs' |  |  | 0.656 |
| walker |  | 3382 | 12 | listing of 'tests/testserver' |  |  | 0.668 |
| walker |  | 3399 | 17 | python body src/requests/hooks.py:25 |  |  | 0.668 |
| ns | 3407 |  | 353 | `models.py` module map: constants, all five classes, `Request` fields | 3.1 |  | 0.639 |
| walker |  | 3417 | 18 | python doc src/requests/hooks.py:32 |  |  | 0.639 |
| walker |  | 3435 | 18 | python body src/requests/help.py:128 |  |  | 0.639 |
| walker |  | 3445 | 10 | python body src/requests/adapters.py:153 |  |  | 0.639 |
| walker |  | 3571 | 126 | python names src/requests/auth.py |  |  | 0.639 |
| walker |  | 3593 | 22 | python decl src/requests/auth.py:78 |  |  | 0.639 |
| walker |  | 3615 | 22 | python decl src/requests/auth.py:116 |  |  | 0.639 |
| walker |  | 3628 | 13 | python doc src/requests/auth.py:78 |  |  | 0.639 |
| walker |  | 3643 | 15 | python doc src/requests/auth.py:116 |  |  | 0.639 |
| ns | 3728 |  | 321 | `PreparedRequest`: attributes + every `prepare_*` step | 3.2 | 3.1 | 0.618 |
| walker |  | 3777 | 134 | python names src/requests/api.py |  |  | 0.634 |
| walker |  | 3812 | 35 | python decl src/requests/api.py:24 |  |  | 0.634 |
| walker |  | 3852 | 40 | python decl src/requests/api.py:74 |  |  | 0.634 |
| walker |  | 3892 | 40 | python decl src/requests/api.py:137 |  |  | 0.634 |
| walker |  | 3932 | 40 | python decl src/requests/api.py:154 |  |  | 0.634 |
| walker |  | 3996 | 64 | python decl src/requests/api.py:117 |  |  | 0.634 |
| walker |  | 4009 | 13 | python module doc tests/__init__.py |  |  | 0.634 |
| ns | 4031 |  | 303 | `Response` attribute set: typed fields + `__attrs__` | 3.3 | 3.1 | 0.606 |
| walker |  | 4256 | 247 | python names src/requests/utils.py |  |  | 0.606 |
| walker |  | 4264 | 8 | python decl src/requests/utils.py:370 |  |  | 0.606 |
| walker |  | 4274 | 10 | python decl src/requests/utils.py:328 |  |  | 0.606 |
| walker |  | 4302 | 28 | python decl src/requests/utils.py:91 |  |  | 0.607 |
| ns | 4313 |  | 282 | `Response`: `__init__`, dunders and every `@property` | 3.4 | 3.3 | 0.591 |
| walker |  | 4335 | 33 | python decl src/requests/utils.py:231 |  |  | 0.591 |
| walker |  | 4372 | 37 | python decl src/requests/utils.py:149 |  |  | 0.591 |
| walker |  | 4409 | 37 | python decl src/requests/utils.py:341 |  |  | 0.591 |
| walker |  | 4448 | 39 | plaintext config docs/requirements.txt |  |  | 0.591 |
| ns | 4465 |  | 152 | `Response`: the remaining members (`iter_content` … `close`) | 3.5 | 3.4 | 0.582 |
| walker |  | 4605 | 157 | python names src/requests/compat.py |  |  | 0.582 |
| ns | 4680 |  | 215 | `adapters.py`: pool defaults + the `BaseAdapter` contract | 3.6 |  | 0.574 |
| walker |  | 4751 | 146 | python names src/requests/models.py |  |  | 0.581 |
| walker |  | 4810 | 59 | python decl src/requests/models.py:255 |  |  | 0.581 |
| walker |  | 4846 | 36 | python decl src/requests/models.py:258 |  |  | 0.581 |
| walker |  | 4920 | 74 | python decl src/requests/models.py:96 |  |  | 0.589 |
| walker |  | 5044 | 124 | python decl src/requests/models.py:109 |  |  | 0.568 |
| ns | 5044 |  | 364 | `HTTPAdapter`: constructor knobs + complete method roster | 3.7 | 3.6 | 0.568 |
| walker |  | 5050 | 6 | python decl src/requests/models.py:112 |  |  | 0.568 |
| walker |  | 5065 | 15 | python decl src/requests/models.py:137 |  |  | 0.568 |
| walker |  | 5082 | 17 | python decl src/requests/models.py:133 |  |  | 0.568 |
| walker |  | 5099 | 17 | python decl src/requests/models.py:147 |  |  | 0.568 |
| walker |  | 5257 | 158 | python module doc src/requests/__version__.py |  |  | 0.628 |
| ns | 5313 |  | 269 | `SessionRedirectMixin` + session-module merge helpers | 3.8 |  | 0.614 |
| ns | 5549 |  | 236 | `cookies.py` module map: classes + free functions | 3.9 |  | 0.604 |
| walker |  | 5610 | 353 | python names src/requests/exceptions.py |  |  | 0.644 |
| walker |  | 5618 | 8 | python doc src/requests/exceptions.py:106 |  |  | 0.644 |
| walker |  | 5666 | 48 | python decl src/requests/exceptions.py:42 |  |  | 0.644 |
| walker |  | 5675 | 9 | python doc src/requests/exceptions.py:38 |  |  | 0.644 |
| walker |  | 5684 | 9 | python doc src/requests/exceptions.py:66 |  |  | 0.644 |
| walker |  | 5740 | 56 | python decl src/requests/exceptions.py:20 |  |  | 0.644 |
| walker |  | 5749 | 9 | python doc src/requests/exceptions.py:70 |  |  | 0.644 |
| walker |  | 5758 | 9 | python doc src/requests/exceptions.py:74 |  |  | 0.644 |
| ns | 5857 |  | 308 | `auth.py`: every auth handler and its methods | 3.10 |  | 0.637 |
| walker |  | 5922 | 164 | python names src/requests/_types.py |  |  | 0.637 |
| walker |  | 5951 | 29 | python decl src/requests/_types.py:27 |  |  | 0.637 |
| walker |  | 5981 | 30 | python decl src/requests/_types.py:32 |  |  | 0.637 |
| walker |  | 5996 | 15 | python doc src/requests/_types.py:42 |  |  | 0.637 |
| walker |  | 6153 | 157 | python decl src/requests/sessions.py:127 |  |  | 0.649 |
| walker |  | 6179 | 26 | python decl src/requests/sessions.py:309 |  |  | 0.649 |
| walker |  | 6205 | 26 | python decl src/requests/sessions.py:370 |  |  | 0.649 |
| ns | 6229 |  | 372 | `_types.py`: public type-alias roster | 4.1 |  | 0.634 |
| walker |  | 6250 | 45 | python decl src/requests/sessions.py:334 |  |  | 0.634 |
| walker |  | 6417 | 167 | python decl src/requests/auth.py:85 |  |  | 0.639 |
| walker |  | 6424 | 7 | python decl src/requests/auth.py:91 |  |  | 0.639 |
| walker |  | 6431 | 7 | python decl src/requests/auth.py:93 |  |  | 0.639 |
| ns | 6540 |  | 311 | `_types.py`: the `Unpack` kwargs TypedDicts | 4.2 | 4.1 | 0.623 |
| walker |  | 6599 | 168 | python decl src/requests/models.py:283 |  |  | 0.644 |
| walker |  | 6610 | 11 | python doc src/requests/adapters.py:153 |  |  | 0.646 |
| walker |  | 6707 | 97 | python decl src/requests/adapters.py:128 |  |  | 0.662 |
| walker |  | 6722 | 15 | python body src/requests/api.py:90 |  |  | 0.662 |
| ns | 6739 |  | 199 | Environment-driven settings: the `merge_environment_settings` core | 4.3 | 2.6 | 0.652 |
| ns | 6846 |  | 107 | `utils.py` module-level constants | 5.1 |  | 0.655 |
| ns | 6994 |  | 148 | `utils.py` function roster 1/3: proxies shim, lengths, netrc, encoders | 5.2 |  | 0.651 |
| walker |  | 7002 | 280 | python names src/requests/cookies.py |  |  | 0.662 |
| walker |  | 7029 | 27 | python decl src/requests/cookies.py:135 |  |  | 0.662 |
| walker |  | 7062 | 33 | python decl src/requests/cookies.py:604 |  |  | 0.662 |
| walker |  | 7101 | 39 | python decl src/requests/cookies.py:164 |  |  | 0.662 |
| walker |  | 7152 | 51 | python decl src/requests/cookies.py:579 |  |  | 0.662 |
| ns | 7161 |  | 167 | `utils.py` function roster 2/3: cookies, encoding detection, URI quoting, CIDR | 5.3 | 5.2 | 0.653 |
| walker |  | 7205 | 53 | python decl src/requests/cookies.py:114 |  |  | 0.653 |
| walker |  | 7261 | 56 | python decl src/requests/cookies.py:563 |  |  | 0.653 |
| walker |  | 7317 | 56 | python decl src/requests/cookies.py:571 |  |  | 0.653 |
| walker |  | 7326 | 9 | python doc src/requests/exceptions.py:78 |  |  | 0.653 |
| ns | 7341 |  | 180 | `utils.py` function roster 3/3: proxy resolution, default headers, header validation | 5.4 | 5.3 | 0.645 |
| walker |  | 7528 | 202 | python decl src/requests/structures.py:96 |  |  | 0.645 |
| walker |  | 7535 | 7 | python decl src/requests/structures.py:126 |  |  | 0.645 |
| walker |  | 7544 | 9 | python decl src/requests/structures.py:123 |  |  | 0.645 |
| walker |  | 7552 | 8 | python doc src/requests/structures.py:96 |  |  | 0.645 |
| ns | 7602 |  | 261 | `structures.py`: `CaseInsensitiveDict` and `LookupDict` in full shape | 5.5 |  | 0.638 |
| ns | 7712 |  | 110 | `status_codes.py`: how `requests.codes` is built | 5.6 |  | 0.634 |
| walker |  | 7762 | 210 | python decl src/requests/structures.py:20 |  |  | 0.648 |
| walker |  | 7816 | 54 | python decl src/requests/structures.py:49 |  |  | 0.648 |
| ns | 7954 |  | 242 | The five small modules: hooks, `_internal_utils`, compat, certs, packages | 5.7 |  | 0.641 |
| walker |  | 8038 | 222 | python names src/requests/_internal_utils.py |  |  | 0.643 |
| walker |  | 8067 | 29 | python decl src/requests/_internal_utils.py:20 |  |  | 0.643 |
| walker |  | 8080 | 13 | python doc src/requests/utils.py:149 |  |  | 0.643 |
| walker |  | 8092 | 12 | python doc src/requests/compat.py:37 |  |  | 0.645 |
| ns | 8131 |  | 177 | `help.py`: the bug-report payload | 5.8 |  | 0.637 |
| walker |  | 8162 | 70 | python decl src/requests/adapters.py:85 |  |  | 0.637 |
| walker |  | 8297 | 135 | python decl src/requests/sessions.py:186 |  |  | 0.640 |
| walker |  | 8306 | 9 | python doc src/requests/exceptions.py:142 |  |  | 0.640 |
| walker |  | 8321 | 15 | python body src/requests/api.py:171 |  |  | 0.640 |
| walker |  | 8362 | 41 | python decl src/requests/models.py:151 |  |  | 0.640 |
| ns | 8464 |  | 333 | `resolve_redirects`: the rules the redirect loop enforces | 6.1 | 3.8 | 0.626 |
| walker |  | 8638 | 276 | python decl src/requests/cookies.py:31 |  |  | 0.626 |
| walker |  | 8646 | 8 | python decl src/requests/cookies.py:101 |  |  | 0.626 |
| walker |  | 8654 | 8 | python decl src/requests/cookies.py:105 |  |  | 0.626 |
| walker |  | 8662 | 8 | python decl src/requests/cookies.py:109 |  |  | 0.626 |
| walker |  | 8705 | 43 | python decl src/requests/models.py:183 |  |  | 0.626 |
| ns | 8753 |  | 289 | `HTTPAdapter.send`: the urllib3 call and the `MaxRetryError` fan-out | 6.2 | 3.7 | 0.615 |
| walker |  | 8827 | 122 | python body src/requests/__init__.py:99 |  |  | 0.615 |
| walker |  | 8873 | 46 | python decl src/requests/models.py:141 |  |  | 0.615 |
| walker |  | 8882 | 9 | python body src/requests/structures.py:73 |  |  | 0.615 |
| walker |  | 8932 | 50 | python body src/requests/_types.py:42 |  |  | 0.615 |
| ns | 8955 |  | 202 | `HTTPAdapter.send`: the remaining `except` arms and the return | 6.3 | 6.2 | 0.607 |
| ns | 9043 |  | 88 | Makefile: the remaining targets (coverage, publish, docs) | 7.1 | 1.8 | 0.611 |
| walker |  | 9254 | 322 | python decl src/requests/models.py:376 |  |  | 0.631 |
| walker |  | 9262 | 8 | python decl src/requests/models.py:471 |  |  | 0.632 |
| walker |  | 9295 | 33 | python decl src/requests/models.py:697 |  |  | 0.632 |
| ns | 9326 |  | 283 | pyproject: every remaining table, and the settings that change how you work here | 7.2 | 1.10 | 0.625 |
| walker |  | 9334 | 39 | python decl src/requests/models.py:574 |  |  | 0.625 |
| walker |  | 9374 | 40 | python decl src/requests/models.py:481 |  |  | 0.625 |
| walker |  | 9415 | 41 | python decl src/requests/models.py:668 |  |  | 0.625 |
| walker |  | 9468 | 53 | python body src/requests/_internal_utils.py:26 |  |  | 0.625 |
| ns | 9481 |  | 155 | `tests/conftest.py`: the fixture set every test builds on | 7.3 |  | 0.619 |
| ns | 9576 |  | 95 | `tests/test_requests.py`: top-level class roster | 7.4 |  | 0.616 |
| walker |  | 9639 | 171 | python decl src/requests/models.py:321 |  |  | 0.616 |
| walker |  | 9695 | 56 | python doc src/requests/_internal_utils.py:26 |  |  | 0.616 |
| walker |  | 9704 | 9 | python doc src/requests/exceptions.py:153 |  |  | 0.616 |
| ns | 9758 |  | 182 | `tests/testserver/server.py`: the local socket server API (complete) | 7.5 |  | 0.610 |
| ns | 9915 |  | 157 | `docs/api.rst`: every section of the developer interface | 7.6 |  | 0.604 |
| ns | 9963 |  | 48 | `HISTORY.md`: the changelog's head | 7.7 |  | 0.602 |
