Score(3000)=0.623 I=0.771 C=0.503 ns_rows≤3K=17/49 grid(1000/1442/2080/3000/4327/6240/9000)=0.692/0.639/0.635/0.623/0.579/0.625/0.599

| source | ns_cum | walker_cum | marginal | descriptor | id | predecessor | Score(B=cum) |
|:-------|-------:|-----------:|---------:|:-----------|:---|:------------|-------------:|
| ns | 47 |  | 47 | Identity lede: what the library is, and its version | 1.1 |  | 0.000 |
| walker |  | 74 | 74 | Fs::DirListing { dir: . } |  |  | 0.000 |
| walker |  | 77 | 3 | Fs::DirListing { dir: src } |  |  | 0.000 |
| walker |  | 111 | 34 | Fs::DirListing { dir: ext } |  |  | 0.000 |
| ns | 121 |  | 74 | Repository root listing (complete) | 1.2 |  | 0.671 |
| walker |  | 169 | 58 | Toml::Identity { file: pyproject.toml } |  |  | 0.672 |
| ns | 216 |  | 95 | `src/requests/` module roster (complete) | 1.3 |  | 0.456 |
| walker |  | 223 | 54 | Fs::DirListing { dir: docs } |  |  | 0.462 |
| walker |  | 227 | 4 | Fs::DirListing { dir: docs/_templates } |  |  | 0.462 |
| walker |  | 236 | 9 | Fs::DirListing { dir: docs/_static } |  |  | 0.462 |
| walker |  | 246 | 10 | Fs::DirListing { dir: docs/dev } |  |  | 0.464 |
| walker |  | 260 | 14 | Fs::DirListing { dir: docs/_themes } |  |  | 0.464 |
| walker |  | 281 | 21 | Fs::DirListing { dir: docs/user } |  |  | 0.468 |
| walker |  | 319 | 38 | Fs::DirListing { dir: docs/community } |  |  | 0.379 |
| ns | 319 |  | 103 | `tests/` roster (complete, incl. `testserver/` and `certs/`) | 1.4 |  | 0.379 |
| walker |  | 411 | 92 | Fs::DirListing { dir: src/requests } |  |  | 0.607 |
| ns | 466 |  | 147 | README: the canonical `requests.get(...)` doctest + every `##` heading | 1.5 | 1.1 | 0.541 |
| ns | 584 |  | 118 | Full package metadata block (`__version__.py`) | 1.6 | 1.1 | 0.507 |
| ns | 682 |  | 98 | CI: `.github/` and workflow file roster (complete) | 1.7 |  | 0.450 |
| ns | 762 |  | 80 | Makefile: install / test / CI targets | 1.8 |  | 0.431 |
| walker |  | 767 | 356 | Plaintext::Whole { file: Makefile } |  |  | 0.477 |
| ns | 885 |  | 123 | `docs/` tree listing (complete) | 1.9 |  | 0.523 |
| walker |  | 942 | 175 | Markdown::ReadmeHeadline { file: README.md } |  |  | 0.660 |
| walker |  | 982 | 40 | Markdown::HeadingsOutline { file: README.md } |  |  | 0.692 |
| ns | 1187 |  | 302 | pyproject: build backend, metadata, runtime dependencies | 1.10 |  | 0.630 |
| walker |  | 1292 | 310 | Code::CodeKey { rung: Names, file: src/requests/__init__.py, decl: 0, sub: 0, line: 0 } |  |  | 0.639 |
| walker |  | 1339 | 47 | Code::CodeKey { rung: Decl, file: src/requests/__init__.py, decl: 1, sub: 0, line: 60 } |  |  | 0.639 |
| walker |  | 1375 | 36 | Code::CodeKey { rung: Names, file: src/requests/help.py, decl: 0, sub: 0, line: 0 } |  |  | 0.639 |
| ns | 1458 |  | 271 | `requests/__init__.py`: name → module re-export map | 2.1 |  | 0.644 |
| ns | 1532 |  | 74 | `api.py`: names of all eight module-level functions | 2.2 |  | 0.630 |
| walker |  | 1725 | 350 | Code::CodeKey { rung: ModuleDoc, file: src/requests/__init__.py, decl: 0, sub: 0, line: 0 } |  |  | 0.630 |
| walker |  | 1866 | 141 | Toml::Dependencies { file: pyproject.toml } |  |  | 0.640 |
| ns | 1885 |  | 353 | `exceptions.py`: the complete class hierarchy | 2.3 |  | 0.599 |
| walker |  | 1909 | 43 | Fs::DirListing { dir: .github } |  |  | 0.612 |
| walker |  | 1950 | 41 | Fs::DirListing { dir: .github/workflows } |  |  | 0.646 |
| walker |  | 1964 | 14 | Fs::DirListing { dir: .github/ISSUE_TEMPLATE } |  |  | 0.663 |
| walker |  | 2014 | 50 | Code::CodeKey { rung: Names, file: src/requests/status_codes.py, decl: 0, sub: 0, line: 0 } |  |  | 0.663 |
| ns | 2061 |  | 176 | `Session` class declaration + docstring | 2.4 |  | 0.635 |
| walker |  | 2161 | 147 | Markdown::Section { file: README.md, section_index: 1, keeps_default_concavity: true } |  |  | 0.635 |
| walker |  | 2218 | 57 | Code::CodeKey { rung: Names, file: src/requests/hooks.py, decl: 0, sub: 0, line: 0 } |  |  | 0.635 |
| walker |  | 2271 | 53 | Code::CodeKey { rung: Decl, file: src/requests/hooks.py, decl: 3, sub: 0, line: 32 } |  |  | 0.635 |
| walker |  | 2328 | 57 | Code::CodeKey { rung: Names, file: src/requests/sessions.py, decl: 0, sub: 0, line: 0 } |  |  | 0.635 |
| ns | 2332 |  | 271 | `Session` attribute set: typed fields + `__attrs__` | 2.5 | 2.4 | 0.598 |
| walker |  | 2359 | 31 | Code::CodeKey { rung: Decl, file: src/requests/sessions.py, decl: 1, sub: 0, line: 76 } |  |  | 0.598 |
| walker |  | 2367 | 8 | Code::CodeKey { rung: Body, file: src/requests/sessions.py, decl: 31, sub: 0, line: 908 } |  |  | 0.598 |
| walker |  | 2420 | 53 | Code::CodeKey { rung: Decl, file: src/requests/sessions.py, decl: 2, sub: 0, line: 108 } |  |  | 0.598 |
| walker |  | 2483 | 63 | Markdown::Section { file: README.md, section_index: 2, keeps_default_concavity: false } |  |  | 0.598 |
| walker |  | 2556 | 73 | Code::CodeKey { rung: Names, file: src/requests/structures.py, decl: 0, sub: 0, line: 0 } |  |  | 0.598 |
| walker |  | 2569 | 13 | Code::CodeKey { rung: Doc, file: src/requests/help.py, decl: 2, sub: 0, line: 69 } |  |  | 0.598 |
| walker |  | 2583 | 14 | Code::CodeKey { rung: Doc, file: src/requests/help.py, decl: 3, sub: 0, line: 128 } |  |  | 0.598 |
| ns | 2620 |  | 288 | `Session` method roster (complete, with line coordinates) | 2.6 | 2.4 | 0.576 |
| walker |  | 2660 | 77 | Fs::DirListing { dir: tests } |  |  | 0.611 |
| walker |  | 2672 | 12 | Fs::DirListing { dir: tests/testserver } |  |  | 0.621 |
| walker |  | 2686 | 14 | Fs::DirListing { dir: tests/certs } |  |  | 0.644 |
| walker |  | 2775 | 89 | Code::CodeKey { rung: Names, file: src/requests/adapters.py, decl: 0, sub: 0, line: 0 } |  |  | 0.644 |
| walker |  | 2815 | 40 | Code::CodeKey { rung: Decl, file: src/requests/adapters.py, decl: 6, sub: 0, line: 122 } |  |  | 0.644 |
| walker |  | 2824 | 9 | Code::CodeKey { rung: Doc, file: src/requests/adapters.py, decl: 6, sub: 0, line: 122 } |  |  | 0.645 |
| walker |  | 2831 | 7 | Code::CodeKey { rung: Body, file: src/requests/adapters.py, decl: 7, sub: 0, line: 125 } |  |  | 0.645 |
| ns | 2871 |  | 251 | `Session.request`: the complete keyword signature | 2.7 | 2.6 | 0.623 |
| ns | 3054 |  | 183 | `Session.__init__`: every default value | 2.8 | 2.6 | 0.607 |
| walker |  | 3100 | 269 | Code::CodeKey { rung: Names, file: src/requests/__init__.py, decl: 0, sub: 1, line: 0 } |  |  | 0.626 |
| walker |  | 3225 | 125 | Toml::PackageMetadata { file: pyproject.toml } |  |  | 0.654 |
| walker |  | 3264 | 39 | Plaintext::Whole { file: docs/requirements.txt } |  |  | 0.654 |
| walker |  | 3281 | 17 | Code::CodeKey { rung: Body, file: src/requests/hooks.py, decl: 2, sub: 0, line: 25 } |  |  | 0.654 |
| walker |  | 3299 | 18 | Code::CodeKey { rung: Doc, file: src/requests/hooks.py, decl: 3, sub: 0, line: 32 } |  |  | 0.654 |
| walker |  | 3317 | 18 | Code::CodeKey { rung: Body, file: src/requests/help.py, decl: 3, sub: 0, line: 128 } |  |  | 0.654 |
| walker |  | 3327 | 10 | Code::CodeKey { rung: Body, file: src/requests/adapters.py, decl: 9, sub: 0, line: 153 } |  |  | 0.654 |
| ns | 3407 |  | 353 | `models.py` module map: constants, all five classes, `Request` fields | 3.1 |  | 0.626 |
| walker |  | 3453 | 126 | Code::CodeKey { rung: Names, file: src/requests/auth.py, decl: 0, sub: 0, line: 0 } |  |  | 0.626 |
| walker |  | 3475 | 22 | Code::CodeKey { rung: Decl, file: src/requests/auth.py, decl: 4, sub: 0, line: 78 } |  |  | 0.626 |
| walker |  | 3497 | 22 | Code::CodeKey { rung: Decl, file: src/requests/auth.py, decl: 13, sub: 0, line: 116 } |  |  | 0.627 |
| walker |  | 3510 | 13 | Code::CodeKey { rung: Doc, file: src/requests/auth.py, decl: 4, sub: 0, line: 78 } |  |  | 0.627 |
| walker |  | 3525 | 15 | Code::CodeKey { rung: Doc, file: src/requests/auth.py, decl: 13, sub: 0, line: 116 } |  |  | 0.627 |
| walker |  | 3659 | 134 | Code::CodeKey { rung: Names, file: src/requests/api.py, decl: 0, sub: 0, line: 0 } |  |  | 0.643 |
| walker |  | 3694 | 35 | Code::CodeKey { rung: Decl, file: src/requests/api.py, decl: 1, sub: 0, line: 24 } |  |  | 0.643 |
| ns | 3728 |  | 321 | `PreparedRequest`: attributes + every `prepare_*` step | 3.2 | 3.1 | 0.622 |
| walker |  | 3734 | 40 | Code::CodeKey { rung: Decl, file: src/requests/api.py, decl: 2, sub: 0, line: 74 } |  |  | 0.622 |
| walker |  | 3774 | 40 | Code::CodeKey { rung: Decl, file: src/requests/api.py, decl: 6, sub: 0, line: 137 } |  |  | 0.622 |
| walker |  | 3814 | 40 | Code::CodeKey { rung: Decl, file: src/requests/api.py, decl: 7, sub: 0, line: 154 } |  |  | 0.622 |
| walker |  | 3878 | 64 | Code::CodeKey { rung: Decl, file: src/requests/api.py, decl: 5, sub: 0, line: 117 } |  |  | 0.622 |
| walker |  | 3891 | 13 | Code::CodeKey { rung: ModuleDoc, file: tests/__init__.py, decl: 0, sub: 0, line: 0 } |  |  | 0.622 |
| ns | 4031 |  | 303 | `Response` attribute set: typed fields + `__attrs__` | 3.3 | 3.1 | 0.594 |
| walker |  | 4138 | 247 | Code::CodeKey { rung: Names, file: src/requests/utils.py, decl: 0, sub: 0, line: 0 } |  |  | 0.595 |
| walker |  | 4146 | 8 | Code::CodeKey { rung: Decl, file: src/requests/utils.py, decl: 14, sub: 0, line: 370 } |  |  | 0.595 |
| walker |  | 4156 | 10 | Code::CodeKey { rung: Decl, file: src/requests/utils.py, decl: 12, sub: 0, line: 328 } |  |  | 0.595 |
| walker |  | 4184 | 28 | Code::CodeKey { rung: Decl, file: src/requests/utils.py, decl: 6, sub: 0, line: 91 } |  |  | 0.595 |
| walker |  | 4217 | 33 | Code::CodeKey { rung: Decl, file: src/requests/utils.py, decl: 9, sub: 0, line: 231 } |  |  | 0.595 |
| walker |  | 4254 | 37 | Code::CodeKey { rung: Decl, file: src/requests/utils.py, decl: 7, sub: 0, line: 149 } |  |  | 0.595 |
| walker |  | 4291 | 37 | Code::CodeKey { rung: Decl, file: src/requests/utils.py, decl: 13, sub: 0, line: 341 } |  |  | 0.595 |
| ns | 4313 |  | 282 | `Response`: `__init__`, dunders and every `@property` | 3.4 | 3.3 | 0.579 |
| walker |  | 4448 | 157 | Code::CodeKey { rung: Names, file: src/requests/compat.py, decl: 0, sub: 0, line: 0 } |  |  | 0.580 |
| ns | 4465 |  | 152 | `Response`: the remaining members (`iter_content` … `close`) | 3.5 | 3.4 | 0.571 |
| walker |  | 4594 | 146 | Code::CodeKey { rung: Names, file: src/requests/models.py, decl: 0, sub: 0, line: 0 } |  |  | 0.577 |
| walker |  | 4653 | 59 | Code::CodeKey { rung: Decl, file: src/requests/models.py, decl: 13, sub: 0, line: 255 } |  |  | 0.577 |
| ns | 4680 |  | 215 | `adapters.py`: pool defaults + the `BaseAdapter` contract | 3.6 |  | 0.570 |
| walker |  | 4689 | 36 | Code::CodeKey { rung: Decl, file: src/requests/models.py, decl: 14, sub: 0, line: 258 } |  |  | 0.570 |
| walker |  | 4763 | 74 | Code::CodeKey { rung: Decl, file: src/requests/models.py, decl: 1, sub: 0, line: 96 } |  |  | 0.578 |
| walker |  | 4887 | 124 | Code::CodeKey { rung: Decl, file: src/requests/models.py, decl: 5, sub: 0, line: 109 } |  |  | 0.578 |
| walker |  | 4893 | 6 | Code::CodeKey { rung: Decl, file: src/requests/models.py, decl: 6, sub: 0, line: 112 } |  |  | 0.578 |
| walker |  | 4908 | 15 | Code::CodeKey { rung: Decl, file: src/requests/models.py, decl: 8, sub: 0, line: 137 } |  |  | 0.578 |
| walker |  | 4925 | 17 | Code::CodeKey { rung: Decl, file: src/requests/models.py, decl: 7, sub: 0, line: 133 } |  |  | 0.578 |
| walker |  | 4942 | 17 | Code::CodeKey { rung: Decl, file: src/requests/models.py, decl: 10, sub: 0, line: 147 } |  |  | 0.578 |
| ns | 5044 |  | 364 | `HTTPAdapter`: constructor knobs + complete method roster | 3.7 | 3.6 | 0.558 |
| walker |  | 5100 | 158 | Code::CodeKey { rung: ModuleDoc, file: src/requests/__version__.py, decl: 0, sub: 0, line: 0 } |  |  | 0.617 |
| ns | 5313 |  | 269 | `SessionRedirectMixin` + session-module merge helpers | 3.8 |  | 0.604 |
| walker |  | 5453 | 353 | Code::CodeKey { rung: Names, file: src/requests/exceptions.py, decl: 0, sub: 0, line: 0 } |  |  | 0.645 |
| walker |  | 5461 | 8 | Code::CodeKey { rung: Doc, file: src/requests/exceptions.py, decl: 15, sub: 0, line: 106 } |  |  | 0.645 |
| walker |  | 5509 | 48 | Code::CodeKey { rung: Decl, file: src/requests/exceptions.py, decl: 4, sub: 0, line: 42 } |  |  | 0.645 |
| walker |  | 5518 | 9 | Code::CodeKey { rung: Doc, file: src/requests/exceptions.py, decl: 3, sub: 0, line: 38 } |  |  | 0.645 |
| walker |  | 5527 | 9 | Code::CodeKey { rung: Doc, file: src/requests/exceptions.py, decl: 7, sub: 0, line: 66 } |  |  | 0.645 |
| ns | 5549 |  | 236 | `cookies.py` module map: classes + free functions | 3.9 |  | 0.634 |
| walker |  | 5583 | 56 | Code::CodeKey { rung: Decl, file: src/requests/exceptions.py, decl: 1, sub: 0, line: 20 } |  |  | 0.634 |
| walker |  | 5592 | 9 | Code::CodeKey { rung: Doc, file: src/requests/exceptions.py, decl: 8, sub: 0, line: 70 } |  |  | 0.634 |
| walker |  | 5601 | 9 | Code::CodeKey { rung: Doc, file: src/requests/exceptions.py, decl: 9, sub: 0, line: 74 } |  |  | 0.634 |
| walker |  | 5765 | 164 | Code::CodeKey { rung: Names, file: src/requests/_types.py, decl: 0, sub: 0, line: 0 } |  |  | 0.635 |
| walker |  | 5794 | 29 | Code::CodeKey { rung: Decl, file: src/requests/_types.py, decl: 4, sub: 0, line: 27 } |  |  | 0.635 |
| walker |  | 5824 | 30 | Code::CodeKey { rung: Decl, file: src/requests/_types.py, decl: 6, sub: 0, line: 32 } |  |  | 0.635 |
| walker |  | 5839 | 15 | Code::CodeKey { rung: Doc, file: src/requests/_types.py, decl: 10, sub: 0, line: 42 } |  |  | 0.635 |
| ns | 5857 |  | 308 | `auth.py`: every auth handler and its methods | 3.10 |  | 0.627 |
| walker |  | 5996 | 157 | Code::CodeKey { rung: Decl, file: src/requests/sessions.py, decl: 3, sub: 0, line: 127 } |  |  | 0.640 |
| walker |  | 6022 | 26 | Code::CodeKey { rung: Decl, file: src/requests/sessions.py, decl: 8, sub: 0, line: 309 } |  |  | 0.640 |
| walker |  | 6048 | 26 | Code::CodeKey { rung: Decl, file: src/requests/sessions.py, decl: 10, sub: 0, line: 370 } |  |  | 0.640 |
| walker |  | 6093 | 45 | Code::CodeKey { rung: Decl, file: src/requests/sessions.py, decl: 9, sub: 0, line: 334 } |  |  | 0.640 |
| ns | 6229 |  | 372 | `_types.py`: public type-alias roster | 4.1 |  | 0.625 |
| walker |  | 6260 | 167 | Code::CodeKey { rung: Decl, file: src/requests/auth.py, decl: 6, sub: 0, line: 85 } |  |  | 0.629 |
| walker |  | 6267 | 7 | Code::CodeKey { rung: Decl, file: src/requests/auth.py, decl: 7, sub: 0, line: 91 } |  |  | 0.629 |
| walker |  | 6274 | 7 | Code::CodeKey { rung: Decl, file: src/requests/auth.py, decl: 8, sub: 0, line: 93 } |  |  | 0.629 |
| walker |  | 6442 | 168 | Code::CodeKey { rung: Decl, file: src/requests/models.py, decl: 16, sub: 0, line: 283 } |  |  | 0.651 |
| walker |  | 6515 | 73 | Plaintext::DeclSurface { file: requirements-dev.txt } |  |  | 0.651 |
| walker |  | 6526 | 11 | Code::CodeKey { rung: Doc, file: src/requests/adapters.py, decl: 9, sub: 0, line: 153 } |  |  | 0.653 |
| ns | 6540 |  | 311 | `_types.py`: the `Unpack` kwargs TypedDicts | 4.2 | 4.1 | 0.637 |
| walker |  | 6623 | 97 | Code::CodeKey { rung: Decl, file: src/requests/adapters.py, decl: 8, sub: 0, line: 128 } |  |  | 0.653 |
| walker |  | 6638 | 15 | Code::CodeKey { rung: Body, file: src/requests/api.py, decl: 3, sub: 0, line: 90 } |  |  | 0.653 |
| ns | 6739 |  | 199 | Environment-driven settings: the `merge_environment_settings` core | 4.3 | 2.6 | 0.643 |
| ns | 6846 |  | 107 | `utils.py` module-level constants | 5.1 |  | 0.647 |
| walker |  | 6918 | 280 | Code::CodeKey { rung: Names, file: src/requests/cookies.py, decl: 0, sub: 0, line: 0 } |  |  | 0.657 |
| walker |  | 6945 | 27 | Code::CodeKey { rung: Decl, file: src/requests/cookies.py, decl: 20, sub: 0, line: 135 } |  |  | 0.657 |
| walker |  | 6978 | 33 | Code::CodeKey { rung: Decl, file: src/requests/cookies.py, decl: 57, sub: 0, line: 604 } |  |  | 0.657 |
| ns | 6994 |  | 148 | `utils.py` function roster 1/3: proxies shim, lengths, netrc, encoders | 5.2 |  | 0.653 |
| walker |  | 7017 | 39 | Code::CodeKey { rung: Decl, file: src/requests/cookies.py, decl: 22, sub: 0, line: 164 } |  |  | 0.653 |
| walker |  | 7068 | 51 | Code::CodeKey { rung: Decl, file: src/requests/cookies.py, decl: 56, sub: 0, line: 579 } |  |  | 0.653 |
| walker |  | 7121 | 53 | Code::CodeKey { rung: Decl, file: src/requests/cookies.py, decl: 16, sub: 0, line: 114 } |  |  | 0.653 |
| ns | 7161 |  | 167 | `utils.py` function roster 2/3: cookies, encoding detection, URI quoting, CIDR | 5.3 | 5.2 | 0.645 |
| walker |  | 7177 | 56 | Code::CodeKey { rung: Decl, file: src/requests/cookies.py, decl: 54, sub: 0, line: 563 } |  |  | 0.645 |
| walker |  | 7233 | 56 | Code::CodeKey { rung: Decl, file: src/requests/cookies.py, decl: 55, sub: 0, line: 571 } |  |  | 0.645 |
| walker |  | 7242 | 9 | Code::CodeKey { rung: Doc, file: src/requests/exceptions.py, decl: 10, sub: 0, line: 78 } |  |  | 0.645 |
| ns | 7341 |  | 180 | `utils.py` function roster 3/3: proxy resolution, default headers, header validation | 5.4 | 5.3 | 0.636 |
| walker |  | 7444 | 202 | Code::CodeKey { rung: Decl, file: src/requests/structures.py, decl: 14, sub: 0, line: 96 } |  |  | 0.637 |
| walker |  | 7451 | 7 | Code::CodeKey { rung: Decl, file: src/requests/structures.py, decl: 20, sub: 0, line: 126 } |  |  | 0.637 |
| walker |  | 7460 | 9 | Code::CodeKey { rung: Decl, file: src/requests/structures.py, decl: 19, sub: 0, line: 123 } |  |  | 0.637 |
| walker |  | 7468 | 8 | Code::CodeKey { rung: Doc, file: src/requests/structures.py, decl: 14, sub: 0, line: 96 } |  |  | 0.637 |
| ns | 7602 |  | 261 | `structures.py`: `CaseInsensitiveDict` and `LookupDict` in full shape | 5.5 |  | 0.629 |
| walker |  | 7678 | 210 | Code::CodeKey { rung: Decl, file: src/requests/structures.py, decl: 3, sub: 0, line: 20 } |  |  | 0.644 |
| ns | 7712 |  | 110 | `status_codes.py`: how `requests.codes` is built | 5.6 |  | 0.640 |
| walker |  | 7732 | 54 | Code::CodeKey { rung: Decl, file: src/requests/structures.py, decl: 4, sub: 0, line: 49 } |  |  | 0.640 |
| walker |  | 7954 | 222 | Code::CodeKey { rung: Names, file: src/requests/_internal_utils.py, decl: 0, sub: 0, line: 0 } |  |  | 0.635 |
| ns | 7954 |  | 242 | The five small modules: hooks, `_internal_utils`, compat, certs, packages | 5.7 |  | 0.635 |
| walker |  | 7983 | 29 | Code::CodeKey { rung: Decl, file: src/requests/_internal_utils.py, decl: 7, sub: 0, line: 20 } |  |  | 0.635 |
| walker |  | 7996 | 13 | Code::CodeKey { rung: Doc, file: src/requests/utils.py, decl: 7, sub: 0, line: 149 } |  |  | 0.635 |
| walker |  | 8008 | 12 | Code::CodeKey { rung: Doc, file: src/requests/compat.py, decl: 1, sub: 0, line: 37 } |  |  | 0.637 |
| walker |  | 8078 | 70 | Code::CodeKey { rung: Decl, file: src/requests/adapters.py, decl: 5, sub: 0, line: 85 } |  |  | 0.637 |
| ns | 8131 |  | 177 | `help.py`: the bug-report payload | 5.8 |  | 0.630 |
| walker |  | 8213 | 135 | Code::CodeKey { rung: Decl, file: src/requests/sessions.py, decl: 7, sub: 0, line: 186 } |  |  | 0.632 |
| walker |  | 8222 | 9 | Code::CodeKey { rung: Doc, file: src/requests/exceptions.py, decl: 24, sub: 0, line: 142 } |  |  | 0.632 |
| walker |  | 8237 | 15 | Code::CodeKey { rung: Body, file: src/requests/api.py, decl: 8, sub: 0, line: 171 } |  |  | 0.632 |
| walker |  | 8278 | 41 | Code::CodeKey { rung: Decl, file: src/requests/models.py, decl: 11, sub: 0, line: 151 } |  |  | 0.632 |
| walker |  | 8388 | 110 | Plaintext::DeclSurface { file: tox.ini } |  |  | 0.632 |
| ns | 8464 |  | 333 | `resolve_redirects`: the rules the redirect loop enforces | 6.1 | 3.8 | 0.619 |
| walker |  | 8664 | 276 | Code::CodeKey { rung: Decl, file: src/requests/cookies.py, decl: 1, sub: 0, line: 31 } |  |  | 0.619 |
| walker |  | 8672 | 8 | Code::CodeKey { rung: Decl, file: src/requests/cookies.py, decl: 13, sub: 0, line: 101 } |  |  | 0.619 |
| walker |  | 8680 | 8 | Code::CodeKey { rung: Decl, file: src/requests/cookies.py, decl: 14, sub: 0, line: 105 } |  |  | 0.619 |
| walker |  | 8688 | 8 | Code::CodeKey { rung: Decl, file: src/requests/cookies.py, decl: 15, sub: 0, line: 109 } |  |  | 0.619 |
| walker |  | 8731 | 43 | Code::CodeKey { rung: Decl, file: src/requests/models.py, decl: 12, sub: 0, line: 183 } |  |  | 0.619 |
| ns | 8753 |  | 289 | `HTTPAdapter.send`: the urllib3 call and the `MaxRetryError` fan-out | 6.2 | 3.7 | 0.607 |
| walker |  | 8853 | 122 | Code::CodeKey { rung: Body, file: src/requests/__init__.py, decl: 2, sub: 0, line: 99 } |  |  | 0.607 |
| walker |  | 8899 | 46 | Code::CodeKey { rung: Decl, file: src/requests/models.py, decl: 9, sub: 0, line: 141 } |  |  | 0.607 |
| walker |  | 8908 | 9 | Code::CodeKey { rung: Body, file: src/requests/structures.py, decl: 9, sub: 0, line: 73 } |  |  | 0.607 |
| ns | 8955 |  | 202 | `HTTPAdapter.send`: the remaining `except` arms and the return | 6.3 | 6.2 | 0.599 |
| walker |  | 8958 | 50 | Code::CodeKey { rung: Body, file: src/requests/_types.py, decl: 10, sub: 0, line: 42 } |  |  | 0.599 |
| ns | 9043 |  | 88 | Makefile: the remaining targets (coverage, publish, docs) | 7.1 | 1.8 | 0.604 |
| walker |  | 9280 | 322 | Code::CodeKey { rung: Decl, file: src/requests/models.py, decl: 20, sub: 0, line: 376 } |  |  | 0.624 |
| walker |  | 9288 | 8 | Code::CodeKey { rung: Decl, file: src/requests/models.py, decl: 26, sub: 0, line: 471 } |  |  | 0.625 |
| walker |  | 9321 | 33 | Code::CodeKey { rung: Decl, file: src/requests/models.py, decl: 32, sub: 0, line: 697 } |  |  | 0.625 |
| ns | 9326 |  | 283 | pyproject: every remaining table, and the settings that change how you work here | 7.2 | 1.10 | 0.613 |
| walker |  | 9360 | 39 | Code::CodeKey { rung: Decl, file: src/requests/models.py, decl: 29, sub: 0, line: 574 } |  |  | 0.613 |
| walker |  | 9400 | 40 | Code::CodeKey { rung: Decl, file: src/requests/models.py, decl: 27, sub: 0, line: 481 } |  |  | 0.613 |
| walker |  | 9441 | 41 | Code::CodeKey { rung: Decl, file: src/requests/models.py, decl: 31, sub: 0, line: 668 } |  |  | 0.613 |
| ns | 9481 |  | 155 | `tests/conftest.py`: the fixture set every test builds on | 7.3 |  | 0.608 |
| walker |  | 9494 | 53 | Code::CodeKey { rung: Body, file: src/requests/_internal_utils.py, decl: 8, sub: 0, line: 26 } |  |  | 0.608 |
| ns | 9576 |  | 95 | `tests/test_requests.py`: top-level class roster | 7.4 |  | 0.605 |
| walker |  | 9665 | 171 | Code::CodeKey { rung: Decl, file: src/requests/models.py, decl: 17, sub: 0, line: 321 } |  |  | 0.605 |
| walker |  | 9721 | 56 | Code::CodeKey { rung: Doc, file: src/requests/_internal_utils.py, decl: 8, sub: 0, line: 26 } |  |  | 0.605 |
| walker |  | 9730 | 9 | Code::CodeKey { rung: Doc, file: src/requests/exceptions.py, decl: 26, sub: 0, line: 153 } |  |  | 0.605 |
| ns | 9758 |  | 182 | `tests/testserver/server.py`: the local socket server API (complete) | 7.5 |  | 0.599 |
| ns | 9915 |  | 157 | `docs/api.rst`: every section of the developer interface | 7.6 |  | 0.593 |
| ns | 9963 |  | 48 | `HISTORY.md`: the changelog's head | 7.7 |  | 0.591 |
