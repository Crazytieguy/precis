Score(3000)=0.726 I=0.914 C=0.577 ns_rows≤3K=17/49 grid(1000/1442/2080/3000/4327/6240/9000)=0.692/0.630/0.689/0.726/0.612/0.586/0.586

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
| ns | 2332 |  | 271 | `Session` attribute set: typed fields + `__attrs__` | 2.5 | 2.4 | 0.649 |
| walker |  | 2358 | 308 | Code::CodeKey { rung: Names, file: src/requests/__init__.py, decl: 0, sub: 0, line: 0 } |  |  | 0.701 |
| walker |  | 2405 | 47 | Code::CodeKey { rung: Decl, file: src/requests/__init__.py, decl: 1, sub: 0, line: 60 } |  |  | 0.701 |
| walker |  | 2441 | 36 | Code::CodeKey { rung: Names, file: src/requests/help.py, decl: 0, sub: 0, line: 0 } |  |  | 0.701 |
| walker |  | 2454 | 13 | Code::CodeKey { rung: Doc, file: src/requests/help.py, decl: 2, sub: 0, line: 69 } |  |  | 0.701 |
| walker |  | 2468 | 14 | Code::CodeKey { rung: Doc, file: src/requests/help.py, decl: 3, sub: 0, line: 128 } |  |  | 0.701 |
| walker |  | 2486 | 18 | Code::CodeKey { rung: Body, file: src/requests/help.py, decl: 3, sub: 0, line: 128 } |  |  | 0.701 |
| walker |  | 2499 | 13 | Code::CodeKey { rung: ModuleDoc, file: tests/__init__.py, decl: 0, sub: 0, line: 0 } |  |  | 0.701 |
| ns | 2620 |  | 288 | `Session` method roster (complete, with line coordinates) | 2.6 | 2.4 | 0.674 |
| walker |  | 2657 | 158 | Code::CodeKey { rung: ModuleDoc, file: src/requests/__version__.py, decl: 0, sub: 0, line: 0 } |  |  | 0.751 |
| walker |  | 2707 | 50 | Code::CodeKey { rung: Names, file: src/requests/status_codes.py, decl: 0, sub: 0, line: 0 } |  |  | 0.751 |
| walker |  | 2780 | 73 | Plaintext::DeclSurface { file: requirements-dev.txt } |  |  | 0.751 |
| walker |  | 2837 | 57 | Code::CodeKey { rung: Names, file: src/requests/hooks.py, decl: 0, sub: 0, line: 0 } |  |  | 0.751 |
| ns | 2871 |  | 251 | `Session.request`: the complete keyword signature | 2.7 | 2.6 | 0.726 |
| walker |  | 2890 | 53 | Code::CodeKey { rung: Decl, file: src/requests/hooks.py, decl: 3, sub: 0, line: 32 } |  |  | 0.726 |
| walker |  | 2907 | 17 | Code::CodeKey { rung: Body, file: src/requests/hooks.py, decl: 2, sub: 0, line: 25 } |  |  | 0.726 |
| walker |  | 2925 | 18 | Code::CodeKey { rung: Doc, file: src/requests/hooks.py, decl: 3, sub: 0, line: 32 } |  |  | 0.726 |
| walker |  | 2982 | 57 | Code::CodeKey { rung: Names, file: src/requests/sessions.py, decl: 0, sub: 0, line: 0 } |  |  | 0.726 |
| walker |  | 3013 | 31 | Code::CodeKey { rung: Decl, file: src/requests/sessions.py, decl: 1, sub: 0, line: 76 } |  |  | 0.726 |
| walker |  | 3021 | 8 | Code::CodeKey { rung: Body, file: src/requests/sessions.py, decl: 31, sub: 0, line: 908 } |  |  | 0.726 |
| ns | 3054 |  | 183 | `Session.__init__`: every default value | 2.8 | 2.6 | 0.708 |
| walker |  | 3074 | 53 | Code::CodeKey { rung: Decl, file: src/requests/sessions.py, decl: 2, sub: 0, line: 108 } |  |  | 0.708 |
| walker |  | 3231 | 157 | Code::CodeKey { rung: Decl, file: src/requests/sessions.py, decl: 3, sub: 0, line: 127 } |  |  | 0.709 |
| walker |  | 3257 | 26 | Code::CodeKey { rung: Decl, file: src/requests/sessions.py, decl: 8, sub: 0, line: 309 } |  |  | 0.709 |
| walker |  | 3283 | 26 | Code::CodeKey { rung: Decl, file: src/requests/sessions.py, decl: 10, sub: 0, line: 370 } |  |  | 0.709 |
| walker |  | 3328 | 45 | Code::CodeKey { rung: Decl, file: src/requests/sessions.py, decl: 9, sub: 0, line: 334 } |  |  | 0.709 |
| walker |  | 3401 | 73 | Code::CodeKey { rung: Names, file: src/requests/structures.py, decl: 0, sub: 0, line: 0 } |  |  | 0.709 |
| ns | 3407 |  | 353 | `models.py` module map: constants, all five classes, `Request` fields | 3.1 |  | 0.678 |
| walker |  | 3603 | 202 | Code::CodeKey { rung: Decl, file: src/requests/structures.py, decl: 14, sub: 0, line: 96 } |  |  | 0.679 |
| walker |  | 3610 | 7 | Code::CodeKey { rung: Decl, file: src/requests/structures.py, decl: 20, sub: 0, line: 126 } |  |  | 0.679 |
| walker |  | 3619 | 9 | Code::CodeKey { rung: Decl, file: src/requests/structures.py, decl: 19, sub: 0, line: 123 } |  |  | 0.679 |
| walker |  | 3627 | 8 | Code::CodeKey { rung: Doc, file: src/requests/structures.py, decl: 14, sub: 0, line: 96 } |  |  | 0.679 |
| ns | 3728 |  | 321 | `PreparedRequest`: attributes + every `prepare_*` step | 3.2 | 3.1 | 0.656 |
| walker |  | 3837 | 210 | Code::CodeKey { rung: Decl, file: src/requests/structures.py, decl: 3, sub: 0, line: 20 } |  |  | 0.657 |
| walker |  | 3891 | 54 | Code::CodeKey { rung: Decl, file: src/requests/structures.py, decl: 4, sub: 0, line: 49 } |  |  | 0.657 |
| walker |  | 4026 | 135 | Code::CodeKey { rung: Decl, file: src/requests/sessions.py, decl: 7, sub: 0, line: 186 } |  |  | 0.658 |
| ns | 4031 |  | 303 | `Response` attribute set: typed fields + `__attrs__` | 3.3 | 3.1 | 0.628 |
| walker |  | 4136 | 110 | Plaintext::DeclSurface { file: tox.ini } |  |  | 0.628 |
| walker |  | 4225 | 89 | Code::CodeKey { rung: Names, file: src/requests/adapters.py, decl: 0, sub: 0, line: 0 } |  |  | 0.628 |
| walker |  | 4265 | 40 | Code::CodeKey { rung: Decl, file: src/requests/adapters.py, decl: 6, sub: 0, line: 122 } |  |  | 0.629 |
| walker |  | 4274 | 9 | Code::CodeKey { rung: Doc, file: src/requests/adapters.py, decl: 6, sub: 0, line: 122 } |  |  | 0.629 |
| walker |  | 4281 | 7 | Code::CodeKey { rung: Body, file: src/requests/adapters.py, decl: 7, sub: 0, line: 125 } |  |  | 0.629 |
| walker |  | 4291 | 10 | Code::CodeKey { rung: Body, file: src/requests/adapters.py, decl: 9, sub: 0, line: 153 } |  |  | 0.629 |
| walker |  | 4302 | 11 | Code::CodeKey { rung: Doc, file: src/requests/adapters.py, decl: 9, sub: 0, line: 153 } |  |  | 0.629 |
| ns | 4313 |  | 282 | `Response`: `__init__`, dunders and every `@property` | 3.4 | 3.3 | 0.612 |
| walker |  | 4399 | 97 | Code::CodeKey { rung: Decl, file: src/requests/adapters.py, decl: 8, sub: 0, line: 128 } |  |  | 0.614 |
| ns | 4465 |  | 152 | `Response`: the remaining members (`iter_content` … `close`) | 3.5 | 3.4 | 0.605 |
| walker |  | 4469 | 70 | Code::CodeKey { rung: Decl, file: src/requests/adapters.py, decl: 5, sub: 0, line: 85 } |  |  | 0.605 |
| ns | 4680 |  | 215 | `adapters.py`: pool defaults + the `BaseAdapter` contract | 3.6 |  | 0.620 |
| walker |  | 4738 | 269 | Code::CodeKey { rung: Names, file: src/requests/__init__.py, decl: 0, sub: 1, line: 0 } |  |  | 0.635 |
| walker |  | 4860 | 122 | Code::CodeKey { rung: Body, file: src/requests/__init__.py, decl: 2, sub: 0, line: 99 } |  |  | 0.635 |
| walker |  | 4869 | 9 | Code::CodeKey { rung: Body, file: src/requests/structures.py, decl: 9, sub: 0, line: 73 } |  |  | 0.635 |
| ns | 5044 |  | 364 | `HTTPAdapter`: constructor knobs + complete method roster | 3.7 | 3.6 | 0.612 |
| walker |  | 5045 | 176 | Markdown::Section { file: README.md, section_index: 3, keeps_default_concavity: false } |  |  | 0.612 |
| ns | 5313 |  | 269 | `SessionRedirectMixin` + session-module merge helpers | 3.8 |  | 0.614 |
| ns | 5549 |  | 236 | `cookies.py` module map: classes + free functions | 3.9 |  | 0.605 |
| walker |  | 5555 | 510 | Code::CodeKey { rung: Body, file: src/requests/__init__.py, decl: 1, sub: 0, line: 60 } |  |  | 0.605 |
| walker |  | 5564 | 9 | Code::CodeKey { rung: Body, file: src/requests/structures.py, decl: 13, sub: 0, line: 92 } |  |  | 0.605 |
| walker |  | 5570 | 6 | Fs::DirListing { dir: tests/certs/valid } |  |  | 0.605 |
| ns | 5857 |  | 308 | `auth.py`: every auth handler and its methods | 3.10 |  | 0.591 |
| walker |  | 5976 | 406 | Code::CodeKey { rung: Decl, file: src/requests/adapters.py, decl: 10, sub: 0, line: 158 } |  |  | 0.602 |
| walker |  | 6007 | 31 | Code::CodeKey { rung: Decl, file: src/requests/adapters.py, decl: 22, sub: 0, line: 565 } |  |  | 0.602 |
| walker |  | 6041 | 34 | Code::CodeKey { rung: Decl, file: src/requests/adapters.py, decl: 20, sub: 0, line: 512 } |  |  | 0.602 |
| walker |  | 6080 | 39 | Code::CodeKey { rung: Decl, file: src/requests/adapters.py, decl: 16, sub: 0, line: 307 } |  |  | 0.602 |
| walker |  | 6128 | 48 | Code::CodeKey { rung: Decl, file: src/requests/adapters.py, decl: 18, sub: 0, line: 403 } |  |  | 0.602 |
| walker |  | 6187 | 59 | Code::CodeKey { rung: Decl, file: src/requests/adapters.py, decl: 14, sub: 0, line: 239 } |  |  | 0.602 |
| ns | 6229 |  | 372 | `_types.py`: public type-alias roster | 4.1 |  | 0.586 |
| walker |  | 6259 | 72 | Code::CodeKey { rung: Decl, file: src/requests/adapters.py, decl: 19, sub: 0, line: 455 } |  |  | 0.586 |
| walker |  | 6335 | 76 | Code::CodeKey { rung: Decl, file: src/requests/adapters.py, decl: 11, sub: 0, line: 201 } |  |  | 0.597 |
| walker |  | 6432 | 97 | Code::CodeKey { rung: Decl, file: src/requests/adapters.py, decl: 25, sub: 0, line: 634 } |  |  | 0.617 |
| ns | 6540 |  | 311 | `_types.py`: the `Unpack` kwargs TypedDicts | 4.2 | 4.1 | 0.602 |
| walker |  | 6558 | 126 | Code::CodeKey { rung: Names, file: src/requests/auth.py, decl: 0, sub: 0, line: 0 } |  |  | 0.606 |
| walker |  | 6580 | 22 | Code::CodeKey { rung: Decl, file: src/requests/auth.py, decl: 4, sub: 0, line: 78 } |  |  | 0.608 |
| walker |  | 6602 | 22 | Code::CodeKey { rung: Decl, file: src/requests/auth.py, decl: 13, sub: 0, line: 116 } |  |  | 0.609 |
| walker |  | 6615 | 13 | Code::CodeKey { rung: Doc, file: src/requests/auth.py, decl: 4, sub: 0, line: 78 } |  |  | 0.609 |
| walker |  | 6630 | 15 | Code::CodeKey { rung: Doc, file: src/requests/auth.py, decl: 13, sub: 0, line: 116 } |  |  | 0.609 |
| ns | 6739 |  | 199 | Environment-driven settings: the `merge_environment_settings` core | 4.3 | 2.6 | 0.600 |
| walker |  | 6797 | 167 | Code::CodeKey { rung: Decl, file: src/requests/auth.py, decl: 6, sub: 0, line: 85 } |  |  | 0.604 |
| walker |  | 6804 | 7 | Code::CodeKey { rung: Decl, file: src/requests/auth.py, decl: 7, sub: 0, line: 91 } |  |  | 0.604 |
| walker |  | 6811 | 7 | Code::CodeKey { rung: Decl, file: src/requests/auth.py, decl: 8, sub: 0, line: 93 } |  |  | 0.604 |
| ns | 6846 |  | 107 | `utils.py` module-level constants | 5.1 |  | 0.601 |
| ns | 6994 |  | 148 | `utils.py` function roster 1/3: proxies shim, lengths, netrc, encoders | 5.2 |  | 0.594 |
| walker |  | 7139 | 328 | Code::CodeKey { rung: Decl, file: src/requests/auth.py, decl: 15, sub: 0, line: 124 } |  |  | 0.609 |
| walker |  | 7146 | 7 | Code::CodeKey { rung: Decl, file: src/requests/auth.py, decl: 16, sub: 0, line: 136 } |  |  | 0.609 |
| walker |  | 7153 | 7 | Code::CodeKey { rung: Decl, file: src/requests/auth.py, decl: 17, sub: 0, line: 138 } |  |  | 0.609 |
| ns | 7161 |  | 167 | `utils.py` function roster 2/3: cookies, encoding detection, URI quoting, CIDR | 5.3 | 5.2 | 0.601 |
| ns | 7341 |  | 180 | `utils.py` function roster 3/3: proxy resolution, default headers, header validation | 5.4 | 5.3 | 0.593 |
| ns | 7602 |  | 261 | `structures.py`: `CaseInsensitiveDict` and `LookupDict` in full shape | 5.5 |  | 0.601 |
| ns | 7712 |  | 110 | `status_codes.py`: how `requests.codes` is built | 5.6 |  | 0.597 |
| walker |  | 7743 | 590 | Toml::Config { file: pyproject.toml } |  |  | 0.607 |
| walker |  | 7877 | 134 | Code::CodeKey { rung: Names, file: src/requests/api.py, decl: 0, sub: 0, line: 0 } |  |  | 0.617 |
| walker |  | 7912 | 35 | Code::CodeKey { rung: Decl, file: src/requests/api.py, decl: 1, sub: 0, line: 24 } |  |  | 0.617 |
| walker |  | 7952 | 40 | Code::CodeKey { rung: Decl, file: src/requests/api.py, decl: 2, sub: 0, line: 74 } |  |  | 0.617 |
| ns | 7954 |  | 242 | The five small modules: hooks, `_internal_utils`, compat, certs, packages | 5.7 |  | 0.609 |
| walker |  | 7992 | 40 | Code::CodeKey { rung: Decl, file: src/requests/api.py, decl: 6, sub: 0, line: 137 } |  |  | 0.609 |
| walker |  | 8032 | 40 | Code::CodeKey { rung: Decl, file: src/requests/api.py, decl: 7, sub: 0, line: 154 } |  |  | 0.609 |
| walker |  | 8096 | 64 | Code::CodeKey { rung: Decl, file: src/requests/api.py, decl: 5, sub: 0, line: 117 } |  |  | 0.609 |
| walker |  | 8111 | 15 | Code::CodeKey { rung: Body, file: src/requests/api.py, decl: 3, sub: 0, line: 90 } |  |  | 0.609 |
| walker |  | 8126 | 15 | Code::CodeKey { rung: Body, file: src/requests/api.py, decl: 8, sub: 0, line: 171 } |  |  | 0.609 |
| ns | 8131 |  | 177 | `help.py`: the bug-report payload | 5.8 |  | 0.602 |
| walker |  | 8144 | 18 | Code::CodeKey { rung: Body, file: src/requests/api.py, decl: 2, sub: 0, line: 74 } |  |  | 0.602 |
| walker |  | 8391 | 247 | Code::CodeKey { rung: Names, file: src/requests/utils.py, decl: 0, sub: 0, line: 0 } |  |  | 0.609 |
| walker |  | 8399 | 8 | Code::CodeKey { rung: Decl, file: src/requests/utils.py, decl: 14, sub: 0, line: 370 } |  |  | 0.609 |
| walker |  | 8409 | 10 | Code::CodeKey { rung: Decl, file: src/requests/utils.py, decl: 12, sub: 0, line: 328 } |  |  | 0.609 |
| walker |  | 8437 | 28 | Code::CodeKey { rung: Decl, file: src/requests/utils.py, decl: 6, sub: 0, line: 91 } |  |  | 0.612 |
| ns | 8464 |  | 333 | `resolve_redirects`: the rules the redirect loop enforces | 6.1 | 3.8 | 0.599 |
| walker |  | 8470 | 33 | Code::CodeKey { rung: Decl, file: src/requests/utils.py, decl: 9, sub: 0, line: 231 } |  |  | 0.599 |
| walker |  | 8507 | 37 | Code::CodeKey { rung: Decl, file: src/requests/utils.py, decl: 7, sub: 0, line: 149 } |  |  | 0.599 |
| walker |  | 8544 | 37 | Code::CodeKey { rung: Decl, file: src/requests/utils.py, decl: 13, sub: 0, line: 341 } |  |  | 0.599 |
| walker |  | 8557 | 13 | Code::CodeKey { rung: Doc, file: src/requests/utils.py, decl: 7, sub: 0, line: 149 } |  |  | 0.599 |
| walker |  | 8574 | 17 | Code::CodeKey { rung: Doc, file: src/requests/utils.py, decl: 10, sub: 0, line: 283 } |  |  | 0.599 |
| walker |  | 8731 | 157 | Code::CodeKey { rung: Names, file: src/requests/compat.py, decl: 0, sub: 0, line: 0 } |  |  | 0.600 |
| walker |  | 8743 | 12 | Code::CodeKey { rung: Doc, file: src/requests/compat.py, decl: 1, sub: 0, line: 37 } |  |  | 0.601 |
| ns | 8753 |  | 289 | `HTTPAdapter.send`: the urllib3 call and the `MaxRetryError` fan-out | 6.2 | 3.7 | 0.590 |
| walker |  | 8889 | 146 | Code::CodeKey { rung: Names, file: src/requests/models.py, decl: 0, sub: 0, line: 0 } |  |  | 0.594 |
| walker |  | 8948 | 59 | Code::CodeKey { rung: Decl, file: src/requests/models.py, decl: 13, sub: 0, line: 255 } |  |  | 0.594 |
| ns | 8955 |  | 202 | `HTTPAdapter.send`: the remaining `except` arms and the return | 6.3 | 6.2 | 0.586 |
| walker |  | 8984 | 36 | Code::CodeKey { rung: Decl, file: src/requests/models.py, decl: 14, sub: 0, line: 258 } |  |  | 0.586 |
| ns | 9043 |  | 88 | Makefile: the remaining targets (coverage, publish, docs) | 7.1 | 1.8 | 0.591 |
| walker |  | 9058 | 74 | Code::CodeKey { rung: Decl, file: src/requests/models.py, decl: 1, sub: 0, line: 96 } |  |  | 0.597 |
| walker |  | 9182 | 124 | Code::CodeKey { rung: Decl, file: src/requests/models.py, decl: 5, sub: 0, line: 109 } |  |  | 0.597 |
| walker |  | 9188 | 6 | Code::CodeKey { rung: Decl, file: src/requests/models.py, decl: 6, sub: 0, line: 112 } |  |  | 0.597 |
| walker |  | 9203 | 15 | Code::CodeKey { rung: Decl, file: src/requests/models.py, decl: 8, sub: 0, line: 137 } |  |  | 0.597 |
| walker |  | 9220 | 17 | Code::CodeKey { rung: Decl, file: src/requests/models.py, decl: 7, sub: 0, line: 133 } |  |  | 0.597 |
| walker |  | 9237 | 17 | Code::CodeKey { rung: Decl, file: src/requests/models.py, decl: 10, sub: 0, line: 147 } |  |  | 0.597 |
| ns | 9326 |  | 283 | pyproject: every remaining table, and the settings that change how you work here | 7.2 | 1.10 | 0.604 |
| walker |  | 9405 | 168 | Code::CodeKey { rung: Decl, file: src/requests/models.py, decl: 16, sub: 0, line: 283 } |  |  | 0.620 |
| walker |  | 9446 | 41 | Code::CodeKey { rung: Decl, file: src/requests/models.py, decl: 11, sub: 0, line: 151 } |  |  | 0.620 |
| ns | 9481 |  | 155 | `tests/conftest.py`: the fixture set every test builds on | 7.3 |  | 0.614 |
| walker |  | 9489 | 43 | Code::CodeKey { rung: Decl, file: src/requests/models.py, decl: 12, sub: 0, line: 183 } |  |  | 0.614 |
| walker |  | 9535 | 46 | Code::CodeKey { rung: Decl, file: src/requests/models.py, decl: 9, sub: 0, line: 141 } |  |  | 0.614 |
| ns | 9576 |  | 95 | `tests/test_requests.py`: top-level class roster | 7.4 |  | 0.611 |
| ns | 9758 |  | 182 | `tests/testserver/server.py`: the local socket server API (complete) | 7.5 |  | 0.605 |
| walker |  | 9857 | 322 | Code::CodeKey { rung: Decl, file: src/requests/models.py, decl: 20, sub: 0, line: 376 } |  |  | 0.623 |
| walker |  | 9865 | 8 | Code::CodeKey { rung: Decl, file: src/requests/models.py, decl: 26, sub: 0, line: 471 } |  |  | 0.625 |
| walker |  | 9898 | 33 | Code::CodeKey { rung: Decl, file: src/requests/models.py, decl: 32, sub: 0, line: 697 } |  |  | 0.625 |
| ns | 9915 |  | 157 | `docs/api.rst`: every section of the developer interface | 7.6 |  | 0.618 |
| walker |  | 9937 | 39 | Code::CodeKey { rung: Decl, file: src/requests/models.py, decl: 29, sub: 0, line: 574 } |  |  | 0.618 |
| ns | 9963 |  | 48 | `HISTORY.md`: the changelog's head | 7.7 |  | 0.617 |
| walker |  | 9977 | 40 | Code::CodeKey { rung: Decl, file: src/requests/models.py, decl: 27, sub: 0, line: 481 } |  |  | 0.617 |
