Score(3000)=0.715 I=0.909 C=0.563 ns_rows≤3K=17/49 grid(1000/1442/2080/3000/4327/6240/9000)=0.692/0.813/0.740/0.715/0.691/0.681/0.667

| source | ns_cum | walker_cum | marginal | descriptor | id | predecessor | Score(B=cum) |
|:-------|-------:|-----------:|---------:|:-----------|:---|:------------|-------------:|
| ns | 47 |  | 47 | Identity lede: what the library is, and its version | 1.1 |  | 0.000 |
| walker |  | 74 | 74 | Fs::DirListing { dir: . } |  |  | 0.000 |
| ns | 121 |  | 74 | Repository root listing (complete) | 1.2 |  | 0.671 |
| ns | 215 |  | 94 | `src/requests/` module roster (complete) | 1.3 |  | 0.454 |
| walker |  | 249 | 175 | Markdown::ReadmeHeadline { file: README.md } |  |  | 0.599 |
| walker |  | 283 | 34 | Fs::DirListing { dir: ext } |  |  | 0.599 |
| ns | 318 |  | 103 | `tests/` roster (complete, incl. `testserver/` and `certs/`) | 1.4 |  | 0.474 |
| walker |  | 341 | 58 | Toml::Identity { file: pyproject.toml } |  |  | 0.475 |
| walker |  | 395 | 54 | Fs::DirListing { dir: docs } |  |  | 0.479 |
| walker |  | 399 | 4 | Fs::DirListing { dir: docs/_templates } |  |  | 0.479 |
| walker |  | 408 | 9 | Fs::DirListing { dir: docs/_static } |  |  | 0.479 |
| walker |  | 418 | 10 | Fs::DirListing { dir: docs/dev } |  |  | 0.480 |
| walker |  | 432 | 14 | Fs::DirListing { dir: docs/_themes } |  |  | 0.480 |
| walker |  | 453 | 21 | Fs::DirListing { dir: docs/user } |  |  | 0.483 |
| ns | 465 |  | 147 | README: the canonical `requests.get(...)` doctest + every `##` heading | 1.5 | 1.1 | 0.497 |
| walker |  | 493 | 40 | Markdown::HeadingsOutline { file: README.md } |  |  | 0.555 |
| walker |  | 531 | 38 | Fs::DirListing { dir: docs/community } |  |  | 0.563 |
| ns | 583 |  | 118 | Full package metadata block (`__version__.py`) | 1.6 | 1.1 | 0.528 |
| walker |  | 625 | 94 | Fs::DirListing { dir: src/requests } |  |  | 0.720 |
| ns | 681 |  | 98 | CI: `.github/` and workflow file roster (complete) | 1.7 |  | 0.638 |
| ns | 761 |  | 80 | Makefile: install / test / CI targets | 1.8 |  | 0.611 |
| ns | 884 |  | 123 | `docs/` tree listing (complete) | 1.9 |  | 0.654 |
| walker |  | 981 | 356 | Plaintext::Whole { file: Makefile } |  |  | 0.692 |
| walker |  | 1139 | 158 | Code::CodeKey { rung: ModuleDoc, file: src/requests/__version__.py, decl: 0, sub: 0, line: 0 } |  |  | 0.801 |
| ns | 1186 |  | 302 | pyproject: build backend, metadata, runtime dependencies | 1.10 |  | 0.729 |
| walker |  | 1202 | 63 | Markdown::Section { file: README.md, section_index: 2, keeps_default_concavity: false } |  |  | 0.729 |
| walker |  | 1245 | 43 | Fs::DirListing { dir: .github } |  |  | 0.746 |
| walker |  | 1286 | 41 | Fs::DirListing { dir: .github/workflows } |  |  | 0.790 |
| walker |  | 1300 | 14 | Fs::DirListing { dir: .github/ISSUE_TEMPLATE } |  |  | 0.813 |
| ns | 1457 |  | 271 | `requests/__init__.py`: name → module re-export map | 2.1 |  | 0.741 |
| ns | 1531 |  | 74 | `api.py`: names of all eight module-level functions | 2.2 |  | 0.724 |
| walker |  | 1610 | 310 | Code::CodeKey { rung: Names, file: src/requests/__init__.py, decl: 0, sub: 0, line: 0 } |  |  | 0.791 |
| walker |  | 1657 | 47 | Code::CodeKey { rung: Decl, file: src/requests/__init__.py, decl: 1, sub: 0, line: 60 } |  |  | 0.791 |
| ns | 1884 |  | 353 | `exceptions.py`: the complete class hierarchy | 2.3 |  | 0.740 |
| walker |  | 1926 | 269 | Code::CodeKey { rung: Names, file: src/requests/__init__.py, decl: 0, sub: 1, line: 0 } |  |  | 0.764 |
| ns | 2060 |  | 176 | `Session` class declaration + docstring | 2.4 |  | 0.731 |
| walker |  | 2067 | 141 | Toml::Dependencies { file: pyproject.toml } |  |  | 0.740 |
| ns | 2331 |  | 271 | `Session` attribute set: typed fields + `__attrs__` | 2.5 | 2.4 | 0.697 |
| walker |  | 2417 | 350 | Code::CodeKey { rung: ModuleDoc, file: src/requests/__init__.py, decl: 0, sub: 0, line: 0 } |  |  | 0.697 |
| walker |  | 2494 | 77 | Fs::DirListing { dir: tests } |  |  | 0.734 |
| walker |  | 2506 | 12 | Fs::DirListing { dir: tests/testserver } |  |  | 0.745 |
| walker |  | 2520 | 14 | Fs::DirListing { dir: tests/certs } |  |  | 0.769 |
| walker |  | 2531 | 11 | Fs::DirListing { dir: tests/certs/mtls } |  |  | 0.769 |
| walker |  | 2545 | 14 | Fs::DirListing { dir: tests/certs/expired } |  |  | 0.769 |
| ns | 2619 |  | 288 | `Session` method roster (complete, with line coordinates) | 2.6 | 2.4 | 0.740 |
| walker |  | 2692 | 147 | Markdown::Section { file: README.md, section_index: 1, keeps_default_concavity: false } |  |  | 0.740 |
| walker |  | 2765 | 73 | Plaintext::DeclSurface { file: requirements-dev.txt } |  |  | 0.740 |
| walker |  | 2822 | 57 | Code::CodeKey { rung: Names, file: src/requests/sessions.py, decl: 0, sub: 0, line: 0 } |  |  | 0.740 |
| walker |  | 2853 | 31 | Code::CodeKey { rung: Decl, file: src/requests/sessions.py, decl: 1, sub: 0, line: 76 } |  |  | 0.740 |
| ns | 2870 |  | 251 | `Session.request`: the complete keyword signature | 2.7 | 2.6 | 0.715 |
| walker |  | 2906 | 53 | Code::CodeKey { rung: Decl, file: src/requests/sessions.py, decl: 2, sub: 0, line: 108 } |  |  | 0.715 |
| ns | 3053 |  | 183 | `Session.__init__`: every default value | 2.8 | 2.6 | 0.697 |
| walker |  | 3063 | 157 | Code::CodeKey { rung: Decl, file: src/requests/sessions.py, decl: 3, sub: 0, line: 127 } |  |  | 0.698 |
| walker |  | 3089 | 26 | Code::CodeKey { rung: Decl, file: src/requests/sessions.py, decl: 8, sub: 0, line: 309 } |  |  | 0.698 |
| walker |  | 3115 | 26 | Code::CodeKey { rung: Decl, file: src/requests/sessions.py, decl: 10, sub: 0, line: 370 } |  |  | 0.698 |
| walker |  | 3160 | 45 | Code::CodeKey { rung: Decl, file: src/requests/sessions.py, decl: 9, sub: 0, line: 334 } |  |  | 0.698 |
| walker |  | 3295 | 135 | Code::CodeKey { rung: Decl, file: src/requests/sessions.py, decl: 7, sub: 0, line: 186 } |  |  | 0.698 |
| ns | 3406 |  | 353 | `models.py` module map: constants, all five classes, `Request` fields | 3.1 |  | 0.668 |
| walker |  | 3568 | 273 | Code::CodeKey { rung: Decl, file: src/requests/sessions.py, decl: 11, sub: 0, line: 395 } |  |  | 0.724 |
| ns | 3727 |  | 321 | `PreparedRequest`: attributes + every `prepare_*` step | 3.2 | 3.1 | 0.700 |
| walker |  | 3753 | 185 | Code::CodeKey { rung: Decl, file: src/requests/sessions.py, decl: 11, sub: 1, line: 395 } |  |  | 0.709 |
| walker |  | 3796 | 43 | Code::CodeKey { rung: Decl, file: src/requests/sessions.py, decl: 21, sub: 0, line: 714 } |  |  | 0.709 |
| walker |  | 3839 | 43 | Code::CodeKey { rung: Decl, file: src/requests/sessions.py, decl: 22, sub: 0, line: 728 } |  |  | 0.709 |
| walker |  | 3897 | 58 | Code::CodeKey { rung: Decl, file: src/requests/sessions.py, decl: 17, sub: 0, line: 655 } |  |  | 0.709 |
| walker |  | 3969 | 72 | Code::CodeKey { rung: Decl, file: src/requests/sessions.py, decl: 20, sub: 0, line: 695 } |  |  | 0.709 |
| ns | 4030 |  | 303 | `Response` attribute set: typed fields + `__attrs__` | 3.3 | 3.1 | 0.677 |
| walker |  | 4138 | 169 | Code::CodeKey { rung: Decl, file: src/requests/sessions.py, decl: 11, sub: 2, line: 395 } |  |  | 0.703 |
| walker |  | 4219 | 81 | Code::CodeKey { rung: Decl, file: src/requests/sessions.py, decl: 25, sub: 0, line: 831 } |  |  | 0.703 |
| ns | 4312 |  | 282 | `Response`: `__init__`, dunders and every `@property` | 3.4 | 3.3 | 0.685 |
| ns | 4464 |  | 152 | `Response`: the remaining members (`iter_content` … `close`) | 3.5 | 3.4 | 0.675 |
| walker |  | 4470 | 251 | Code::CodeKey { rung: Decl, file: src/requests/sessions.py, decl: 16, sub: 0, line: 557 } |  |  | 0.705 |
| walker |  | 4646 | 176 | Markdown::Section { file: README.md, section_index: 3, keeps_default_concavity: false } |  |  | 0.705 |
| walker |  | 4662 | 16 | Code::CodeKey { rung: Doc, file: src/requests/sessions.py, decl: 27, sub: 0, line: 883 } |  |  | 0.705 |
| ns | 4679 |  | 215 | `adapters.py`: pool defaults + the `BaseAdapter` contract | 3.6 |  | 0.689 |
| walker |  | 4701 | 39 | Plaintext::DeclSurface { file: docs/requirements.txt } |  |  | 0.689 |
| ns | 5043 |  | 364 | `HTTPAdapter`: constructor knobs + complete method roster | 3.7 | 3.6 | 0.664 |
| walker |  | 5054 | 353 | Code::CodeKey { rung: Names, file: src/requests/exceptions.py, decl: 0, sub: 0, line: 0 } |  |  | 0.704 |
| walker |  | 5102 | 48 | Code::CodeKey { rung: Decl, file: src/requests/exceptions.py, decl: 4, sub: 0, line: 42 } |  |  | 0.704 |
| walker |  | 5158 | 56 | Code::CodeKey { rung: Decl, file: src/requests/exceptions.py, decl: 1, sub: 0, line: 20 } |  |  | 0.704 |
| walker |  | 5166 | 8 | Code::CodeKey { rung: Doc, file: src/requests/exceptions.py, decl: 15, sub: 0, line: 106 } |  |  | 0.704 |
| walker |  | 5175 | 9 | Code::CodeKey { rung: Doc, file: src/requests/exceptions.py, decl: 3, sub: 0, line: 38 } |  |  | 0.704 |
| walker |  | 5184 | 9 | Code::CodeKey { rung: Doc, file: src/requests/exceptions.py, decl: 7, sub: 0, line: 66 } |  |  | 0.704 |
| walker |  | 5193 | 9 | Code::CodeKey { rung: Doc, file: src/requests/exceptions.py, decl: 8, sub: 0, line: 70 } |  |  | 0.704 |
| walker |  | 5202 | 9 | Code::CodeKey { rung: Doc, file: src/requests/exceptions.py, decl: 9, sub: 0, line: 74 } |  |  | 0.704 |
| walker |  | 5211 | 9 | Code::CodeKey { rung: Doc, file: src/requests/exceptions.py, decl: 10, sub: 0, line: 78 } |  |  | 0.704 |
| walker |  | 5220 | 9 | Code::CodeKey { rung: Doc, file: src/requests/exceptions.py, decl: 24, sub: 0, line: 142 } |  |  | 0.704 |
| walker |  | 5229 | 9 | Code::CodeKey { rung: Doc, file: src/requests/exceptions.py, decl: 26, sub: 0, line: 153 } |  |  | 0.704 |
| walker |  | 5239 | 10 | Code::CodeKey { rung: Doc, file: src/requests/exceptions.py, decl: 22, sub: 0, line: 134 } |  |  | 0.704 |
| walker |  | 5250 | 11 | Code::CodeKey { rung: Doc, file: src/requests/exceptions.py, decl: 4, sub: 0, line: 42 } |  |  | 0.704 |
| walker |  | 5261 | 11 | Code::CodeKey { rung: Doc, file: src/requests/exceptions.py, decl: 18, sub: 0, line: 118 } |  |  | 0.704 |
| walker |  | 5272 | 11 | Code::CodeKey { rung: Doc, file: src/requests/exceptions.py, decl: 20, sub: 0, line: 126 } |  |  | 0.704 |
| walker |  | 5284 | 12 | Code::CodeKey { rung: Doc, file: src/requests/exceptions.py, decl: 19, sub: 0, line: 122 } |  |  | 0.704 |
| walker |  | 5297 | 13 | Code::CodeKey { rung: Doc, file: src/requests/exceptions.py, decl: 23, sub: 0, line: 138 } |  |  | 0.704 |
| walker |  | 5311 | 14 | Code::CodeKey { rung: Doc, file: src/requests/exceptions.py, decl: 14, sub: 0, line: 102 } |  |  | 0.704 |
| ns | 5312 |  | 269 | `SessionRedirectMixin` + session-module merge helpers | 3.8 |  | 0.703 |
| walker |  | 5325 | 14 | Code::CodeKey { rung: Doc, file: src/requests/exceptions.py, decl: 17, sub: 0, line: 114 } |  |  | 0.703 |
| walker |  | 5339 | 14 | Code::CodeKey { rung: Doc, file: src/requests/exceptions.py, decl: 28, sub: 0, line: 161 } |  |  | 0.703 |
| walker |  | 5355 | 16 | Code::CodeKey { rung: Doc, file: src/requests/exceptions.py, decl: 21, sub: 0, line: 130 } |  |  | 0.703 |
| walker |  | 5372 | 17 | Code::CodeKey { rung: Doc, file: src/requests/exceptions.py, decl: 25, sub: 0, line: 146 } |  |  | 0.703 |
| walker |  | 5390 | 18 | Code::CodeKey { rung: Doc, file: src/requests/exceptions.py, decl: 13, sub: 0, line: 98 } |  |  | 0.703 |
| walker |  | 5408 | 18 | Code::CodeKey { rung: Doc, file: src/requests/exceptions.py, decl: 16, sub: 0, line: 110 } |  |  | 0.703 |
| walker |  | 5426 | 18 | Code::CodeKey { rung: Doc, file: src/requests/sessions.py, decl: 6, sub: 0, line: 154 } |  |  | 0.707 |
| walker |  | 5445 | 19 | Code::CodeKey { rung: Doc, file: src/requests/exceptions.py, decl: 2, sub: 0, line: 28 } |  |  | 0.707 |
| walker |  | 5464 | 19 | Code::CodeKey { rung: Doc, file: src/requests/exceptions.py, decl: 27, sub: 0, line: 157 } |  |  | 0.707 |
| walker |  | 5483 | 19 | Code::CodeKey { rung: Doc, file: src/requests/sessions.py, decl: 7, sub: 0, line: 186 } |  |  | 0.711 |
| ns | 5548 |  | 236 | `cookies.py` module map: classes + free functions | 3.9 |  | 0.700 |
| walker |  | 5593 | 110 | Plaintext::DeclSurface { file: tox.ini } |  |  | 0.700 |
| walker |  | 5651 | 58 | Plaintext::Whole { file: tox.ini } |  |  | 0.700 |
| walker |  | 5671 | 20 | Code::CodeKey { rung: Doc, file: src/requests/sessions.py, decl: 5, sub: 0, line: 134 } |  |  | 0.704 |
| walker |  | 5817 | 146 | Code::CodeKey { rung: Names, file: src/requests/models.py, decl: 0, sub: 0, line: 0 } |  |  | 0.709 |
| ns | 5856 |  | 308 | `auth.py`: every auth handler and its methods | 3.10 |  | 0.693 |
| walker |  | 5876 | 59 | Code::CodeKey { rung: Decl, file: src/requests/models.py, decl: 13, sub: 0, line: 255 } |  |  | 0.693 |
| walker |  | 5912 | 36 | Code::CodeKey { rung: Decl, file: src/requests/models.py, decl: 14, sub: 0, line: 258 } |  |  | 0.693 |
| walker |  | 5986 | 74 | Code::CodeKey { rung: Decl, file: src/requests/models.py, decl: 1, sub: 0, line: 96 } |  |  | 0.700 |
| walker |  | 6110 | 124 | Code::CodeKey { rung: Decl, file: src/requests/models.py, decl: 5, sub: 0, line: 109 } |  |  | 0.700 |
| walker |  | 6116 | 6 | Code::CodeKey { rung: Decl, file: src/requests/models.py, decl: 6, sub: 0, line: 112 } |  |  | 0.700 |
| walker |  | 6131 | 15 | Code::CodeKey { rung: Decl, file: src/requests/models.py, decl: 8, sub: 0, line: 137 } |  |  | 0.700 |
| walker |  | 6148 | 17 | Code::CodeKey { rung: Decl, file: src/requests/models.py, decl: 7, sub: 0, line: 133 } |  |  | 0.700 |
| walker |  | 6165 | 17 | Code::CodeKey { rung: Decl, file: src/requests/models.py, decl: 10, sub: 0, line: 147 } |  |  | 0.700 |
| walker |  | 6206 | 41 | Code::CodeKey { rung: Decl, file: src/requests/models.py, decl: 11, sub: 0, line: 151 } |  |  | 0.700 |
| ns | 6228 |  | 372 | `_types.py`: public type-alias roster | 4.1 |  | 0.681 |
| walker |  | 6249 | 43 | Code::CodeKey { rung: Decl, file: src/requests/models.py, decl: 12, sub: 0, line: 183 } |  |  | 0.681 |
| walker |  | 6295 | 46 | Code::CodeKey { rung: Decl, file: src/requests/models.py, decl: 9, sub: 0, line: 141 } |  |  | 0.681 |
| walker |  | 6463 | 168 | Code::CodeKey { rung: Decl, file: src/requests/models.py, decl: 16, sub: 0, line: 283 } |  |  | 0.702 |
| ns | 6539 |  | 311 | `_types.py`: the `Unpack` kwargs TypedDicts | 4.2 | 4.1 | 0.685 |
| walker |  | 6634 | 171 | Code::CodeKey { rung: Decl, file: src/requests/models.py, decl: 17, sub: 0, line: 321 } |  |  | 0.685 |
| ns | 6738 |  | 199 | Environment-driven settings: the `merge_environment_settings` core | 4.3 | 2.6 | 0.675 |
| ns | 6845 |  | 107 | `utils.py` module-level constants | 5.1 |  | 0.671 |
| walker |  | 6956 | 322 | Code::CodeKey { rung: Decl, file: src/requests/models.py, decl: 20, sub: 0, line: 376 } |  |  | 0.695 |
| walker |  | 6964 | 8 | Code::CodeKey { rung: Decl, file: src/requests/models.py, decl: 26, sub: 0, line: 471 } |  |  | 0.696 |
| ns | 6993 |  | 148 | `utils.py` function roster 1/3: proxies shim, lengths, netrc, encoders | 5.2 |  | 0.688 |
| walker |  | 6997 | 33 | Code::CodeKey { rung: Decl, file: src/requests/models.py, decl: 32, sub: 0, line: 697 } |  |  | 0.688 |
| walker |  | 7036 | 39 | Code::CodeKey { rung: Decl, file: src/requests/models.py, decl: 29, sub: 0, line: 574 } |  |  | 0.688 |
| walker |  | 7076 | 40 | Code::CodeKey { rung: Decl, file: src/requests/models.py, decl: 27, sub: 0, line: 481 } |  |  | 0.688 |
| walker |  | 7117 | 41 | Code::CodeKey { rung: Decl, file: src/requests/models.py, decl: 31, sub: 0, line: 668 } |  |  | 0.688 |
| ns | 7160 |  | 167 | `utils.py` function roster 2/3: cookies, encoding detection, URI quoting, CIDR | 5.3 | 5.2 | 0.679 |
| walker |  | 7293 | 176 | Code::CodeKey { rung: Decl, file: src/requests/models.py, decl: 22, sub: 0, line: 422 } |  |  | 0.679 |
| ns | 7340 |  | 180 | `utils.py` function roster 3/3: proxy resolution, default headers, header validation | 5.4 | 5.3 | 0.670 |
| walker |  | 7554 | 261 | Code::CodeKey { rung: Decl, file: src/requests/models.py, decl: 34, sub: 0, line: 730 } |  |  | 0.696 |
| ns | 7601 |  | 261 | `structures.py`: `CaseInsensitiveDict` and `LookupDict` in full shape | 5.5 |  | 0.684 |
| ns | 7711 |  | 110 | `status_codes.py`: how `requests.codes` is built | 5.6 |  | 0.679 |
| walker |  | 7745 | 191 | Code::CodeKey { rung: Decl, file: src/requests/models.py, decl: 34, sub: 1, line: 730 } |  |  | 0.687 |
| walker |  | 7753 | 8 | Code::CodeKey { rung: Decl, file: src/requests/models.py, decl: 44, sub: 0, line: 859 } |  |  | 0.688 |
| walker |  | 7761 | 8 | Code::CodeKey { rung: Decl, file: src/requests/models.py, decl: 45, sub: 0, line: 874 } |  |  | 0.689 |
| ns | 7953 |  | 242 | The five small modules: hooks, `_internal_utils`, compat, certs, packages | 5.7 |  | 0.679 |
| walker |  | 7981 | 220 | Code::CodeKey { rung: Decl, file: src/requests/models.py, decl: 34, sub: 2, line: 730 } |  |  | 0.693 |
| walker |  | 7989 | 8 | Code::CodeKey { rung: Decl, file: src/requests/models.py, decl: 46, sub: 0, line: 881 } |  |  | 0.694 |
| walker |  | 7997 | 8 | Code::CodeKey { rung: Decl, file: src/requests/models.py, decl: 47, sub: 0, line: 889 } |  |  | 0.696 |
| walker |  | 8005 | 8 | Code::CodeKey { rung: Decl, file: src/requests/models.py, decl: 48, sub: 0, line: 894 } |  |  | 0.697 |
| walker |  | 8014 | 9 | Code::CodeKey { rung: Decl, file: src/requests/models.py, decl: 55, sub: 0, line: 1030 } |  |  | 0.698 |
| walker |  | 8023 | 9 | Code::CodeKey { rung: Decl, file: src/requests/models.py, decl: 56, sub: 0, line: 1049 } |  |  | 0.700 |
| walker |  | 8032 | 9 | Code::CodeKey { rung: Decl, file: src/requests/models.py, decl: 58, sub: 0, line: 1122 } |  |  | 0.701 |
| walker |  | 8068 | 36 | Code::CodeKey { rung: Decl, file: src/requests/models.py, decl: 51, sub: 0, line: 912 } |  |  | 0.701 |
| walker |  | 8113 | 45 | Code::CodeKey { rung: Decl, file: src/requests/models.py, decl: 50, sub: 0, line: 908 } |  |  | 0.701 |
| ns | 8130 |  | 177 | `help.py`: the bug-report payload | 5.8 |  | 0.693 |
| walker |  | 8159 | 46 | Code::CodeKey { rung: Decl, file: src/requests/models.py, decl: 49, sub: 0, line: 904 } |  |  | 0.693 |
| walker |  | 8220 | 61 | Code::CodeKey { rung: Decl, file: src/requests/models.py, decl: 54, sub: 0, line: 990 } |  |  | 0.693 |
| walker |  | 8291 | 71 | Code::CodeKey { rung: Decl, file: src/requests/models.py, decl: 52, sub: 0, line: 975 } |  |  | 0.693 |
| walker |  | 8364 | 73 | Code::CodeKey { rung: Decl, file: src/requests/models.py, decl: 53, sub: 0, line: 982 } |  |  | 0.693 |
| walker |  | 8376 | 12 | Code::CodeKey { rung: Doc, file: src/requests/models.py, decl: 14, sub: 0, line: 258 } |  |  | 0.693 |
| walker |  | 8388 | 12 | Code::CodeKey { rung: Doc, file: src/requests/models.py, decl: 33, sub: 0, line: 720 } |  |  | 0.693 |
| walker |  | 8401 | 13 | Code::CodeKey { rung: Doc, file: src/requests/models.py, decl: 6, sub: 0, line: 112 } |  |  | 0.693 |
| walker |  | 8414 | 13 | Code::CodeKey { rung: Doc, file: src/requests/models.py, decl: 25, sub: 0, line: 465 } |  |  | 0.693 |
| walker |  | 8427 | 13 | Code::CodeKey { rung: Doc, file: src/requests/models.py, decl: 27, sub: 0, line: 481 } |  |  | 0.693 |
| walker |  | 8440 | 13 | Code::CodeKey { rung: Doc, file: src/requests/models.py, decl: 28, sub: 0, line: 563 } |  |  | 0.693 |
| walker |  | 8454 | 14 | Code::CodeKey { rung: Doc, file: src/requests/models.py, decl: 29, sub: 0, line: 574 } |  |  | 0.693 |
| ns | 8463 |  | 333 | `resolve_redirects`: the rules the redirect loop enforces | 6.1 | 3.8 | 0.679 |
| walker |  | 8468 | 14 | Code::CodeKey { rung: Doc, file: src/requests/models.py, decl: 31, sub: 0, line: 668 } |  |  | 0.679 |
| walker |  | 8483 | 15 | Code::CodeKey { rung: Doc, file: src/requests/models.py, decl: 55, sub: 0, line: 1030 } |  |  | 0.679 |
| walker |  | 8499 | 16 | Code::CodeKey { rung: Doc, file: src/requests/models.py, decl: 22, sub: 0, line: 422 } |  |  | 0.679 |
| walker |  | 8515 | 16 | Code::CodeKey { rung: Doc, file: src/requests/models.py, decl: 43, sub: 0, line: 855 } |  |  | 0.679 |
| walker |  | 8532 | 17 | Code::CodeKey { rung: Doc, file: src/requests/models.py, decl: 30, sub: 0, line: 652 } |  |  | 0.679 |
| walker |  | 8550 | 18 | Code::CodeKey { rung: Doc, file: src/requests/models.py, decl: 46, sub: 0, line: 881 } |  |  | 0.679 |
| walker |  | 8568 | 18 | Code::CodeKey { rung: Doc, file: src/requests/models.py, decl: 59, sub: 0, line: 1140 } |  |  | 0.679 |
| walker |  | 8587 | 19 | Code::CodeKey { rung: Doc, file: src/requests/models.py, decl: 58, sub: 0, line: 1122 } |  |  | 0.679 |
| walker |  | 8637 | 50 | Code::CodeKey { rung: Names, file: src/requests/status_codes.py, decl: 0, sub: 0, line: 0 } |  |  | 0.680 |
| ns | 8752 |  | 289 | `HTTPAdapter.send`: the urllib3 call and the `MaxRetryError` fan-out | 6.2 | 3.7 | 0.667 |
| walker |  | 8771 | 134 | Code::CodeKey { rung: Names, file: src/requests/api.py, decl: 0, sub: 0, line: 0 } |  |  | 0.676 |
| walker |  | 8806 | 35 | Code::CodeKey { rung: Decl, file: src/requests/api.py, decl: 1, sub: 0, line: 24 } |  |  | 0.676 |
| walker |  | 8846 | 40 | Code::CodeKey { rung: Decl, file: src/requests/api.py, decl: 2, sub: 0, line: 74 } |  |  | 0.676 |
| walker |  | 8886 | 40 | Code::CodeKey { rung: Decl, file: src/requests/api.py, decl: 6, sub: 0, line: 137 } |  |  | 0.676 |
| walker |  | 8926 | 40 | Code::CodeKey { rung: Decl, file: src/requests/api.py, decl: 7, sub: 0, line: 154 } |  |  | 0.676 |
| ns | 8954 |  | 202 | `HTTPAdapter.send`: the remaining `except` arms and the return | 6.3 | 6.2 | 0.667 |
| walker |  | 8990 | 64 | Code::CodeKey { rung: Decl, file: src/requests/api.py, decl: 5, sub: 0, line: 117 } |  |  | 0.667 |
| walker |  | 9003 | 13 | Code::CodeKey { rung: ModuleDoc, file: tests/__init__.py, decl: 0, sub: 0, line: 0 } |  |  | 0.667 |
| walker |  | 9025 | 22 | Code::CodeKey { rung: Doc, file: src/requests/models.py, decl: 48, sub: 0, line: 894 } |  |  | 0.667 |
| ns | 9042 |  | 88 | Makefile: the remaining targets (coverage, publish, docs) | 7.1 | 1.8 | 0.671 |
| walker |  | 9049 | 24 | Code::CodeKey { rung: Doc, file: src/requests/models.py, decl: 19, sub: 0, line: 358 } |  |  | 0.671 |
| walker |  | 9073 | 24 | Code::CodeKey { rung: Doc, file: src/requests/models.py, decl: 47, sub: 0, line: 889 } |  |  | 0.671 |
| walker |  | 9109 | 36 | Code::CodeKey { rung: Names, file: src/requests/help.py, decl: 0, sub: 0, line: 0 } |  |  | 0.671 |
| walker |  | 9122 | 13 | Code::CodeKey { rung: Doc, file: src/requests/help.py, decl: 2, sub: 0, line: 69 } |  |  | 0.671 |
| walker |  | 9136 | 14 | Code::CodeKey { rung: Doc, file: src/requests/help.py, decl: 3, sub: 0, line: 128 } |  |  | 0.671 |
| walker |  | 9225 | 89 | Code::CodeKey { rung: Names, file: src/requests/adapters.py, decl: 0, sub: 0, line: 0 } |  |  | 0.672 |
| walker |  | 9265 | 40 | Code::CodeKey { rung: Decl, file: src/requests/adapters.py, decl: 6, sub: 0, line: 122 } |  |  | 0.674 |
| ns | 9325 |  | 283 | pyproject: every remaining table, and the settings that change how you work here | 7.2 | 1.10 | 0.661 |
| walker |  | 9335 | 70 | Code::CodeKey { rung: Decl, file: src/requests/adapters.py, decl: 5, sub: 0, line: 85 } |  |  | 0.661 |
| walker |  | 9432 | 97 | Code::CodeKey { rung: Decl, file: src/requests/adapters.py, decl: 8, sub: 0, line: 128 } |  |  | 0.670 |
| ns | 9480 |  | 155 | `tests/conftest.py`: the fixture set every test builds on | 7.3 |  | 0.665 |
| ns | 9575 |  | 95 | `tests/test_requests.py`: top-level class roster | 7.4 |  | 0.661 |
| ns | 9757 |  | 182 | `tests/testserver/server.py`: the local socket server API (complete) | 7.5 |  | 0.655 |
| walker |  | 9838 | 406 | Code::CodeKey { rung: Decl, file: src/requests/adapters.py, decl: 10, sub: 0, line: 158 } |  |  | 0.661 |
| walker |  | 9869 | 31 | Code::CodeKey { rung: Decl, file: src/requests/adapters.py, decl: 22, sub: 0, line: 565 } |  |  | 0.661 |
| walker |  | 9903 | 34 | Code::CodeKey { rung: Decl, file: src/requests/adapters.py, decl: 20, sub: 0, line: 512 } |  |  | 0.661 |
| ns | 9914 |  | 157 | `docs/api.rst`: every section of the developer interface | 7.6 |  | 0.654 |
| walker |  | 9942 | 39 | Code::CodeKey { rung: Decl, file: src/requests/adapters.py, decl: 16, sub: 0, line: 307 } |  |  | 0.654 |
| ns | 9962 |  | 48 | `HISTORY.md`: the changelog's head | 7.7 |  | 0.653 |
| walker |  | 9990 | 48 | Code::CodeKey { rung: Decl, file: src/requests/adapters.py, decl: 18, sub: 0, line: 403 } |  |  | 0.653 |
| walker |  | 9997 | 7 | Code::CodeKey { rung: Decl, file: src/requests/adapters.py, decl: 14, sub: 0, line: 239 } |  |  | 0.653 |
