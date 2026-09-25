Score(3000)=0.715 I=0.908 C=0.563 ns_rows≤3K=17/49 grid(1000/1442/2080/3000/4327/6240/9000)=0.692/0.813/0.740/0.715/0.603/0.613/0.595

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
| walker |  | 1140 | 158 | Code::CodeKey { rung: ModuleDoc, file: src/requests/__version__.py, decl: 0, sub: 0, line: 0 } |  |  | 0.801 |
| ns | 1187 |  | 302 | pyproject: build backend, metadata, runtime dependencies | 1.10 |  | 0.729 |
| walker |  | 1203 | 63 | Markdown::Section { file: README.md, section_index: 2, keeps_default_concavity: false } |  |  | 0.729 |
| walker |  | 1246 | 43 | Fs::DirListing { dir: .github } |  |  | 0.746 |
| walker |  | 1287 | 41 | Fs::DirListing { dir: .github/workflows } |  |  | 0.790 |
| walker |  | 1301 | 14 | Fs::DirListing { dir: .github/ISSUE_TEMPLATE } |  |  | 0.813 |
| ns | 1458 |  | 271 | `requests/__init__.py`: name → module re-export map | 2.1 |  | 0.741 |
| ns | 1532 |  | 74 | `api.py`: names of all eight module-level functions | 2.2 |  | 0.724 |
| walker |  | 1611 | 310 | Code::CodeKey { rung: Names, file: src/requests/__init__.py, decl: 0, sub: 0, line: 0 } |  |  | 0.791 |
| walker |  | 1658 | 47 | Code::CodeKey { rung: Decl, file: src/requests/__init__.py, decl: 1, sub: 0, line: 60 } |  |  | 0.791 |
| ns | 1885 |  | 353 | `exceptions.py`: the complete class hierarchy | 2.3 |  | 0.740 |
| walker |  | 1927 | 269 | Code::CodeKey { rung: Names, file: src/requests/__init__.py, decl: 0, sub: 1, line: 0 } |  |  | 0.764 |
| ns | 2061 |  | 176 | `Session` class declaration + docstring | 2.4 |  | 0.731 |
| walker |  | 2068 | 141 | Toml::Dependencies { file: pyproject.toml } |  |  | 0.740 |
| ns | 2332 |  | 271 | `Session` attribute set: typed fields + `__attrs__` | 2.5 | 2.4 | 0.697 |
| walker |  | 2418 | 350 | Code::CodeKey { rung: ModuleDoc, file: src/requests/__init__.py, decl: 0, sub: 0, line: 0 } |  |  | 0.697 |
| walker |  | 2495 | 77 | Fs::DirListing { dir: tests } |  |  | 0.734 |
| walker |  | 2507 | 12 | Fs::DirListing { dir: tests/testserver } |  |  | 0.745 |
| walker |  | 2521 | 14 | Fs::DirListing { dir: tests/certs } |  |  | 0.769 |
| ns | 2620 |  | 288 | `Session` method roster (complete, with line coordinates) | 2.6 | 2.4 | 0.740 |
| walker |  | 2668 | 147 | Markdown::Section { file: README.md, section_index: 1, keeps_default_concavity: false } |  |  | 0.740 |
| walker |  | 2790 | 122 | Code::CodeKey { rung: Body, file: src/requests/__init__.py, decl: 2, sub: 0, line: 99 } |  |  | 0.740 |
| walker |  | 2863 | 73 | Plaintext::DeclSurface { file: requirements-dev.txt } |  |  | 0.740 |
| ns | 2871 |  | 251 | `Session.request`: the complete keyword signature | 2.7 | 2.6 | 0.715 |
| walker |  | 3039 | 176 | Markdown::Section { file: README.md, section_index: 3, keeps_default_concavity: false } |  |  | 0.715 |
| ns | 3054 |  | 183 | `Session.__init__`: every default value | 2.8 | 2.6 | 0.697 |
| walker |  | 3078 | 39 | Plaintext::DeclSurface { file: docs/requirements.txt } |  |  | 0.697 |
| walker |  | 3188 | 110 | Plaintext::DeclSurface { file: tox.ini } |  |  | 0.697 |
| walker |  | 3246 | 58 | Plaintext::Whole { file: tox.ini } |  |  | 0.697 |
| walker |  | 3259 | 13 | Code::CodeKey { rung: ModuleDoc, file: tests/__init__.py, decl: 0, sub: 0, line: 0 } |  |  | 0.697 |
| walker |  | 3316 | 57 | Code::CodeKey { rung: Names, file: src/requests/sessions.py, decl: 0, sub: 0, line: 0 } |  |  | 0.697 |
| walker |  | 3347 | 31 | Code::CodeKey { rung: Decl, file: src/requests/sessions.py, decl: 1, sub: 0, line: 76 } |  |  | 0.697 |
| walker |  | 3355 | 8 | Code::CodeKey { rung: Body, file: src/requests/sessions.py, decl: 31, sub: 0, line: 908 } |  |  | 0.697 |
| ns | 3407 |  | 353 | `models.py` module map: constants, all five classes, `Request` fields | 3.1 |  | 0.667 |
| walker |  | 3408 | 53 | Code::CodeKey { rung: Decl, file: src/requests/sessions.py, decl: 2, sub: 0, line: 108 } |  |  | 0.667 |
| walker |  | 3565 | 157 | Code::CodeKey { rung: Decl, file: src/requests/sessions.py, decl: 3, sub: 0, line: 127 } |  |  | 0.668 |
| walker |  | 3591 | 26 | Code::CodeKey { rung: Decl, file: src/requests/sessions.py, decl: 8, sub: 0, line: 309 } |  |  | 0.668 |
| walker |  | 3617 | 26 | Code::CodeKey { rung: Decl, file: src/requests/sessions.py, decl: 10, sub: 0, line: 370 } |  |  | 0.668 |
| walker |  | 3662 | 45 | Code::CodeKey { rung: Decl, file: src/requests/sessions.py, decl: 9, sub: 0, line: 334 } |  |  | 0.668 |
| walker |  | 3680 | 18 | Code::CodeKey { rung: Doc, file: src/requests/sessions.py, decl: 6, sub: 0, line: 154 } |  |  | 0.668 |
| walker |  | 3700 | 20 | Code::CodeKey { rung: Doc, file: src/requests/sessions.py, decl: 5, sub: 0, line: 134 } |  |  | 0.668 |
| ns | 3728 |  | 321 | `PreparedRequest`: attributes + every `prepare_*` step | 3.2 | 3.1 | 0.647 |
| walker |  | 3835 | 135 | Code::CodeKey { rung: Decl, file: src/requests/sessions.py, decl: 7, sub: 0, line: 186 } |  |  | 0.647 |
| walker |  | 3854 | 19 | Code::CodeKey { rung: Doc, file: src/requests/sessions.py, decl: 7, sub: 0, line: 186 } |  |  | 0.647 |
| walker |  | 3893 | 39 | Code::CodeKey { rung: Doc, file: src/requests/sessions.py, decl: 10, sub: 0, line: 370 } |  |  | 0.647 |
| walker |  | 3929 | 36 | Code::CodeKey { rung: Names, file: src/requests/help.py, decl: 0, sub: 0, line: 0 } |  |  | 0.647 |
| walker |  | 3942 | 13 | Code::CodeKey { rung: Doc, file: src/requests/help.py, decl: 2, sub: 0, line: 69 } |  |  | 0.647 |
| walker |  | 3956 | 14 | Code::CodeKey { rung: Doc, file: src/requests/help.py, decl: 3, sub: 0, line: 128 } |  |  | 0.647 |
| walker |  | 3974 | 18 | Code::CodeKey { rung: Body, file: src/requests/help.py, decl: 3, sub: 0, line: 128 } |  |  | 0.647 |
| walker |  | 4030 | 56 | Code::CodeKey { rung: Doc, file: src/requests/sessions.py, decl: 8, sub: 0, line: 309 } |  |  | 0.647 |
| ns | 4031 |  | 303 | `Response` attribute set: typed fields + `__attrs__` | 3.3 | 3.1 | 0.618 |
| walker |  | 4087 | 57 | Code::CodeKey { rung: Doc, file: src/requests/sessions.py, decl: 2, sub: 0, line: 108 } |  |  | 0.618 |
| walker |  | 4176 | 89 | Code::CodeKey { rung: Names, file: src/requests/adapters.py, decl: 0, sub: 0, line: 0 } |  |  | 0.618 |
| walker |  | 4216 | 40 | Code::CodeKey { rung: Decl, file: src/requests/adapters.py, decl: 6, sub: 0, line: 122 } |  |  | 0.619 |
| walker |  | 4223 | 7 | Code::CodeKey { rung: Body, file: src/requests/adapters.py, decl: 7, sub: 0, line: 125 } |  |  | 0.619 |
| walker |  | 4232 | 9 | Code::CodeKey { rung: Doc, file: src/requests/adapters.py, decl: 6, sub: 0, line: 122 } |  |  | 0.619 |
| walker |  | 4242 | 10 | Code::CodeKey { rung: Body, file: src/requests/adapters.py, decl: 9, sub: 0, line: 153 } |  |  | 0.619 |
| walker |  | 4253 | 11 | Code::CodeKey { rung: Doc, file: src/requests/adapters.py, decl: 9, sub: 0, line: 153 } |  |  | 0.619 |
| ns | 4313 |  | 282 | `Response`: `__init__`, dunders and every `@property` | 3.4 | 3.3 | 0.603 |
| walker |  | 4323 | 70 | Code::CodeKey { rung: Decl, file: src/requests/adapters.py, decl: 5, sub: 0, line: 85 } |  |  | 0.603 |
| walker |  | 4420 | 97 | Code::CodeKey { rung: Decl, file: src/requests/adapters.py, decl: 8, sub: 0, line: 128 } |  |  | 0.604 |
| walker |  | 4430 | 10 | Code::CodeKey { rung: Body, file: src/requests/adapters.py, decl: 8, sub: 0, line: 128 } |  |  | 0.604 |
| ns | 4465 |  | 152 | `Response`: the remaining members (`iter_content` … `close`) | 3.5 | 3.4 | 0.595 |
| walker |  | 4587 | 157 | Code::CodeKey { rung: Names, file: src/requests/compat.py, decl: 0, sub: 0, line: 0 } |  |  | 0.595 |
| walker |  | 4599 | 12 | Code::CodeKey { rung: Doc, file: src/requests/compat.py, decl: 1, sub: 0, line: 37 } |  |  | 0.595 |
| ns | 4680 |  | 215 | `adapters.py`: pool defaults + the `BaseAdapter` contract | 3.6 |  | 0.611 |
| walker |  | 4855 | 256 | Markdown::Section { file: README.md, section_index: 4, keeps_default_concavity: false } |  |  | 0.611 |
| walker |  | 4861 | 6 | Fs::DirListing { dir: tests/certs/valid } |  |  | 0.611 |
| ns | 5044 |  | 364 | `HTTPAdapter`: constructor knobs + complete method roster | 3.7 | 3.6 | 0.589 |
| walker |  | 5214 | 353 | Code::CodeKey { rung: Names, file: src/requests/exceptions.py, decl: 0, sub: 0, line: 0 } |  |  | 0.632 |
| walker |  | 5262 | 48 | Code::CodeKey { rung: Decl, file: src/requests/exceptions.py, decl: 4, sub: 0, line: 42 } |  |  | 0.632 |
| walker |  | 5270 | 8 | Code::CodeKey { rung: Doc, file: src/requests/exceptions.py, decl: 15, sub: 0, line: 106 } |  |  | 0.632 |
| walker |  | 5279 | 9 | Code::CodeKey { rung: Doc, file: src/requests/exceptions.py, decl: 3, sub: 0, line: 38 } |  |  | 0.632 |
| walker |  | 5288 | 9 | Code::CodeKey { rung: Doc, file: src/requests/exceptions.py, decl: 7, sub: 0, line: 66 } |  |  | 0.632 |
| walker |  | 5297 | 9 | Code::CodeKey { rung: Doc, file: src/requests/exceptions.py, decl: 8, sub: 0, line: 70 } |  |  | 0.632 |
| walker |  | 5306 | 9 | Code::CodeKey { rung: Doc, file: src/requests/exceptions.py, decl: 9, sub: 0, line: 74 } |  |  | 0.632 |
| ns | 5313 |  | 269 | `SessionRedirectMixin` + session-module merge helpers | 3.8 |  | 0.645 |
| walker |  | 5315 | 9 | Code::CodeKey { rung: Doc, file: src/requests/exceptions.py, decl: 10, sub: 0, line: 78 } |  |  | 0.645 |
| walker |  | 5324 | 9 | Code::CodeKey { rung: Doc, file: src/requests/exceptions.py, decl: 24, sub: 0, line: 142 } |  |  | 0.645 |
| walker |  | 5333 | 9 | Code::CodeKey { rung: Doc, file: src/requests/exceptions.py, decl: 26, sub: 0, line: 153 } |  |  | 0.645 |
| walker |  | 5389 | 56 | Code::CodeKey { rung: Decl, file: src/requests/exceptions.py, decl: 1, sub: 0, line: 20 } |  |  | 0.645 |
| walker |  | 5399 | 10 | Code::CodeKey { rung: Doc, file: src/requests/exceptions.py, decl: 22, sub: 0, line: 134 } |  |  | 0.645 |
| walker |  | 5410 | 11 | Code::CodeKey { rung: Doc, file: src/requests/exceptions.py, decl: 4, sub: 0, line: 42 } |  |  | 0.645 |
| walker |  | 5421 | 11 | Code::CodeKey { rung: Doc, file: src/requests/exceptions.py, decl: 18, sub: 0, line: 118 } |  |  | 0.645 |
| walker |  | 5432 | 11 | Code::CodeKey { rung: Doc, file: src/requests/exceptions.py, decl: 20, sub: 0, line: 126 } |  |  | 0.645 |
| walker |  | 5444 | 12 | Code::CodeKey { rung: Doc, file: src/requests/exceptions.py, decl: 19, sub: 0, line: 122 } |  |  | 0.645 |
| walker |  | 5457 | 13 | Code::CodeKey { rung: Doc, file: src/requests/exceptions.py, decl: 23, sub: 0, line: 138 } |  |  | 0.645 |
| walker |  | 5471 | 14 | Code::CodeKey { rung: Doc, file: src/requests/exceptions.py, decl: 14, sub: 0, line: 102 } |  |  | 0.645 |
| walker |  | 5485 | 14 | Code::CodeKey { rung: Doc, file: src/requests/exceptions.py, decl: 17, sub: 0, line: 114 } |  |  | 0.645 |
| walker |  | 5499 | 14 | Code::CodeKey { rung: Doc, file: src/requests/exceptions.py, decl: 28, sub: 0, line: 161 } |  |  | 0.645 |
| walker |  | 5515 | 16 | Code::CodeKey { rung: Doc, file: src/requests/exceptions.py, decl: 21, sub: 0, line: 130 } |  |  | 0.645 |
| walker |  | 5532 | 17 | Code::CodeKey { rung: Doc, file: src/requests/exceptions.py, decl: 25, sub: 0, line: 146 } |  |  | 0.645 |
| ns | 5549 |  | 236 | `cookies.py` module map: classes + free functions | 3.9 |  | 0.634 |
| walker |  | 5550 | 18 | Code::CodeKey { rung: Doc, file: src/requests/exceptions.py, decl: 13, sub: 0, line: 98 } |  |  | 0.634 |
| walker |  | 5568 | 18 | Code::CodeKey { rung: Doc, file: src/requests/exceptions.py, decl: 16, sub: 0, line: 110 } |  |  | 0.634 |
| walker |  | 5587 | 19 | Code::CodeKey { rung: Doc, file: src/requests/exceptions.py, decl: 2, sub: 0, line: 28 } |  |  | 0.634 |
| walker |  | 5606 | 19 | Code::CodeKey { rung: Doc, file: src/requests/exceptions.py, decl: 27, sub: 0, line: 157 } |  |  | 0.634 |
| walker |  | 5634 | 28 | Code::CodeKey { rung: Doc, file: src/requests/exceptions.py, decl: 1, sub: 0, line: 20 } |  |  | 0.634 |
| walker |  | 5677 | 43 | Code::CodeKey { rung: Doc, file: src/requests/exceptions.py, decl: 12, sub: 0, line: 91 } |  |  | 0.634 |
| walker |  | 5742 | 65 | Code::CodeKey { rung: Doc, file: src/requests/exceptions.py, decl: 11, sub: 0, line: 82 } |  |  | 0.634 |
| ns | 5857 |  | 308 | `auth.py`: every auth handler and its methods | 3.10 |  | 0.620 |
| walker |  | 6148 | 406 | Code::CodeKey { rung: Decl, file: src/requests/adapters.py, decl: 10, sub: 0, line: 158 } |  |  | 0.630 |
| walker |  | 6179 | 31 | Code::CodeKey { rung: Decl, file: src/requests/adapters.py, decl: 22, sub: 0, line: 565 } |  |  | 0.630 |
| walker |  | 6213 | 34 | Code::CodeKey { rung: Decl, file: src/requests/adapters.py, decl: 20, sub: 0, line: 512 } |  |  | 0.630 |
| ns | 6229 |  | 372 | `_types.py`: public type-alias roster | 4.1 |  | 0.613 |
| walker |  | 6252 | 39 | Code::CodeKey { rung: Decl, file: src/requests/adapters.py, decl: 16, sub: 0, line: 307 } |  |  | 0.613 |
| walker |  | 6300 | 48 | Code::CodeKey { rung: Decl, file: src/requests/adapters.py, decl: 18, sub: 0, line: 403 } |  |  | 0.613 |
| walker |  | 6359 | 59 | Code::CodeKey { rung: Decl, file: src/requests/adapters.py, decl: 14, sub: 0, line: 239 } |  |  | 0.613 |
| walker |  | 6431 | 72 | Code::CodeKey { rung: Decl, file: src/requests/adapters.py, decl: 19, sub: 0, line: 455 } |  |  | 0.613 |
| walker |  | 6507 | 76 | Code::CodeKey { rung: Decl, file: src/requests/adapters.py, decl: 11, sub: 0, line: 201 } |  |  | 0.625 |
| ns | 6540 |  | 311 | `_types.py`: the `Unpack` kwargs TypedDicts | 4.2 | 4.1 | 0.609 |
| walker |  | 6604 | 97 | Code::CodeKey { rung: Decl, file: src/requests/adapters.py, decl: 25, sub: 0, line: 634 } |  |  | 0.628 |
| walker |  | 6657 | 53 | Code::CodeKey { rung: Doc, file: src/requests/adapters.py, decl: 21, sub: 0, line: 555 } |  |  | 0.628 |
| walker |  | 6724 | 67 | Code::CodeKey { rung: Doc, file: src/requests/sessions.py, decl: 1, sub: 0, line: 76 } |  |  | 0.628 |
| ns | 6739 |  | 199 | Environment-driven settings: the `merge_environment_settings` core | 4.3 | 2.6 | 0.618 |
| ns | 6846 |  | 107 | `utils.py` module-level constants | 5.1 |  | 0.615 |
| walker |  | 6870 | 146 | Code::CodeKey { rung: Names, file: src/requests/models.py, decl: 0, sub: 0, line: 0 } |  |  | 0.620 |
| walker |  | 6929 | 59 | Code::CodeKey { rung: Decl, file: src/requests/models.py, decl: 13, sub: 0, line: 255 } |  |  | 0.620 |
| walker |  | 6965 | 36 | Code::CodeKey { rung: Decl, file: src/requests/models.py, decl: 14, sub: 0, line: 258 } |  |  | 0.620 |
| walker |  | 6977 | 12 | Code::CodeKey { rung: Doc, file: src/requests/models.py, decl: 14, sub: 0, line: 258 } |  |  | 0.620 |
| ns | 6994 |  | 148 | `utils.py` function roster 1/3: proxies shim, lengths, netrc, encoders | 5.2 |  | 0.612 |
| walker |  | 7051 | 74 | Code::CodeKey { rung: Decl, file: src/requests/models.py, decl: 1, sub: 0, line: 96 } |  |  | 0.619 |
| ns | 7161 |  | 167 | `utils.py` function roster 2/3: cookies, encoding detection, URI quoting, CIDR | 5.3 | 5.2 | 0.611 |
| walker |  | 7175 | 124 | Code::CodeKey { rung: Decl, file: src/requests/models.py, decl: 5, sub: 0, line: 109 } |  |  | 0.611 |
| walker |  | 7181 | 6 | Code::CodeKey { rung: Decl, file: src/requests/models.py, decl: 6, sub: 0, line: 112 } |  |  | 0.611 |
| walker |  | 7196 | 15 | Code::CodeKey { rung: Decl, file: src/requests/models.py, decl: 8, sub: 0, line: 137 } |  |  | 0.611 |
| walker |  | 7213 | 17 | Code::CodeKey { rung: Decl, file: src/requests/models.py, decl: 7, sub: 0, line: 133 } |  |  | 0.611 |
| walker |  | 7230 | 17 | Code::CodeKey { rung: Decl, file: src/requests/models.py, decl: 10, sub: 0, line: 147 } |  |  | 0.611 |
| walker |  | 7271 | 41 | Code::CodeKey { rung: Decl, file: src/requests/models.py, decl: 11, sub: 0, line: 151 } |  |  | 0.611 |
| walker |  | 7314 | 43 | Code::CodeKey { rung: Decl, file: src/requests/models.py, decl: 12, sub: 0, line: 183 } |  |  | 0.611 |
| ns | 7341 |  | 180 | `utils.py` function roster 3/3: proxy resolution, default headers, header validation | 5.4 | 5.3 | 0.603 |
| walker |  | 7360 | 46 | Code::CodeKey { rung: Decl, file: src/requests/models.py, decl: 9, sub: 0, line: 141 } |  |  | 0.603 |
| walker |  | 7373 | 13 | Code::CodeKey { rung: Doc, file: src/requests/models.py, decl: 6, sub: 0, line: 112 } |  |  | 0.603 |
| walker |  | 7541 | 168 | Code::CodeKey { rung: Decl, file: src/requests/models.py, decl: 16, sub: 0, line: 283 } |  |  | 0.623 |
| walker |  | 7565 | 24 | Code::CodeKey { rung: Doc, file: src/requests/models.py, decl: 19, sub: 0, line: 358 } |  |  | 0.623 |
| ns | 7602 |  | 261 | `structures.py`: `CaseInsensitiveDict` and `LookupDict` in full shape | 5.5 |  | 0.612 |
| ns | 7712 |  | 110 | `status_codes.py`: how `requests.codes` is built | 5.6 |  | 0.607 |
| walker |  | 7736 | 171 | Code::CodeKey { rung: Decl, file: src/requests/models.py, decl: 17, sub: 0, line: 321 } |  |  | 0.607 |
| walker |  | 7771 | 35 | Code::CodeKey { rung: Doc, file: src/requests/models.py, decl: 15, sub: 0, line: 271 } |  |  | 0.607 |
| ns | 7954 |  | 242 | The five small modules: hooks, `_internal_utils`, compat, certs, packages | 5.7 |  | 0.600 |
| walker |  | 8093 | 322 | Code::CodeKey { rung: Decl, file: src/requests/models.py, decl: 20, sub: 0, line: 376 } |  |  | 0.622 |
| walker |  | 8101 | 8 | Code::CodeKey { rung: Decl, file: src/requests/models.py, decl: 26, sub: 0, line: 471 } |  |  | 0.624 |
| ns | 8131 |  | 177 | `help.py`: the bug-report payload | 5.8 |  | 0.617 |
| walker |  | 8134 | 33 | Code::CodeKey { rung: Decl, file: src/requests/models.py, decl: 32, sub: 0, line: 697 } |  |  | 0.617 |
| walker |  | 8173 | 39 | Code::CodeKey { rung: Decl, file: src/requests/models.py, decl: 29, sub: 0, line: 574 } |  |  | 0.617 |
| walker |  | 8213 | 40 | Code::CodeKey { rung: Decl, file: src/requests/models.py, decl: 27, sub: 0, line: 481 } |  |  | 0.617 |
| walker |  | 8254 | 41 | Code::CodeKey { rung: Decl, file: src/requests/models.py, decl: 31, sub: 0, line: 668 } |  |  | 0.617 |
| walker |  | 8266 | 12 | Code::CodeKey { rung: Doc, file: src/requests/models.py, decl: 33, sub: 0, line: 720 } |  |  | 0.617 |
| walker |  | 8279 | 13 | Code::CodeKey { rung: Doc, file: src/requests/models.py, decl: 25, sub: 0, line: 465 } |  |  | 0.617 |
| walker |  | 8292 | 13 | Code::CodeKey { rung: Doc, file: src/requests/models.py, decl: 27, sub: 0, line: 481 } |  |  | 0.617 |
| walker |  | 8305 | 13 | Code::CodeKey { rung: Doc, file: src/requests/models.py, decl: 28, sub: 0, line: 563 } |  |  | 0.617 |
| walker |  | 8319 | 14 | Code::CodeKey { rung: Doc, file: src/requests/models.py, decl: 29, sub: 0, line: 574 } |  |  | 0.617 |
| walker |  | 8333 | 14 | Code::CodeKey { rung: Doc, file: src/requests/models.py, decl: 31, sub: 0, line: 668 } |  |  | 0.617 |
| walker |  | 8350 | 17 | Code::CodeKey { rung: Doc, file: src/requests/models.py, decl: 30, sub: 0, line: 652 } |  |  | 0.617 |
| ns | 8464 |  | 333 | `resolve_redirects`: the rules the redirect loop enforces | 6.1 | 3.8 | 0.604 |
| walker |  | 8526 | 176 | Code::CodeKey { rung: Decl, file: src/requests/models.py, decl: 22, sub: 0, line: 422 } |  |  | 0.604 |
| walker |  | 8542 | 16 | Code::CodeKey { rung: Doc, file: src/requests/models.py, decl: 22, sub: 0, line: 422 } |  |  | 0.604 |
| walker |  | 8619 | 77 | Code::CodeKey { rung: Doc, file: src/requests/exceptions.py, decl: 5, sub: 0, line: 45 } |  |  | 0.604 |
| walker |  | 8669 | 50 | Code::CodeKey { rung: Names, file: src/requests/status_codes.py, decl: 0, sub: 0, line: 0 } |  |  | 0.605 |
| ns | 8753 |  | 289 | `HTTPAdapter.send`: the urllib3 call and the `MaxRetryError` fan-out | 6.2 | 3.7 | 0.593 |
| walker |  | 8803 | 134 | Code::CodeKey { rung: Names, file: src/requests/api.py, decl: 0, sub: 0, line: 0 } |  |  | 0.603 |
| walker |  | 8838 | 35 | Code::CodeKey { rung: Decl, file: src/requests/api.py, decl: 1, sub: 0, line: 24 } |  |  | 0.603 |
| walker |  | 8878 | 40 | Code::CodeKey { rung: Decl, file: src/requests/api.py, decl: 2, sub: 0, line: 74 } |  |  | 0.603 |
| walker |  | 8918 | 40 | Code::CodeKey { rung: Decl, file: src/requests/api.py, decl: 6, sub: 0, line: 137 } |  |  | 0.603 |
| ns | 8955 |  | 202 | `HTTPAdapter.send`: the remaining `except` arms and the return | 6.3 | 6.2 | 0.595 |
| walker |  | 8958 | 40 | Code::CodeKey { rung: Decl, file: src/requests/api.py, decl: 7, sub: 0, line: 154 } |  |  | 0.595 |
| walker |  | 9022 | 64 | Code::CodeKey { rung: Decl, file: src/requests/api.py, decl: 5, sub: 0, line: 117 } |  |  | 0.595 |
| walker |  | 9037 | 15 | Code::CodeKey { rung: Body, file: src/requests/api.py, decl: 3, sub: 0, line: 90 } |  |  | 0.595 |
| ns | 9043 |  | 88 | Makefile: the remaining targets (coverage, publish, docs) | 7.1 | 1.8 | 0.600 |
| walker |  | 9052 | 15 | Code::CodeKey { rung: Body, file: src/requests/api.py, decl: 8, sub: 0, line: 171 } |  |  | 0.600 |
| walker |  | 9070 | 18 | Code::CodeKey { rung: Body, file: src/requests/api.py, decl: 2, sub: 0, line: 74 } |  |  | 0.600 |
| walker |  | 9153 | 83 | Code::CodeKey { rung: Doc, file: src/requests/models.py, decl: 11, sub: 0, line: 151 } |  |  | 0.600 |
| walker |  | 9279 | 126 | Code::CodeKey { rung: Names, file: src/requests/auth.py, decl: 0, sub: 0, line: 0 } |  |  | 0.603 |
| walker |  | 9301 | 22 | Code::CodeKey { rung: Decl, file: src/requests/auth.py, decl: 4, sub: 0, line: 78 } |  |  | 0.604 |
| walker |  | 9323 | 22 | Code::CodeKey { rung: Decl, file: src/requests/auth.py, decl: 13, sub: 0, line: 116 } |  |  | 0.605 |
| ns | 9326 |  | 283 | pyproject: every remaining table, and the settings that change how you work here | 7.2 | 1.10 | 0.593 |
| walker |  | 9335 | 12 | Code::CodeKey { rung: Doc, file: src/requests/auth.py, decl: 3, sub: 0, line: 34 } |  |  | 0.593 |
| walker |  | 9348 | 13 | Code::CodeKey { rung: Doc, file: src/requests/auth.py, decl: 4, sub: 0, line: 78 } |  |  | 0.593 |
| walker |  | 9363 | 15 | Code::CodeKey { rung: Doc, file: src/requests/auth.py, decl: 13, sub: 0, line: 116 } |  |  | 0.593 |
| ns | 9481 |  | 155 | `tests/conftest.py`: the fixture set every test builds on | 7.3 |  | 0.589 |
| walker |  | 9530 | 167 | Code::CodeKey { rung: Decl, file: src/requests/auth.py, decl: 6, sub: 0, line: 85 } |  |  | 0.592 |
| walker |  | 9537 | 7 | Code::CodeKey { rung: Decl, file: src/requests/auth.py, decl: 7, sub: 0, line: 91 } |  |  | 0.592 |
| walker |  | 9544 | 7 | Code::CodeKey { rung: Decl, file: src/requests/auth.py, decl: 8, sub: 0, line: 93 } |  |  | 0.592 |
| walker |  | 9559 | 15 | Code::CodeKey { rung: Doc, file: src/requests/auth.py, decl: 6, sub: 0, line: 85 } |  |  | 0.592 |
| ns | 9576 |  | 95 | `tests/test_requests.py`: top-level class roster | 7.4 |  | 0.589 |
| ns | 9758 |  | 182 | `tests/testserver/server.py`: the local socket server API (complete) | 7.5 |  | 0.583 |
| walker |  | 9887 | 328 | Code::CodeKey { rung: Decl, file: src/requests/auth.py, decl: 15, sub: 0, line: 124 } |  |  | 0.594 |
| walker |  | 9894 | 7 | Code::CodeKey { rung: Decl, file: src/requests/auth.py, decl: 16, sub: 0, line: 136 } |  |  | 0.594 |
| walker |  | 9901 | 7 | Code::CodeKey { rung: Decl, file: src/requests/auth.py, decl: 17, sub: 0, line: 138 } |  |  | 0.594 |
| ns | 9915 |  | 157 | `docs/api.rst`: every section of the developer interface | 7.6 |  | 0.588 |
| walker |  | 9916 | 15 | Code::CodeKey { rung: Doc, file: src/requests/auth.py, decl: 15, sub: 0, line: 124 } |  |  | 0.588 |
| walker |  | 9931 | 15 | Code::CodeKey { rung: Doc, file: src/requests/auth.py, decl: 21, sub: 0, line: 268 } |  |  | 0.588 |
| walker |  | 9953 | 22 | Code::CodeKey { rung: Doc, file: src/requests/auth.py, decl: 20, sub: 0, line: 157 } |  |  | 0.588 |
| ns | 9963 |  | 48 | `HISTORY.md`: the changelog's head | 7.7 |  | 0.586 |
| walker |  | 9998 | 45 | Code::CodeKey { rung: Doc, file: src/requests/auth.py, decl: 22, sub: 0, line: 273 } |  |  | 0.586 |
