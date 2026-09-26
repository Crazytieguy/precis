Score(3000)=0.715 I=0.908 C=0.563 ns_rows≤3K=17/49 grid(1000/1442/2080/3000/4327/6240/9000)=0.692/0.813/0.740/0.715/0.660/0.661/0.651

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
| ns | 2870 |  | 251 | `Session.request`: the complete keyword signature | 2.7 | 2.6 | 0.715 |
| walker |  | 2941 | 176 | Markdown::Section { file: README.md, section_index: 3, keeps_default_concavity: false } |  |  | 0.715 |
| walker |  | 2980 | 39 | Plaintext::DeclSurface { file: docs/requirements.txt } |  |  | 0.715 |
| ns | 3053 |  | 183 | `Session.__init__`: every default value | 2.8 | 2.6 | 0.697 |
| walker |  | 3090 | 110 | Plaintext::DeclSurface { file: tox.ini } |  |  | 0.697 |
| walker |  | 3148 | 58 | Plaintext::Whole { file: tox.ini } |  |  | 0.697 |
| walker |  | 3294 | 146 | Code::CodeKey { rung: Names, file: src/requests/models.py, decl: 0, sub: 0, line: 0 } |  |  | 0.697 |
| walker |  | 3353 | 59 | Code::CodeKey { rung: Decl, file: src/requests/models.py, decl: 13, sub: 0, line: 255 } |  |  | 0.697 |
| walker |  | 3389 | 36 | Code::CodeKey { rung: Decl, file: src/requests/models.py, decl: 14, sub: 0, line: 258 } |  |  | 0.697 |
| ns | 3406 |  | 353 | `models.py` module map: constants, all five classes, `Request` fields | 3.1 |  | 0.674 |
| walker |  | 3463 | 74 | Code::CodeKey { rung: Decl, file: src/requests/models.py, decl: 1, sub: 0, line: 96 } |  |  | 0.685 |
| walker |  | 3587 | 124 | Code::CodeKey { rung: Decl, file: src/requests/models.py, decl: 5, sub: 0, line: 109 } |  |  | 0.685 |
| walker |  | 3593 | 6 | Code::CodeKey { rung: Decl, file: src/requests/models.py, decl: 6, sub: 0, line: 112 } |  |  | 0.685 |
| walker |  | 3608 | 15 | Code::CodeKey { rung: Decl, file: src/requests/models.py, decl: 8, sub: 0, line: 137 } |  |  | 0.685 |
| walker |  | 3625 | 17 | Code::CodeKey { rung: Decl, file: src/requests/models.py, decl: 7, sub: 0, line: 133 } |  |  | 0.685 |
| walker |  | 3642 | 17 | Code::CodeKey { rung: Decl, file: src/requests/models.py, decl: 10, sub: 0, line: 147 } |  |  | 0.685 |
| walker |  | 3683 | 41 | Code::CodeKey { rung: Decl, file: src/requests/models.py, decl: 11, sub: 0, line: 151 } |  |  | 0.685 |
| walker |  | 3726 | 43 | Code::CodeKey { rung: Decl, file: src/requests/models.py, decl: 12, sub: 0, line: 183 } |  |  | 0.685 |
| ns | 3727 |  | 321 | `PreparedRequest`: attributes + every `prepare_*` step | 3.2 | 3.1 | 0.663 |
| walker |  | 3772 | 46 | Code::CodeKey { rung: Decl, file: src/requests/models.py, decl: 9, sub: 0, line: 141 } |  |  | 0.663 |
| walker |  | 3940 | 168 | Code::CodeKey { rung: Decl, file: src/requests/models.py, decl: 16, sub: 0, line: 283 } |  |  | 0.693 |
| ns | 4030 |  | 303 | `Response` attribute set: typed fields + `__attrs__` | 3.3 | 3.1 | 0.662 |
| walker |  | 4111 | 171 | Code::CodeKey { rung: Decl, file: src/requests/models.py, decl: 17, sub: 0, line: 321 } |  |  | 0.662 |
| ns | 4312 |  | 282 | `Response`: `__init__`, dunders and every `@property` | 3.4 | 3.3 | 0.645 |
| walker |  | 4433 | 322 | Code::CodeKey { rung: Decl, file: src/requests/models.py, decl: 20, sub: 0, line: 376 } |  |  | 0.679 |
| walker |  | 4441 | 8 | Code::CodeKey { rung: Decl, file: src/requests/models.py, decl: 26, sub: 0, line: 471 } |  |  | 0.681 |
| ns | 4464 |  | 152 | `Response`: the remaining members (`iter_content` … `close`) | 3.5 | 3.4 | 0.671 |
| walker |  | 4474 | 33 | Code::CodeKey { rung: Decl, file: src/requests/models.py, decl: 32, sub: 0, line: 697 } |  |  | 0.671 |
| walker |  | 4513 | 39 | Code::CodeKey { rung: Decl, file: src/requests/models.py, decl: 29, sub: 0, line: 574 } |  |  | 0.671 |
| walker |  | 4553 | 40 | Code::CodeKey { rung: Decl, file: src/requests/models.py, decl: 27, sub: 0, line: 481 } |  |  | 0.671 |
| walker |  | 4594 | 41 | Code::CodeKey { rung: Decl, file: src/requests/models.py, decl: 31, sub: 0, line: 668 } |  |  | 0.671 |
| ns | 4679 |  | 215 | `adapters.py`: pool defaults + the `BaseAdapter` contract | 3.6 |  | 0.656 |
| walker |  | 4770 | 176 | Code::CodeKey { rung: Decl, file: src/requests/models.py, decl: 22, sub: 0, line: 422 } |  |  | 0.656 |
| walker |  | 5031 | 261 | Code::CodeKey { rung: Decl, file: src/requests/models.py, decl: 34, sub: 0, line: 730 } |  |  | 0.692 |
| ns | 5043 |  | 364 | `HTTPAdapter`: constructor knobs + complete method roster | 3.7 | 3.6 | 0.667 |
| walker |  | 5222 | 191 | Code::CodeKey { rung: Decl, file: src/requests/models.py, decl: 34, sub: 1, line: 730 } |  |  | 0.678 |
| walker |  | 5230 | 8 | Code::CodeKey { rung: Decl, file: src/requests/models.py, decl: 44, sub: 0, line: 859 } |  |  | 0.679 |
| walker |  | 5238 | 8 | Code::CodeKey { rung: Decl, file: src/requests/models.py, decl: 45, sub: 0, line: 874 } |  |  | 0.680 |
| ns | 5312 |  | 269 | `SessionRedirectMixin` + session-module merge helpers | 3.8 |  | 0.665 |
| walker |  | 5458 | 220 | Code::CodeKey { rung: Decl, file: src/requests/models.py, decl: 34, sub: 2, line: 730 } |  |  | 0.683 |
| walker |  | 5466 | 8 | Code::CodeKey { rung: Decl, file: src/requests/models.py, decl: 46, sub: 0, line: 881 } |  |  | 0.685 |
| walker |  | 5474 | 8 | Code::CodeKey { rung: Decl, file: src/requests/models.py, decl: 47, sub: 0, line: 889 } |  |  | 0.687 |
| walker |  | 5482 | 8 | Code::CodeKey { rung: Decl, file: src/requests/models.py, decl: 48, sub: 0, line: 894 } |  |  | 0.689 |
| walker |  | 5491 | 9 | Code::CodeKey { rung: Decl, file: src/requests/models.py, decl: 55, sub: 0, line: 1030 } |  |  | 0.690 |
| walker |  | 5500 | 9 | Code::CodeKey { rung: Decl, file: src/requests/models.py, decl: 56, sub: 0, line: 1049 } |  |  | 0.692 |
| walker |  | 5509 | 9 | Code::CodeKey { rung: Decl, file: src/requests/models.py, decl: 58, sub: 0, line: 1122 } |  |  | 0.694 |
| walker |  | 5545 | 36 | Code::CodeKey { rung: Decl, file: src/requests/models.py, decl: 51, sub: 0, line: 912 } |  |  | 0.694 |
| ns | 5548 |  | 236 | `cookies.py` module map: classes + free functions | 3.9 |  | 0.683 |
| walker |  | 5590 | 45 | Code::CodeKey { rung: Decl, file: src/requests/models.py, decl: 50, sub: 0, line: 908 } |  |  | 0.683 |
| walker |  | 5636 | 46 | Code::CodeKey { rung: Decl, file: src/requests/models.py, decl: 49, sub: 0, line: 904 } |  |  | 0.683 |
| walker |  | 5697 | 61 | Code::CodeKey { rung: Decl, file: src/requests/models.py, decl: 54, sub: 0, line: 990 } |  |  | 0.683 |
| walker |  | 5768 | 71 | Code::CodeKey { rung: Decl, file: src/requests/models.py, decl: 52, sub: 0, line: 975 } |  |  | 0.683 |
| walker |  | 5841 | 73 | Code::CodeKey { rung: Decl, file: src/requests/models.py, decl: 53, sub: 0, line: 982 } |  |  | 0.683 |
| walker |  | 5853 | 12 | Code::CodeKey { rung: Doc, file: src/requests/models.py, decl: 14, sub: 0, line: 258 } |  |  | 0.683 |
| ns | 5856 |  | 308 | `auth.py`: every auth handler and its methods | 3.10 |  | 0.668 |
| walker |  | 5865 | 12 | Code::CodeKey { rung: Doc, file: src/requests/models.py, decl: 33, sub: 0, line: 720 } |  |  | 0.668 |
| walker |  | 5878 | 13 | Code::CodeKey { rung: Doc, file: src/requests/models.py, decl: 6, sub: 0, line: 112 } |  |  | 0.668 |
| walker |  | 5891 | 13 | Code::CodeKey { rung: Doc, file: src/requests/models.py, decl: 25, sub: 0, line: 465 } |  |  | 0.668 |
| walker |  | 5904 | 13 | Code::CodeKey { rung: Doc, file: src/requests/models.py, decl: 27, sub: 0, line: 481 } |  |  | 0.668 |
| walker |  | 5917 | 13 | Code::CodeKey { rung: Doc, file: src/requests/models.py, decl: 28, sub: 0, line: 563 } |  |  | 0.668 |
| walker |  | 5931 | 14 | Code::CodeKey { rung: Doc, file: src/requests/models.py, decl: 29, sub: 0, line: 574 } |  |  | 0.668 |
| walker |  | 5945 | 14 | Code::CodeKey { rung: Doc, file: src/requests/models.py, decl: 31, sub: 0, line: 668 } |  |  | 0.668 |
| walker |  | 6002 | 57 | Code::CodeKey { rung: Names, file: src/requests/sessions.py, decl: 0, sub: 0, line: 0 } |  |  | 0.669 |
| walker |  | 6033 | 31 | Code::CodeKey { rung: Decl, file: src/requests/sessions.py, decl: 1, sub: 0, line: 76 } |  |  | 0.669 |
| walker |  | 6086 | 53 | Code::CodeKey { rung: Decl, file: src/requests/sessions.py, decl: 2, sub: 0, line: 108 } |  |  | 0.669 |
| ns | 6228 |  | 372 | `_types.py`: public type-alias roster | 4.1 |  | 0.651 |
| walker |  | 6243 | 157 | Code::CodeKey { rung: Decl, file: src/requests/sessions.py, decl: 3, sub: 0, line: 127 } |  |  | 0.662 |
| walker |  | 6269 | 26 | Code::CodeKey { rung: Decl, file: src/requests/sessions.py, decl: 8, sub: 0, line: 309 } |  |  | 0.662 |
| walker |  | 6295 | 26 | Code::CodeKey { rung: Decl, file: src/requests/sessions.py, decl: 10, sub: 0, line: 370 } |  |  | 0.662 |
| walker |  | 6340 | 45 | Code::CodeKey { rung: Decl, file: src/requests/sessions.py, decl: 9, sub: 0, line: 334 } |  |  | 0.662 |
| walker |  | 6475 | 135 | Code::CodeKey { rung: Decl, file: src/requests/sessions.py, decl: 7, sub: 0, line: 186 } |  |  | 0.665 |
| ns | 6539 |  | 311 | `_types.py`: the `Unpack` kwargs TypedDicts | 4.2 | 4.1 | 0.649 |
| ns | 6738 |  | 199 | Environment-driven settings: the `merge_environment_settings` core | 4.3 | 2.6 | 0.639 |
| walker |  | 6748 | 273 | Code::CodeKey { rung: Decl, file: src/requests/sessions.py, decl: 11, sub: 0, line: 395 } |  |  | 0.674 |
| ns | 6845 |  | 107 | `utils.py` module-level constants | 5.1 |  | 0.670 |
| walker |  | 6933 | 185 | Code::CodeKey { rung: Decl, file: src/requests/sessions.py, decl: 11, sub: 1, line: 395 } |  |  | 0.676 |
| walker |  | 6976 | 43 | Code::CodeKey { rung: Decl, file: src/requests/sessions.py, decl: 21, sub: 0, line: 714 } |  |  | 0.676 |
| ns | 6993 |  | 148 | `utils.py` function roster 1/3: proxies shim, lengths, netrc, encoders | 5.2 |  | 0.668 |
| walker |  | 7019 | 43 | Code::CodeKey { rung: Decl, file: src/requests/sessions.py, decl: 22, sub: 0, line: 728 } |  |  | 0.668 |
| walker |  | 7077 | 58 | Code::CodeKey { rung: Decl, file: src/requests/sessions.py, decl: 17, sub: 0, line: 655 } |  |  | 0.668 |
| walker |  | 7149 | 72 | Code::CodeKey { rung: Decl, file: src/requests/sessions.py, decl: 20, sub: 0, line: 695 } |  |  | 0.668 |
| ns | 7160 |  | 167 | `utils.py` function roster 2/3: cookies, encoding detection, URI quoting, CIDR | 5.3 | 5.2 | 0.660 |
| walker |  | 7318 | 169 | Code::CodeKey { rung: Decl, file: src/requests/sessions.py, decl: 11, sub: 2, line: 395 } |  |  | 0.677 |
| ns | 7340 |  | 180 | `utils.py` function roster 3/3: proxy resolution, default headers, header validation | 5.4 | 5.3 | 0.668 |
| walker |  | 7399 | 81 | Code::CodeKey { rung: Decl, file: src/requests/sessions.py, decl: 25, sub: 0, line: 831 } |  |  | 0.668 |
| ns | 7601 |  | 261 | `structures.py`: `CaseInsensitiveDict` and `LookupDict` in full shape | 5.5 |  | 0.657 |
| walker |  | 7650 | 251 | Code::CodeKey { rung: Decl, file: src/requests/sessions.py, decl: 16, sub: 0, line: 557 } |  |  | 0.678 |
| walker |  | 7665 | 15 | Code::CodeKey { rung: Doc, file: src/requests/models.py, decl: 55, sub: 0, line: 1030 } |  |  | 0.678 |
| walker |  | 7681 | 16 | Code::CodeKey { rung: Doc, file: src/requests/models.py, decl: 22, sub: 0, line: 422 } |  |  | 0.678 |
| walker |  | 7697 | 16 | Code::CodeKey { rung: Doc, file: src/requests/models.py, decl: 43, sub: 0, line: 855 } |  |  | 0.678 |
| ns | 7711 |  | 110 | `status_codes.py`: how `requests.codes` is built | 5.6 |  | 0.673 |
| walker |  | 7713 | 16 | Code::CodeKey { rung: Doc, file: src/requests/sessions.py, decl: 27, sub: 0, line: 883 } |  |  | 0.673 |
| walker |  | 7730 | 17 | Code::CodeKey { rung: Doc, file: src/requests/models.py, decl: 30, sub: 0, line: 652 } |  |  | 0.673 |
| walker |  | 7748 | 18 | Code::CodeKey { rung: Doc, file: src/requests/models.py, decl: 46, sub: 0, line: 881 } |  |  | 0.673 |
| walker |  | 7766 | 18 | Code::CodeKey { rung: Doc, file: src/requests/models.py, decl: 59, sub: 0, line: 1140 } |  |  | 0.673 |
| walker |  | 7784 | 18 | Code::CodeKey { rung: Doc, file: src/requests/sessions.py, decl: 6, sub: 0, line: 154 } |  |  | 0.675 |
| walker |  | 7803 | 19 | Code::CodeKey { rung: Doc, file: src/requests/models.py, decl: 58, sub: 0, line: 1122 } |  |  | 0.675 |
| walker |  | 7822 | 19 | Code::CodeKey { rung: Doc, file: src/requests/sessions.py, decl: 7, sub: 0, line: 186 } |  |  | 0.679 |
| walker |  | 7842 | 20 | Code::CodeKey { rung: Doc, file: src/requests/sessions.py, decl: 5, sub: 0, line: 134 } |  |  | 0.681 |
| walker |  | 7855 | 13 | Code::CodeKey { rung: ModuleDoc, file: tests/__init__.py, decl: 0, sub: 0, line: 0 } |  |  | 0.681 |
| walker |  | 7877 | 22 | Code::CodeKey { rung: Doc, file: src/requests/models.py, decl: 48, sub: 0, line: 894 } |  |  | 0.681 |
| walker |  | 7901 | 24 | Code::CodeKey { rung: Doc, file: src/requests/models.py, decl: 19, sub: 0, line: 358 } |  |  | 0.681 |
| walker |  | 7925 | 24 | Code::CodeKey { rung: Doc, file: src/requests/models.py, decl: 47, sub: 0, line: 889 } |  |  | 0.681 |
| ns | 7953 |  | 242 | The five small modules: hooks, `_internal_utils`, compat, certs, packages | 5.7 |  | 0.672 |
| ns | 8130 |  | 177 | `help.py`: the bug-report payload | 5.8 |  | 0.664 |
| walker |  | 8181 | 256 | Markdown::Section { file: README.md, section_index: 4, keeps_default_concavity: false } |  |  | 0.664 |
| walker |  | 8215 | 34 | Code::CodeKey { rung: Doc, file: src/requests/sessions.py, decl: 24, sub: 0, line: 752 } |  |  | 0.664 |
| walker |  | 8221 | 6 | Fs::DirListing { dir: tests/certs/valid } |  |  | 0.664 |
| walker |  | 8256 | 35 | Code::CodeKey { rung: Doc, file: src/requests/models.py, decl: 15, sub: 0, line: 271 } |  |  | 0.664 |
| walker |  | 8293 | 37 | Code::CodeKey { rung: Doc, file: src/requests/models.py, decl: 34, sub: 0, line: 730 } |  |  | 0.672 |
| walker |  | 8332 | 39 | Code::CodeKey { rung: Doc, file: src/requests/sessions.py, decl: 10, sub: 0, line: 370 } |  |  | 0.672 |
| walker |  | 8373 | 41 | Code::CodeKey { rung: Doc, file: src/requests/sessions.py, decl: 28, sub: 0, line: 888 } |  |  | 0.672 |
| walker |  | 8415 | 42 | Code::CodeKey { rung: Doc, file: src/requests/sessions.py, decl: 25, sub: 0, line: 831 } |  |  | 0.672 |
| ns | 8463 |  | 333 | `resolve_redirects`: the rules the redirect loop enforces | 6.1 | 3.8 | 0.658 |
| walker |  | 8631 | 216 | Code::CodeKey { rung: Names, file: src/requests/utils.py, decl: 0, sub: 0, line: 0 } |  |  | 0.663 |
| walker |  | 8641 | 10 | Code::CodeKey { rung: Decl, file: src/requests/utils.py, decl: 12, sub: 0, line: 328 } |  |  | 0.663 |
| walker |  | 8669 | 28 | Code::CodeKey { rung: Decl, file: src/requests/utils.py, decl: 6, sub: 0, line: 91 } |  |  | 0.666 |
| walker |  | 8702 | 33 | Code::CodeKey { rung: Decl, file: src/requests/utils.py, decl: 9, sub: 0, line: 231 } |  |  | 0.666 |
| walker |  | 8739 | 37 | Code::CodeKey { rung: Decl, file: src/requests/utils.py, decl: 7, sub: 0, line: 149 } |  |  | 0.666 |
| walker |  | 8752 | 13 | Code::CodeKey { rung: Doc, file: src/requests/utils.py, decl: 7, sub: 0, line: 149 } |  |  | 0.654 |
| ns | 8752 |  | 289 | `HTTPAdapter.send`: the urllib3 call and the `MaxRetryError` fan-out | 6.2 | 3.7 | 0.654 |
| walker |  | 8769 | 17 | Code::CodeKey { rung: Doc, file: src/requests/utils.py, decl: 10, sub: 0, line: 283 } |  |  | 0.654 |
| walker |  | 8786 | 17 | Code::CodeKey { rung: Doc, file: src/requests/utils.py, decl: 12, sub: 0, line: 328 } |  |  | 0.654 |
| walker |  | 8805 | 19 | Code::CodeKey { rung: Doc, file: src/requests/utils.py, decl: 9, sub: 0, line: 231 } |  |  | 0.654 |
| ns | 8954 |  | 202 | `HTTPAdapter.send`: the remaining `except` arms and the return | 6.3 | 6.2 | 0.645 |
| ns | 9042 |  | 88 | Makefile: the remaining targets (coverage, publish, docs) | 7.1 | 1.8 | 0.649 |
| walker |  | 9085 | 280 | Code::CodeKey { rung: Names, file: src/requests/cookies.py, decl: 0, sub: 0, line: 0 } |  |  | 0.658 |
| walker |  | 9112 | 27 | Code::CodeKey { rung: Decl, file: src/requests/cookies.py, decl: 20, sub: 0, line: 135 } |  |  | 0.658 |
| walker |  | 9145 | 33 | Code::CodeKey { rung: Decl, file: src/requests/cookies.py, decl: 57, sub: 0, line: 604 } |  |  | 0.658 |
| walker |  | 9184 | 39 | Code::CodeKey { rung: Decl, file: src/requests/cookies.py, decl: 22, sub: 0, line: 164 } |  |  | 0.658 |
| walker |  | 9235 | 51 | Code::CodeKey { rung: Decl, file: src/requests/cookies.py, decl: 56, sub: 0, line: 579 } |  |  | 0.658 |
| walker |  | 9288 | 53 | Code::CodeKey { rung: Decl, file: src/requests/cookies.py, decl: 16, sub: 0, line: 114 } |  |  | 0.658 |
| ns | 9325 |  | 283 | pyproject: every remaining table, and the settings that change how you work here | 7.2 | 1.10 | 0.645 |
| walker |  | 9344 | 56 | Code::CodeKey { rung: Decl, file: src/requests/cookies.py, decl: 54, sub: 0, line: 563 } |  |  | 0.645 |
| walker |  | 9400 | 56 | Code::CodeKey { rung: Decl, file: src/requests/cookies.py, decl: 55, sub: 0, line: 571 } |  |  | 0.645 |
| ns | 9480 |  | 155 | `tests/conftest.py`: the fixture set every test builds on | 7.3 |  | 0.640 |
| walker |  | 9549 | 149 | Code::CodeKey { rung: Decl, file: src/requests/cookies.py, decl: 24, sub: 0, line: 191 } |  |  | 0.640 |
| ns | 9575 |  | 95 | `tests/test_requests.py`: top-level class roster | 7.4 |  | 0.636 |
| walker |  | 9592 | 43 | Code::CodeKey { rung: Decl, file: src/requests/cookies.py, decl: 26, sub: 0, line: 229 } |  |  | 0.636 |
| walker |  | 9658 | 66 | Code::CodeKey { rung: Decl, file: src/requests/cookies.py, decl: 25, sub: 0, line: 211 } |  |  | 0.636 |
| ns | 9757 |  | 182 | `tests/testserver/server.py`: the local socket server API (complete) | 7.5 |  | 0.630 |
| ns | 9914 |  | 157 | `docs/api.rst`: every section of the developer interface | 7.6 |  | 0.623 |
| walker |  | 9934 | 276 | Code::CodeKey { rung: Decl, file: src/requests/cookies.py, decl: 1, sub: 0, line: 31 } |  |  | 0.623 |
| walker |  | 9942 | 8 | Code::CodeKey { rung: Decl, file: src/requests/cookies.py, decl: 13, sub: 0, line: 101 } |  |  | 0.623 |
| walker |  | 9950 | 8 | Code::CodeKey { rung: Decl, file: src/requests/cookies.py, decl: 14, sub: 0, line: 105 } |  |  | 0.623 |
| walker |  | 9958 | 8 | Code::CodeKey { rung: Decl, file: src/requests/cookies.py, decl: 15, sub: 0, line: 109 } |  |  | 0.623 |
| ns | 9962 |  | 48 | `HISTORY.md`: the changelog's head | 7.7 |  | 0.622 |
| walker |  | 9988 | 30 | Code::CodeKey { rung: Decl, file: src/requests/cookies.py, decl: 24, sub: 1, line: 191 } |  |  | 0.622 |
