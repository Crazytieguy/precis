Score(3000)=0.745 I=0.920 C=0.603 ns_rows≤3K=17/49 grid(1000/1442/2080/3000/4327/6240/9000)=0.692/0.630/0.689/0.745/0.627/0.606/0.586

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
| walker |  | 1334 | 352 | Code::CodeKey { rung: ModuleDoc, file: src/requests/__init__.py, decl: 0, sub: 0, line: 0 } |  |  | 0.630 |
| ns | 1458 |  | 271 | `requests/__init__.py`: name → module re-export map | 2.1 |  | 0.574 |
| walker |  | 1475 | 141 | Toml::Dependencies { file: pyproject.toml } |  |  | 0.586 |
| walker |  | 1518 | 43 | Fs::DirListing { dir: .github } |  |  | 0.601 |
| ns | 1532 |  | 74 | `api.py`: names of all eight module-level functions | 2.2 |  | 0.587 |
| walker |  | 1559 | 41 | Fs::DirListing { dir: .github/workflows } |  |  | 0.625 |
| walker |  | 1573 | 14 | Fs::DirListing { dir: .github/ISSUE_TEMPLATE } |  |  | 0.644 |
| walker |  | 1720 | 147 | Markdown::Section { file: README.md, section_index: 1, keeps_default_concavity: true } |  |  | 0.644 |
| walker |  | 1783 | 63 | Markdown::Section { file: README.md, section_index: 2, keeps_default_concavity: false } |  |  | 0.644 |
| walker |  | 1860 | 77 | Fs::DirListing { dir: tests } |  |  | 0.689 |
| walker |  | 1872 | 12 | Fs::DirListing { dir: tests/testserver } |  |  | 0.701 |
| ns | 1885 |  | 353 | `exceptions.py`: the complete class hierarchy | 2.3 |  | 0.656 |
| walker |  | 1886 | 14 | Fs::DirListing { dir: tests/certs } |  |  | 0.683 |
| walker |  | 2011 | 125 | Toml::PackageMetadata { file: pyproject.toml } |  |  | 0.720 |
| walker |  | 2050 | 39 | Plaintext::Whole { file: docs/requirements.txt } |  |  | 0.720 |
| ns | 2061 |  | 176 | `Session` class declaration + docstring | 2.4 |  | 0.689 |
| walker |  | 2063 | 13 | Code::CodeKey { rung: ModuleDoc, file: tests/__init__.py, decl: 0, sub: 0, line: 0 } |  |  | 0.689 |
| walker |  | 2221 | 158 | Code::CodeKey { rung: ModuleDoc, file: src/requests/__version__.py, decl: 0, sub: 0, line: 0 } |  |  | 0.772 |
| walker |  | 2294 | 73 | Plaintext::DeclSurface { file: requirements-dev.txt } |  |  | 0.772 |
| ns | 2332 |  | 271 | `Session` attribute set: typed fields + `__attrs__` | 2.5 | 2.4 | 0.727 |
| walker |  | 2602 | 308 | Code::CodeKey { rung: Names, file: src/requests/__init__.py, decl: 0, sub: 0, line: 0 } |  |  | 0.780 |
| ns | 2620 |  | 288 | `Session` method roster (complete, with line coordinates) | 2.6 | 2.4 | 0.751 |
| walker |  | 2649 | 47 | Code::CodeKey { rung: Decl, file: src/requests/__init__.py, decl: 1, sub: 0, line: 60 } |  |  | 0.751 |
| ns | 2871 |  | 251 | `Session.request`: the complete keyword signature | 2.7 | 2.6 | 0.726 |
| walker |  | 2918 | 269 | Code::CodeKey { rung: Names, file: src/requests/__init__.py, decl: 0, sub: 1, line: 0 } |  |  | 0.745 |
| walker |  | 3028 | 110 | Plaintext::DeclSurface { file: tox.ini } |  |  | 0.745 |
| ns | 3054 |  | 183 | `Session.__init__`: every default value | 2.8 | 2.6 | 0.726 |
| walker |  | 3150 | 122 | Code::CodeKey { rung: Body, file: src/requests/__init__.py, decl: 2, sub: 0, line: 99 } |  |  | 0.726 |
| walker |  | 3326 | 176 | Markdown::Section { file: README.md, section_index: 3, keeps_default_concavity: false } |  |  | 0.726 |
| ns | 3407 |  | 353 | `models.py` module map: constants, all five classes, `Request` fields | 3.1 |  | 0.695 |
| ns | 3728 |  | 321 | `PreparedRequest`: attributes + every `prepare_*` step | 3.2 | 3.1 | 0.672 |
| walker |  | 3836 | 510 | Code::CodeKey { rung: Body, file: src/requests/__init__.py, decl: 1, sub: 0, line: 60 } |  |  | 0.672 |
| walker |  | 3842 | 6 | Fs::DirListing { dir: tests/certs/valid } |  |  | 0.672 |
| walker |  | 3899 | 57 | Code::CodeKey { rung: Names, file: src/requests/sessions.py, decl: 0, sub: 0, line: 0 } |  |  | 0.672 |
| walker |  | 3930 | 31 | Code::CodeKey { rung: Decl, file: src/requests/sessions.py, decl: 1, sub: 0, line: 76 } |  |  | 0.672 |
| walker |  | 3938 | 8 | Code::CodeKey { rung: Body, file: src/requests/sessions.py, decl: 31, sub: 0, line: 908 } |  |  | 0.672 |
| walker |  | 3991 | 53 | Code::CodeKey { rung: Decl, file: src/requests/sessions.py, decl: 2, sub: 0, line: 108 } |  |  | 0.672 |
| ns | 4031 |  | 303 | `Response` attribute set: typed fields + `__attrs__` | 3.3 | 3.1 | 0.642 |
| walker |  | 4148 | 157 | Code::CodeKey { rung: Decl, file: src/requests/sessions.py, decl: 3, sub: 0, line: 127 } |  |  | 0.643 |
| walker |  | 4174 | 26 | Code::CodeKey { rung: Decl, file: src/requests/sessions.py, decl: 8, sub: 0, line: 309 } |  |  | 0.643 |
| walker |  | 4200 | 26 | Code::CodeKey { rung: Decl, file: src/requests/sessions.py, decl: 10, sub: 0, line: 370 } |  |  | 0.643 |
| walker |  | 4245 | 45 | Code::CodeKey { rung: Decl, file: src/requests/sessions.py, decl: 9, sub: 0, line: 334 } |  |  | 0.643 |
| walker |  | 4263 | 18 | Code::CodeKey { rung: Doc, file: src/requests/sessions.py, decl: 6, sub: 0, line: 154 } |  |  | 0.643 |
| walker |  | 4283 | 20 | Code::CodeKey { rung: Doc, file: src/requests/sessions.py, decl: 5, sub: 0, line: 134 } |  |  | 0.644 |
| ns | 4313 |  | 282 | `Response`: `__init__`, dunders and every `@property` | 3.4 | 3.3 | 0.627 |
| walker |  | 4418 | 135 | Code::CodeKey { rung: Decl, file: src/requests/sessions.py, decl: 7, sub: 0, line: 186 } |  |  | 0.627 |
| walker |  | 4437 | 19 | Code::CodeKey { rung: Doc, file: src/requests/sessions.py, decl: 7, sub: 0, line: 186 } |  |  | 0.627 |
| ns | 4465 |  | 152 | `Response`: the remaining members (`iter_content` … `close`) | 3.5 | 3.4 | 0.618 |
| walker |  | 4476 | 39 | Code::CodeKey { rung: Doc, file: src/requests/sessions.py, decl: 10, sub: 0, line: 370 } |  |  | 0.618 |
| walker |  | 4532 | 56 | Code::CodeKey { rung: Doc, file: src/requests/sessions.py, decl: 8, sub: 0, line: 309 } |  |  | 0.618 |
| walker |  | 4589 | 57 | Code::CodeKey { rung: Doc, file: src/requests/sessions.py, decl: 2, sub: 0, line: 108 } |  |  | 0.618 |
| walker |  | 4656 | 67 | Code::CodeKey { rung: Doc, file: src/requests/sessions.py, decl: 1, sub: 0, line: 76 } |  |  | 0.618 |
| ns | 4680 |  | 215 | `adapters.py`: pool defaults + the `BaseAdapter` contract | 3.6 |  | 0.604 |
| ns | 5044 |  | 364 | `HTTPAdapter`: constructor knobs + complete method roster | 3.7 | 3.6 | 0.582 |
| walker |  | 5246 | 590 | Toml::Config { file: pyproject.toml } |  |  | 0.594 |
| walker |  | 5282 | 36 | Code::CodeKey { rung: Names, file: src/requests/help.py, decl: 0, sub: 0, line: 0 } |  |  | 0.594 |
| walker |  | 5295 | 13 | Code::CodeKey { rung: Doc, file: src/requests/help.py, decl: 2, sub: 0, line: 69 } |  |  | 0.594 |
| walker |  | 5309 | 14 | Code::CodeKey { rung: Doc, file: src/requests/help.py, decl: 3, sub: 0, line: 128 } |  |  | 0.594 |
| ns | 5313 |  | 269 | `SessionRedirectMixin` + session-module merge helpers | 3.8 |  | 0.610 |
| walker |  | 5327 | 18 | Code::CodeKey { rung: Body, file: src/requests/help.py, decl: 3, sub: 0, line: 128 } |  |  | 0.610 |
| walker |  | 5416 | 89 | Code::CodeKey { rung: Names, file: src/requests/adapters.py, decl: 0, sub: 0, line: 0 } |  |  | 0.611 |
| walker |  | 5456 | 40 | Code::CodeKey { rung: Decl, file: src/requests/adapters.py, decl: 6, sub: 0, line: 122 } |  |  | 0.614 |
| walker |  | 5463 | 7 | Code::CodeKey { rung: Body, file: src/requests/adapters.py, decl: 7, sub: 0, line: 125 } |  |  | 0.614 |
| walker |  | 5472 | 9 | Code::CodeKey { rung: Doc, file: src/requests/adapters.py, decl: 6, sub: 0, line: 122 } |  |  | 0.616 |
| walker |  | 5482 | 10 | Code::CodeKey { rung: Body, file: src/requests/adapters.py, decl: 9, sub: 0, line: 153 } |  |  | 0.616 |
| walker |  | 5493 | 11 | Code::CodeKey { rung: Doc, file: src/requests/adapters.py, decl: 9, sub: 0, line: 153 } |  |  | 0.618 |
| ns | 5549 |  | 236 | `cookies.py` module map: classes + free functions | 3.9 |  | 0.608 |
| walker |  | 5590 | 97 | Code::CodeKey { rung: Decl, file: src/requests/adapters.py, decl: 8, sub: 0, line: 128 } |  |  | 0.627 |
| walker |  | 5600 | 10 | Code::CodeKey { rung: Body, file: src/requests/adapters.py, decl: 8, sub: 0, line: 128 } |  |  | 0.627 |
| walker |  | 5670 | 70 | Code::CodeKey { rung: Decl, file: src/requests/adapters.py, decl: 5, sub: 0, line: 85 } |  |  | 0.627 |
| ns | 5857 |  | 308 | `auth.py`: every auth handler and its methods | 3.10 |  | 0.613 |
| walker |  | 6076 | 406 | Code::CodeKey { rung: Decl, file: src/requests/adapters.py, decl: 10, sub: 0, line: 158 } |  |  | 0.623 |
| walker |  | 6107 | 31 | Code::CodeKey { rung: Decl, file: src/requests/adapters.py, decl: 22, sub: 0, line: 565 } |  |  | 0.623 |
| walker |  | 6141 | 34 | Code::CodeKey { rung: Decl, file: src/requests/adapters.py, decl: 20, sub: 0, line: 512 } |  |  | 0.623 |
| walker |  | 6180 | 39 | Code::CodeKey { rung: Decl, file: src/requests/adapters.py, decl: 16, sub: 0, line: 307 } |  |  | 0.623 |
| walker |  | 6228 | 48 | Code::CodeKey { rung: Decl, file: src/requests/adapters.py, decl: 18, sub: 0, line: 403 } |  |  | 0.623 |
| ns | 6229 |  | 372 | `_types.py`: public type-alias roster | 4.1 |  | 0.606 |
| walker |  | 6287 | 59 | Code::CodeKey { rung: Decl, file: src/requests/adapters.py, decl: 14, sub: 0, line: 239 } |  |  | 0.606 |
| walker |  | 6359 | 72 | Code::CodeKey { rung: Decl, file: src/requests/adapters.py, decl: 19, sub: 0, line: 455 } |  |  | 0.606 |
| walker |  | 6435 | 76 | Code::CodeKey { rung: Decl, file: src/requests/adapters.py, decl: 11, sub: 0, line: 201 } |  |  | 0.618 |
| walker |  | 6532 | 97 | Code::CodeKey { rung: Decl, file: src/requests/adapters.py, decl: 25, sub: 0, line: 634 } |  |  | 0.637 |
| ns | 6540 |  | 311 | `_types.py`: the `Unpack` kwargs TypedDicts | 4.2 | 4.1 | 0.621 |
| walker |  | 6585 | 53 | Code::CodeKey { rung: Doc, file: src/requests/adapters.py, decl: 21, sub: 0, line: 555 } |  |  | 0.621 |
| ns | 6739 |  | 199 | Environment-driven settings: the `merge_environment_settings` core | 4.3 | 2.6 | 0.612 |
| walker |  | 6742 | 157 | Code::CodeKey { rung: Names, file: src/requests/compat.py, decl: 0, sub: 0, line: 0 } |  |  | 0.612 |
| walker |  | 6754 | 12 | Code::CodeKey { rung: Doc, file: src/requests/compat.py, decl: 1, sub: 0, line: 37 } |  |  | 0.612 |
| ns | 6846 |  | 107 | `utils.py` module-level constants | 5.1 |  | 0.609 |
| ns | 6994 |  | 148 | `utils.py` function roster 1/3: proxies shim, lengths, netrc, encoders | 5.2 |  | 0.601 |
| walker |  | 7107 | 353 | Code::CodeKey { rung: Names, file: src/requests/exceptions.py, decl: 0, sub: 0, line: 0 } |  |  | 0.635 |
| walker |  | 7115 | 8 | Code::CodeKey { rung: Doc, file: src/requests/exceptions.py, decl: 15, sub: 0, line: 106 } |  |  | 0.635 |
| ns | 7161 |  | 167 | `utils.py` function roster 2/3: cookies, encoding detection, URI quoting, CIDR | 5.3 | 5.2 | 0.627 |
| walker |  | 7163 | 48 | Code::CodeKey { rung: Decl, file: src/requests/exceptions.py, decl: 4, sub: 0, line: 42 } |  |  | 0.627 |
| walker |  | 7172 | 9 | Code::CodeKey { rung: Doc, file: src/requests/exceptions.py, decl: 3, sub: 0, line: 38 } |  |  | 0.627 |
| walker |  | 7181 | 9 | Code::CodeKey { rung: Doc, file: src/requests/exceptions.py, decl: 7, sub: 0, line: 66 } |  |  | 0.627 |
| walker |  | 7190 | 9 | Code::CodeKey { rung: Doc, file: src/requests/exceptions.py, decl: 8, sub: 0, line: 70 } |  |  | 0.627 |
| walker |  | 7199 | 9 | Code::CodeKey { rung: Doc, file: src/requests/exceptions.py, decl: 9, sub: 0, line: 74 } |  |  | 0.627 |
| walker |  | 7208 | 9 | Code::CodeKey { rung: Doc, file: src/requests/exceptions.py, decl: 10, sub: 0, line: 78 } |  |  | 0.627 |
| walker |  | 7217 | 9 | Code::CodeKey { rung: Doc, file: src/requests/exceptions.py, decl: 24, sub: 0, line: 142 } |  |  | 0.627 |
| walker |  | 7226 | 9 | Code::CodeKey { rung: Doc, file: src/requests/exceptions.py, decl: 26, sub: 0, line: 153 } |  |  | 0.627 |
| walker |  | 7282 | 56 | Code::CodeKey { rung: Decl, file: src/requests/exceptions.py, decl: 1, sub: 0, line: 20 } |  |  | 0.627 |
| walker |  | 7292 | 10 | Code::CodeKey { rung: Doc, file: src/requests/exceptions.py, decl: 22, sub: 0, line: 134 } |  |  | 0.627 |
| walker |  | 7303 | 11 | Code::CodeKey { rung: Doc, file: src/requests/exceptions.py, decl: 4, sub: 0, line: 42 } |  |  | 0.627 |
| walker |  | 7314 | 11 | Code::CodeKey { rung: Doc, file: src/requests/exceptions.py, decl: 18, sub: 0, line: 118 } |  |  | 0.627 |
| walker |  | 7325 | 11 | Code::CodeKey { rung: Doc, file: src/requests/exceptions.py, decl: 20, sub: 0, line: 126 } |  |  | 0.627 |
| walker |  | 7337 | 12 | Code::CodeKey { rung: Doc, file: src/requests/exceptions.py, decl: 19, sub: 0, line: 122 } |  |  | 0.627 |
| ns | 7341 |  | 180 | `utils.py` function roster 3/3: proxy resolution, default headers, header validation | 5.4 | 5.3 | 0.619 |
| walker |  | 7350 | 13 | Code::CodeKey { rung: Doc, file: src/requests/exceptions.py, decl: 23, sub: 0, line: 138 } |  |  | 0.619 |
| walker |  | 7364 | 14 | Code::CodeKey { rung: Doc, file: src/requests/exceptions.py, decl: 14, sub: 0, line: 102 } |  |  | 0.619 |
| walker |  | 7378 | 14 | Code::CodeKey { rung: Doc, file: src/requests/exceptions.py, decl: 17, sub: 0, line: 114 } |  |  | 0.619 |
| walker |  | 7392 | 14 | Code::CodeKey { rung: Doc, file: src/requests/exceptions.py, decl: 28, sub: 0, line: 161 } |  |  | 0.619 |
| walker |  | 7408 | 16 | Code::CodeKey { rung: Doc, file: src/requests/exceptions.py, decl: 21, sub: 0, line: 130 } |  |  | 0.619 |
| walker |  | 7425 | 17 | Code::CodeKey { rung: Doc, file: src/requests/exceptions.py, decl: 25, sub: 0, line: 146 } |  |  | 0.619 |
| walker |  | 7443 | 18 | Code::CodeKey { rung: Doc, file: src/requests/exceptions.py, decl: 13, sub: 0, line: 98 } |  |  | 0.619 |
| walker |  | 7461 | 18 | Code::CodeKey { rung: Doc, file: src/requests/exceptions.py, decl: 16, sub: 0, line: 110 } |  |  | 0.619 |
| walker |  | 7480 | 19 | Code::CodeKey { rung: Doc, file: src/requests/exceptions.py, decl: 2, sub: 0, line: 28 } |  |  | 0.619 |
| walker |  | 7499 | 19 | Code::CodeKey { rung: Doc, file: src/requests/exceptions.py, decl: 27, sub: 0, line: 157 } |  |  | 0.619 |
| walker |  | 7527 | 28 | Code::CodeKey { rung: Doc, file: src/requests/exceptions.py, decl: 1, sub: 0, line: 20 } |  |  | 0.619 |
| walker |  | 7570 | 43 | Code::CodeKey { rung: Doc, file: src/requests/exceptions.py, decl: 12, sub: 0, line: 91 } |  |  | 0.619 |
| ns | 7602 |  | 261 | `structures.py`: `CaseInsensitiveDict` and `LookupDict` in full shape | 5.5 |  | 0.608 |
| walker |  | 7635 | 65 | Code::CodeKey { rung: Doc, file: src/requests/exceptions.py, decl: 11, sub: 0, line: 82 } |  |  | 0.608 |
| walker |  | 7712 | 77 | Code::CodeKey { rung: Doc, file: src/requests/exceptions.py, decl: 5, sub: 0, line: 45 } |  |  | 0.604 |
| ns | 7712 |  | 110 | `status_codes.py`: how `requests.codes` is built | 5.6 |  | 0.604 |
| walker |  | 7858 | 146 | Code::CodeKey { rung: Names, file: src/requests/models.py, decl: 0, sub: 0, line: 0 } |  |  | 0.608 |
| walker |  | 7917 | 59 | Code::CodeKey { rung: Decl, file: src/requests/models.py, decl: 13, sub: 0, line: 255 } |  |  | 0.608 |
| walker |  | 7953 | 36 | Code::CodeKey { rung: Decl, file: src/requests/models.py, decl: 14, sub: 0, line: 258 } |  |  | 0.608 |
| ns | 7954 |  | 242 | The five small modules: hooks, `_internal_utils`, compat, certs, packages | 5.7 |  | 0.601 |
| walker |  | 7965 | 12 | Code::CodeKey { rung: Doc, file: src/requests/models.py, decl: 14, sub: 0, line: 258 } |  |  | 0.601 |
| walker |  | 8039 | 74 | Code::CodeKey { rung: Decl, file: src/requests/models.py, decl: 1, sub: 0, line: 96 } |  |  | 0.607 |
| ns | 8131 |  | 177 | `help.py`: the bug-report payload | 5.8 |  | 0.600 |
| walker |  | 8163 | 124 | Code::CodeKey { rung: Decl, file: src/requests/models.py, decl: 5, sub: 0, line: 109 } |  |  | 0.600 |
| walker |  | 8169 | 6 | Code::CodeKey { rung: Decl, file: src/requests/models.py, decl: 6, sub: 0, line: 112 } |  |  | 0.600 |
| walker |  | 8184 | 15 | Code::CodeKey { rung: Decl, file: src/requests/models.py, decl: 8, sub: 0, line: 137 } |  |  | 0.600 |
| walker |  | 8201 | 17 | Code::CodeKey { rung: Decl, file: src/requests/models.py, decl: 7, sub: 0, line: 133 } |  |  | 0.600 |
| walker |  | 8218 | 17 | Code::CodeKey { rung: Decl, file: src/requests/models.py, decl: 10, sub: 0, line: 147 } |  |  | 0.600 |
| walker |  | 8231 | 13 | Code::CodeKey { rung: Doc, file: src/requests/models.py, decl: 6, sub: 0, line: 112 } |  |  | 0.600 |
| walker |  | 8272 | 41 | Code::CodeKey { rung: Decl, file: src/requests/models.py, decl: 11, sub: 0, line: 151 } |  |  | 0.600 |
| walker |  | 8315 | 43 | Code::CodeKey { rung: Decl, file: src/requests/models.py, decl: 12, sub: 0, line: 183 } |  |  | 0.600 |
| walker |  | 8361 | 46 | Code::CodeKey { rung: Decl, file: src/requests/models.py, decl: 9, sub: 0, line: 141 } |  |  | 0.600 |
| ns | 8464 |  | 333 | `resolve_redirects`: the rules the redirect loop enforces | 6.1 | 3.8 | 0.588 |
| walker |  | 8529 | 168 | Code::CodeKey { rung: Decl, file: src/requests/models.py, decl: 16, sub: 0, line: 283 } |  |  | 0.605 |
| walker |  | 8553 | 24 | Code::CodeKey { rung: Doc, file: src/requests/models.py, decl: 19, sub: 0, line: 358 } |  |  | 0.605 |
| walker |  | 8724 | 171 | Code::CodeKey { rung: Decl, file: src/requests/models.py, decl: 17, sub: 0, line: 321 } |  |  | 0.605 |
| ns | 8753 |  | 289 | `HTTPAdapter.send`: the urllib3 call and the `MaxRetryError` fan-out | 6.2 | 3.7 | 0.594 |
| walker |  | 8759 | 35 | Code::CodeKey { rung: Doc, file: src/requests/models.py, decl: 15, sub: 0, line: 271 } |  |  | 0.594 |
| ns | 8955 |  | 202 | `HTTPAdapter.send`: the remaining `except` arms and the return | 6.3 | 6.2 | 0.586 |
| ns | 9043 |  | 88 | Makefile: the remaining targets (coverage, publish, docs) | 7.1 | 1.8 | 0.591 |
| walker |  | 9081 | 322 | Code::CodeKey { rung: Decl, file: src/requests/models.py, decl: 20, sub: 0, line: 376 } |  |  | 0.611 |
| walker |  | 9089 | 8 | Code::CodeKey { rung: Decl, file: src/requests/models.py, decl: 26, sub: 0, line: 471 } |  |  | 0.613 |
| walker |  | 9122 | 33 | Code::CodeKey { rung: Decl, file: src/requests/models.py, decl: 32, sub: 0, line: 697 } |  |  | 0.613 |
| walker |  | 9161 | 39 | Code::CodeKey { rung: Decl, file: src/requests/models.py, decl: 29, sub: 0, line: 574 } |  |  | 0.613 |
| walker |  | 9201 | 40 | Code::CodeKey { rung: Decl, file: src/requests/models.py, decl: 27, sub: 0, line: 481 } |  |  | 0.613 |
| walker |  | 9242 | 41 | Code::CodeKey { rung: Decl, file: src/requests/models.py, decl: 31, sub: 0, line: 668 } |  |  | 0.613 |
| walker |  | 9254 | 12 | Code::CodeKey { rung: Doc, file: src/requests/models.py, decl: 33, sub: 0, line: 720 } |  |  | 0.613 |
| walker |  | 9267 | 13 | Code::CodeKey { rung: Doc, file: src/requests/models.py, decl: 25, sub: 0, line: 465 } |  |  | 0.613 |
| walker |  | 9280 | 13 | Code::CodeKey { rung: Doc, file: src/requests/models.py, decl: 27, sub: 0, line: 481 } |  |  | 0.613 |
| walker |  | 9293 | 13 | Code::CodeKey { rung: Doc, file: src/requests/models.py, decl: 28, sub: 0, line: 563 } |  |  | 0.613 |
| walker |  | 9307 | 14 | Code::CodeKey { rung: Doc, file: src/requests/models.py, decl: 29, sub: 0, line: 574 } |  |  | 0.613 |
| walker |  | 9321 | 14 | Code::CodeKey { rung: Doc, file: src/requests/models.py, decl: 31, sub: 0, line: 668 } |  |  | 0.613 |
| ns | 9326 |  | 283 | pyproject: every remaining table, and the settings that change how you work here | 7.2 | 1.10 | 0.619 |
| walker |  | 9338 | 17 | Code::CodeKey { rung: Doc, file: src/requests/models.py, decl: 30, sub: 0, line: 652 } |  |  | 0.619 |
| ns | 9481 |  | 155 | `tests/conftest.py`: the fixture set every test builds on | 7.3 |  | 0.614 |
| walker |  | 9514 | 176 | Code::CodeKey { rung: Decl, file: src/requests/models.py, decl: 22, sub: 0, line: 422 } |  |  | 0.614 |
| walker |  | 9530 | 16 | Code::CodeKey { rung: Doc, file: src/requests/models.py, decl: 22, sub: 0, line: 422 } |  |  | 0.614 |
| ns | 9576 |  | 95 | `tests/test_requests.py`: top-level class roster | 7.4 |  | 0.611 |
| walker |  | 9580 | 50 | Code::CodeKey { rung: Names, file: src/requests/status_codes.py, decl: 0, sub: 0, line: 0 } |  |  | 0.612 |
| walker |  | 9714 | 134 | Code::CodeKey { rung: Names, file: src/requests/api.py, decl: 0, sub: 0, line: 0 } |  |  | 0.620 |
| walker |  | 9749 | 35 | Code::CodeKey { rung: Decl, file: src/requests/api.py, decl: 1, sub: 0, line: 24 } |  |  | 0.620 |
| ns | 9758 |  | 182 | `tests/testserver/server.py`: the local socket server API (complete) | 7.5 |  | 0.614 |
| walker |  | 9789 | 40 | Code::CodeKey { rung: Decl, file: src/requests/api.py, decl: 2, sub: 0, line: 74 } |  |  | 0.614 |
| walker |  | 9829 | 40 | Code::CodeKey { rung: Decl, file: src/requests/api.py, decl: 6, sub: 0, line: 137 } |  |  | 0.614 |
| walker |  | 9869 | 40 | Code::CodeKey { rung: Decl, file: src/requests/api.py, decl: 7, sub: 0, line: 154 } |  |  | 0.614 |
| ns | 9915 |  | 157 | `docs/api.rst`: every section of the developer interface | 7.6 |  | 0.608 |
| walker |  | 9933 | 64 | Code::CodeKey { rung: Decl, file: src/requests/api.py, decl: 5, sub: 0, line: 117 } |  |  | 0.608 |
| walker |  | 9948 | 15 | Code::CodeKey { rung: Body, file: src/requests/api.py, decl: 3, sub: 0, line: 90 } |  |  | 0.608 |
| walker |  | 9963 | 15 | Code::CodeKey { rung: Body, file: src/requests/api.py, decl: 8, sub: 0, line: 171 } |  |  | 0.606 |
| ns | 9963 |  | 48 | `HISTORY.md`: the changelog's head | 7.7 |  | 0.606 |
| walker |  | 9981 | 18 | Code::CodeKey { rung: Body, file: src/requests/api.py, decl: 2, sub: 0, line: 74 } |  |  | 0.606 |
