Score(3000)=0.367 I=0.463 C=0.290 ns_rows≤3K=17/44 grid(1000/1442/2080/3000/4327/6240/9000)=0.562/0.566/0.403/0.367/0.340/0.333/0.367

| source | ns_cum | walker_cum | marginal | descriptor | id | predecessor | Score(B=cum) |
|:-------|-------:|-----------:|---------:|:-----------|:---|:------------|-------------:|
| walker |  | 83 | 83 | Fs::DirListing { dir: . } |  |  | 0.000 |
| ns | 94 |  | 94 | README identity lede | 1.1 |  | 0.000 |
| walker |  | 102 | 19 | Fs::DirListing { dir: extra } |  |  | 0.000 |
| walker |  | 140 | 38 | Markdown::ReadmeHeadline { file: README.rst } |  |  | 0.245 |
| ns | 177 |  | 83 | Repository root listing (complete) | 1.2 |  | 0.603 |
| walker |  | 211 | 71 | Toml::Identity { file: pyproject.toml } |  |  | 0.611 |
| ns | 246 |  | 69 | beets/ core package listing (complete) | 1.3 |  | 0.454 |
| walker |  | 280 | 69 | Fs::DirListing { dir: beets } |  |  | 0.679 |
| walker |  | 289 | 9 | Fs::DirListing { dir: beets/ui } |  |  | 0.680 |
| walker |  | 307 | 18 | Fs::DirListing { dir: beets/autotag } |  |  | 0.681 |
| walker |  | 325 | 18 | Code::CodeKey { rung: ModuleDoc, file: beets/autotag/__init__.py, decl: 0, sub: 0, line: 0 } |  |  | 0.681 |
| ns | 344 |  | 98 | Shipped packages, Python floor, `beet` console entry point | 1.4 |  | 0.605 |
| walker |  | 347 | 22 | Fs::DirListing { dir: beets/importer } |  |  | 0.610 |
| walker |  | 385 | 38 | Code::CodeKey { rung: ModuleDoc, file: beets/importer/__init__.py, decl: 0, sub: 0, line: 0 } |  |  | 0.611 |
| walker |  | 425 | 40 | Code::CodeKey { rung: ModuleDoc, file: beets/__init__.py, decl: 0, sub: 0, line: 0 } |  |  | 0.611 |
| ns | 442 |  | 98 | Core subpackage listings: dbcore, library, autotag, importer | 1.5 |  | 0.516 |
| walker |  | 453 | 28 | Fs::DirListing { dir: beets/dbcore } |  |  | 0.569 |
| walker |  | 488 | 35 | Code::CodeKey { rung: ModuleDoc, file: beets/dbcore/__init__.py, decl: 0, sub: 0, line: 0 } |  |  | 0.569 |
| walker |  | 529 | 41 | Code::CodeKey { rung: ModuleDoc, file: beets/__main__.py, decl: 0, sub: 0, line: 0 } |  |  | 0.569 |
| ns | 542 |  | 100 | Project metadata header (poetry, version, license, URLs) | 1.6 |  | 0.556 |
| walker |  | 559 | 30 | Fs::DirListing { dir: beets/library } |  |  | 0.641 |
| walker |  | 623 | 64 | Code::CodeKey { rung: ModuleDoc, file: beets/ui/__init__.py, decl: 0, sub: 0, line: 0 } |  |  | 0.641 |
| ns | 690 |  | 148 | README capability bullets, part 1 (plugin framing + metadata sources) | 1.7 |  | 0.606 |
| walker |  | 695 | 72 | Code::CodeKey { rung: Names, file: beets/__init__.py, decl: 0, sub: 0, line: 0 } |  |  | 0.606 |
| walker |  | 721 | 26 | Code::CodeKey { rung: Decl, file: beets/__init__.py, decl: 2, sub: 0, line: 35 } |  |  | 0.606 |
| walker |  | 731 | 10 | Code::CodeKey { rung: Doc, file: beets/__init__.py, decl: 1, sub: 0, line: 26 } |  |  | 0.606 |
| ns | 885 |  | 195 | README capability bullets, part 2 (files, art, web, MPD) | 1.8 |  | 0.562 |
| walker |  | 989 | 258 | Code::CodeKey { rung: Names, file: beets/ui/__init__.py, decl: 0, sub: 0, line: 0 } |  |  | 0.562 |
| walker |  | 1016 | 27 | Code::CodeKey { rung: Decl, file: beets/ui/__init__.py, decl: 2, sub: 0, line: 65 } |  |  | 0.562 |
| ns | 1051 |  | 166 | UI, util and test-helper subpackage listings | 1.9 |  | 0.467 |
| walker |  | 1083 | 67 | Fs::DirListing { dir: beets/ui/commands } |  |  | 0.506 |
| walker |  | 1097 | 14 | Fs::DirListing { dir: beets/ui/commands/import_ } |  |  | 0.506 |
| walker |  | 1117 | 20 | Code::CodeKey { rung: ModuleDoc, file: beets/ui/commands/import_/__init__.py, decl: 0, sub: 0, line: 0 } |  |  | 0.506 |
| walker |  | 1150 | 33 | Code::CodeKey { rung: ModuleDoc, file: beets/ui/commands/__init__.py, decl: 0, sub: 0, line: 0 } |  |  | 0.506 |
| walker |  | 1225 | 75 | Fs::DirListing { dir: beets/util } |  |  | 0.600 |
| ns | 1227 |  | 176 | Canonical test / lint / typecheck commands | 1.10 |  | 0.562 |
| walker |  | 1239 | 14 | Code::CodeKey { rung: ModuleDoc, file: beets/util/__init__.py, decl: 0, sub: 0, line: 0 } |  |  | 0.562 |
| walker |  | 1316 | 77 | Fs::DirListing { dir: docs } |  |  | 0.566 |
| walker |  | 1320 | 4 | Fs::DirListing { dir: docs/_templates } |  |  | 0.566 |
| walker |  | 1324 | 4 | Fs::DirListing { dir: docs/extensions } |  |  | 0.566 |
| walker |  | 1338 | 14 | Fs::DirListing { dir: docs/_static } |  |  | 0.566 |
| walker |  | 1360 | 22 | Fs::DirListing { dir: docs/api } |  |  | 0.566 |
| walker |  | 1386 | 26 | Fs::DirListing { dir: docs/guides } |  |  | 0.566 |
| walker |  | 1412 | 26 | Fs::DirListing { dir: docs/reference } |  |  | 0.566 |
| walker |  | 1440 | 28 | Fs::DirListing { dir: docs/dev } |  |  | 0.566 |
| ns | 1445 |  | 218 | test/ and docs/ top-level listings (complete) | 1.11 |  | 0.503 |
| walker |  | 1454 | 14 | Code::CodeKey { rung: Names, file: beets/library/library.py, decl: 0, sub: 0, line: 0 } |  |  | 0.503 |
| walker |  | 1481 | 27 | Toml::Operational { file: pyproject.toml } |  |  | 0.505 |
| walker |  | 1502 | 21 | Fs::DirListing { dir: docs/_templates/autosummary } |  |  | 0.505 |
| walker |  | 1713 | 211 | Code::CodeKey { rung: Names, file: beets/ui/commands/__init__.py, decl: 0, sub: 0, line: 0 } |  |  | 0.506 |
| walker |  | 1723 | 10 | Code::CodeKey { rung: Doc, file: beets/ui/commands/__init__.py, decl: 1, sub: 0, line: 36 } |  |  | 0.506 |
| ns | 1828 |  | 383 | beetsplug/ bundled plugin listing (complete, 81 entries) | 1.12 |  | 0.415 |
| walker |  | 1830 | 107 | Code::CodeKey { rung: Decl, file: beets/ui/commands/__init__.py, decl: 2, sub: 0, line: 50 } |  |  | 0.417 |
| walker |  | 1854 | 24 | Fs::DirListing { dir: docs/dev/plugins } |  |  | 0.417 |
| walker |  | 2006 | 152 | Code::CodeKey { rung: Names, file: beets/importer/__init__.py, decl: 0, sub: 0, line: 0 } |  |  | 0.403 |
| ns | 2006 |  | 178 | beets.library public export block (complete) | 2.1 |  | 0.403 |
| ns | 2183 |  | 177 | The `beet` subcommand roster (default_commands) | 2.2 |  | 0.408 |
| walker |  | 2325 | 319 | Code::CodeKey { rung: Names, file: beets/util/__init__.py, decl: 0, sub: 0, line: 0 } |  |  | 0.408 |
| walker |  | 2359 | 34 | Code::CodeKey { rung: Decl, file: beets/util/__init__.py, decl: 14, sub: 0, line: 130 } |  |  | 0.408 |
| ns | 2373 |  | 190 | Library class: models and schema migrations | 2.3 |  | 0.399 |
| walker |  | 2455 | 96 | Code::CodeKey { rung: Decl, file: beets/util/__init__.py, decl: 8, sub: 0, line: 74 } |  |  | 0.399 |
| walker |  | 2465 | 10 | Code::CodeKey { rung: Body, file: beets/util/__init__.py, decl: 12, sub: 0, line: 115 } |  |  | 0.399 |
| ns | 2528 |  | 155 | Library method roster (complete) | 2.4 | 2.3 | 0.392 |
| walker |  | 2637 | 172 | Code::CodeKey { rung: Names, file: beets/ui/commands/import_/__init__.py, decl: 0, sub: 0, line: 0 } |  |  | 0.392 |
| walker |  | 2663 | 26 | Code::CodeKey { rung: Decl, file: beets/ui/commands/import_/__init__.py, decl: 7, sub: 0, line: 160 } |  |  | 0.392 |
| walker |  | 2681 | 18 | Code::CodeKey { rung: Doc, file: beets/ui/commands/import_/__init__.py, decl: 3, sub: 0, line: 34 } |  |  | 0.392 |
| walker |  | 2701 | 20 | Code::CodeKey { rung: Names, file: beets/library/fields.py, decl: 0, sub: 0, line: 0 } |  |  | 0.392 |
| walker |  | 2721 | 20 | Code::CodeKey { rung: Names, file: beets/util/hidden.py, decl: 0, sub: 0, line: 0 } |  |  | 0.392 |
| walker |  | 2749 | 28 | Code::CodeKey { rung: Body, file: beets/ui/__init__.py, decl: 7, sub: 0, line: 111 } |  |  | 0.392 |
| walker |  | 2939 | 190 | Code::CodeKey { rung: Names, file: beets/autotag/__init__.py, decl: 0, sub: 0, line: 0 } |  |  | 0.392 |
| ns | 2952 |  | 424 | models.py class headers: LibModel, FormattedItemMapping, Album, Item | 2.5 |  | 0.367 |
| walker |  | 2971 | 32 | Code::CodeKey { rung: Doc, file: beets/ui/commands/import_/__init__.py, decl: 4, sub: 0, line: 49 } |  |  | 0.367 |
| walker |  | 3004 | 33 | Code::CodeKey { rung: Doc, file: beets/ui/commands/import_/__init__.py, decl: 2, sub: 0, line: 14 } |  |  | 0.367 |
| ns | 3125 |  | 173 | beets.dbcore public export block + package docstring | 2.6 |  | 0.357 |
| walker |  | 3215 | 211 | Code::CodeKey { rung: Names, file: beets/library/__init__.py, decl: 0, sub: 0, line: 0 } |  |  | 0.389 |
| walker |  | 3282 | 67 | Code::CodeKey { rung: Decl, file: beets/library/__init__.py, decl: 1, sub: 0, line: 8 } |  |  | 0.389 |
| ns | 3298 |  | 173 | config_default.yaml: library, directory, plugins, ignore rules | 3.1 |  | 0.378 |
| walker |  | 3301 | 19 | Code::CodeKey { rung: Body, file: beets/library/__init__.py, decl: 2, sub: 0, line: 15 } |  |  | 0.378 |
| walker |  | 3316 | 15 | Code::CodeKey { rung: Names, file: beets/ui/commands/help.py, decl: 0, sub: 0, line: 0 } |  |  | 0.378 |
| walker |  | 3342 | 26 | Code::CodeKey { rung: Names, file: beets/util/m3u.py, decl: 0, sub: 0, line: 0 } |  |  | 0.378 |
| walker |  | 3349 | 7 | Code::CodeKey { rung: Decl, file: beets/util/m3u.py, decl: 1, sub: 0, line: 22 } |  |  | 0.378 |
| walker |  | 3386 | 37 | Code::CodeKey { rung: Doc, file: beets/ui/__init__.py, decl: 3, sub: 0, line: 71 } |  |  | 0.378 |
| walker |  | 3413 | 27 | Code::CodeKey { rung: Names, file: beets/importer/state.py, decl: 0, sub: 0, line: 0 } |  |  | 0.378 |
| walker |  | 3453 | 40 | Code::CodeKey { rung: Doc, file: beets/__init__.py, decl: 2, sub: 0, line: 35 } |  |  | 0.378 |
| ns | 3687 |  | 389 | config_default.yaml: the complete `import:` option block | 3.2 |  | 0.359 |
| walker |  | 3738 | 285 | Code::CodeKey { rung: Names, file: beets/ui/__init__.py, decl: 0, sub: 1, line: 0 } |  |  | 0.359 |
| walker |  | 3745 | 7 | Code::CodeKey { rung: Decl, file: beets/ui/__init__.py, decl: 18, sub: 0, line: 459 } |  |  | 0.359 |
| walker |  | 3809 | 64 | Code::CodeKey { rung: Decl, file: beets/ui/__init__.py, decl: 13, sub: 0, line: 207 } |  |  | 0.359 |
| ns | 3866 |  | 179 | config_default.yaml: tagging defaults, `paths:` templates, threading | 3.4 |  | 0.352 |
| walker |  | 3881 | 72 | Code::CodeKey { rung: Decl, file: beets/ui/__init__.py, decl: 19, sub: 0, line: 466 } |  |  | 0.352 |
| walker |  | 3907 | 26 | Code::CodeKey { rung: Decl, file: beets/ui/__init__.py, decl: 40, sub: 0, line: 807 } |  |  | 0.352 |
| walker |  | 4005 | 98 | Code::CodeKey { rung: Decl, file: beets/ui/__init__.py, decl: 33, sub: 0, line: 676 } |  |  | 0.352 |
| walker |  | 4110 | 105 | Code::CodeKey { rung: Decl, file: beets/ui/__init__.py, decl: 27, sub: 0, line: 637 } |  |  | 0.352 |
| walker |  | 4118 | 8 | Code::CodeKey { rung: Decl, file: beets/ui/__init__.py, decl: 31, sub: 0, line: 664 } |  |  | 0.352 |
| walker |  | 4128 | 10 | Code::CodeKey { rung: Decl, file: beets/ui/__init__.py, decl: 32, sub: 0, line: 668 } |  |  | 0.352 |
| ns | 4171 |  | 305 | Path-template function roster: DefaultTemplateFunctions (complete) | 3.5 |  | 0.344 |
| walker |  | 4241 | 113 | Code::CodeKey { rung: Decl, file: beets/ui/__init__.py, decl: 20, sub: 0, line: 498 } |  |  | 0.344 |
| ns | 4291 |  | 120 | config_default.yaml: display formats and default sorts | 3.6 |  | 0.340 |
| walker |  | 4512 | 271 | Code::CodeKey { rung: Names, file: beets/dbcore/__init__.py, decl: 0, sub: 0, line: 0 } |  |  | 0.367 |
| walker |  | 4535 | 23 | Code::CodeKey { rung: Body, file: beets/util/__init__.py, decl: 15, sub: 0, line: 136 } |  |  | 0.367 |
| walker |  | 4569 | 34 | Code::CodeKey { rung: Names, file: beets/util/config.py, decl: 0, sub: 0, line: 0 } |  |  | 0.367 |
| walker |  | 4584 | 15 | Code::CodeKey { rung: Decl, file: beets/util/config.py, decl: 3, sub: 0, line: 78 } |  |  | 0.367 |
| walker |  | 4609 | 25 | Code::CodeKey { rung: Decl, file: beets/util/config.py, decl: 1, sub: 0, line: 9 } |  |  | 0.367 |
| walker |  | 4630 | 21 | Code::CodeKey { rung: Names, file: beets/ui/commands/utils.py, decl: 0, sub: 0, line: 0 } |  |  | 0.367 |
| walker |  | 4645 | 15 | Fs::DirListing { dir: beets/test } |  |  | 0.377 |
| ns | 4673 |  | 382 | config_default.yaml: autotagger thresholds and distance weights | 3.7 |  | 0.363 |
| walker |  | 4685 | 40 | Code::CodeKey { rung: Names, file: beets/library/exceptions.py, decl: 0, sub: 0, line: 0 } |  |  | 0.363 |
| walker |  | 4698 | 13 | Code::CodeKey { rung: Decl, file: beets/library/exceptions.py, decl: 4, sub: 0, line: 27 } |  |  | 0.363 |
| walker |  | 4711 | 13 | Code::CodeKey { rung: Decl, file: beets/library/exceptions.py, decl: 6, sub: 0, line: 34 } |  |  | 0.363 |
| walker |  | 4741 | 30 | Code::CodeKey { rung: Decl, file: beets/library/exceptions.py, decl: 1, sub: 0, line: 4 } |  |  | 0.363 |
| ns | 4813 |  | 140 | config_default.yaml: terminal UI, colour names, import layout | 3.8 |  | 0.356 |
| walker |  | 4822 | 81 | Code::CodeKey { rung: Names, file: beets/context.py, decl: 0, sub: 0, line: 0 } |  |  | 0.356 |
| walker |  | 4830 | 8 | Code::CodeKey { rung: Decl, file: beets/context.py, decl: 4, sub: 0, line: 18 } |  |  | 0.356 |
| walker |  | 4842 | 12 | Code::CodeKey { rung: Body, file: beets/context.py, decl: 2, sub: 0, line: 8 } |  |  | 0.356 |
| walker |  | 4853 | 11 | Code::CodeKey { rung: Doc, file: beets/context.py, decl: 2, sub: 0, line: 8 } |  |  | 0.356 |
| walker |  | 4865 | 12 | Code::CodeKey { rung: Body, file: beets/context.py, decl: 3, sub: 0, line: 13 } |  |  | 0.356 |
| walker |  | 4922 | 57 | Code::CodeKey { rung: Body, file: beets/__init__.py, decl: 1, sub: 0, line: 26 } |  |  | 0.356 |
| walker |  | 4961 | 39 | Fs::DirListing { dir: .github } |  |  | 0.356 |
| ns | 4986 |  | 173 | Album method roster (complete, names only) | 4.1 | 2.5 | 0.351 |
| walker |  | 4992 | 31 | Fs::DirListing { dir: .github/workflows } |  |  | 0.351 |
| walker |  | 5033 | 41 | Fs::DirListing { dir: docs/dev/plugins/other } |  |  | 0.351 |
| walker |  | 5061 | 28 | Code::CodeKey { rung: Decl, file: beets/ui/commands/help.py, decl: 1, sub: 0, line: 6 } |  |  | 0.351 |
| walker |  | 5072 | 11 | Code::CodeKey { rung: Doc, file: beets/context.py, decl: 3, sub: 0, line: 13 } |  |  | 0.351 |
| ns | 5290 |  | 304 | Item method roster (complete, names only) | 4.3 | 2.5 | 0.342 |
| walker |  | 5341 | 269 | Code::CodeKey { rung: Names, file: beets/plugins.py, decl: 0, sub: 0, line: 0 } |  |  | 0.342 |
| walker |  | 5358 | 17 | Code::CodeKey { rung: Decl, file: beets/plugins.py, decl: 6, sub: 0, line: 116 } |  |  | 0.342 |
| walker |  | 5386 | 28 | Code::CodeKey { rung: Decl, file: beets/plugins.py, decl: 8, sub: 0, line: 127 } |  |  | 0.342 |
| walker |  | 5438 | 52 | Code::CodeKey { rung: Decl, file: beets/plugins.py, decl: 11, sub: 0, line: 147 } |  |  | 0.342 |
| walker |  | 5447 | 9 | Code::CodeKey { rung: Body, file: beets/plugins.py, decl: 30, sub: 0, line: 500 } |  |  | 0.342 |
| ns | 5458 |  | 168 | Item search fields and MediaFile-backed field sets | 4.4 | 2.5 | 0.337 |
| ns | 5629 |  | 171 | LibModel shared-method roster (complete after `_fields`) | 4.5 | 2.5 | 0.333 |
| walker |  | 5700 | 253 | Code::CodeKey { rung: Names, file: beets/util/__init__.py, decl: 0, sub: 1, line: 0 } |  |  | 0.333 |
| walker |  | 5725 | 25 | Code::CodeKey { rung: Decl, file: beets/util/__init__.py, decl: 18, sub: 0, line: 169 } |  |  | 0.333 |
| walker |  | 5754 | 29 | Code::CodeKey { rung: Decl, file: beets/util/__init__.py, decl: 29, sub: 0, line: 405 } |  |  | 0.333 |
| ns | 5787 |  | 158 | beets.importer public exports + package docstring | 5.1 |  | 0.340 |
| walker |  | 5805 | 51 | Code::CodeKey { rung: Decl, file: beets/util/__init__.py, decl: 25, sub: 0, line: 309 } |  |  | 0.340 |
| walker |  | 5871 | 66 | Code::CodeKey { rung: Decl, file: beets/util/__init__.py, decl: 17, sub: 0, line: 158 } |  |  | 0.340 |
| walker |  | 5941 | 70 | Code::CodeKey { rung: Decl, file: beets/util/__init__.py, decl: 21, sub: 0, line: 208 } |  |  | 0.340 |
| walker |  | 6008 | 67 | Code::CodeKey { rung: Decl, file: beets/ui/__init__.py, decl: 23, sub: 0, line: 534 } |  |  | 0.340 |
| walker |  | 6061 | 53 | Code::CodeKey { rung: Names, file: beets/util/units.py, decl: 0, sub: 0, line: 0 } |  |  | 0.340 |
| walker |  | 6116 | 55 | Code::CodeKey { rung: Names, file: beets/util/lyrics.py, decl: 0, sub: 0, line: 0 } |  |  | 0.340 |
| ns | 6147 |  | 360 | importer/stages.py: complete stage-function roster with section banners | 5.2 |  | 0.333 |
| walker |  | 6171 | 55 | Code::CodeKey { rung: Decl, file: beets/util/config.py, decl: 2, sub: 0, line: 29 } |  |  | 0.333 |
| walker |  | 6249 | 78 | Code::CodeKey { rung: Body, file: beets/ui/commands/__init__.py, decl: 1, sub: 0, line: 36 } |  |  | 0.333 |
| walker |  | 6308 | 59 | Code::CodeKey { rung: Names, file: beets/importer/session.py, decl: 0, sub: 0, line: 0 } |  |  | 0.333 |
| walker |  | 6315 | 7 | Code::CodeKey { rung: Decl, file: beets/importer/session.py, decl: 3, sub: 0, line: 42 } |  |  | 0.333 |
| ns | 6572 |  | 425 | ImportSession.run(): how the pipeline is assembled | 5.3 | 5.2 | 0.323 |
| walker |  | 6698 | 383 | Fs::DirListing { dir: beetsplug } |  |  | 0.414 |
| walker |  | 6709 | 11 | Fs::DirListing { dir: beetsplug/bpd } |  |  | 0.414 |
| walker |  | 6721 | 12 | Fs::DirListing { dir: beetsplug/web } |  |  | 0.414 |
| walker |  | 6737 | 16 | Code::CodeKey { rung: ModuleDoc, file: beetsplug/web/__init__.py, decl: 0, sub: 0, line: 0 } |  |  | 0.414 |
| walker |  | 6751 | 14 | Fs::DirListing { dir: beetsplug/discogs } |  |  | 0.414 |
| walker |  | 6767 | 16 | Fs::DirListing { dir: beetsplug/metasync } |  |  | 0.414 |
| walker |  | 6784 | 17 | Code::CodeKey { rung: ModuleDoc, file: beetsplug/metasync/__init__.py, decl: 0, sub: 0, line: 0 } |  |  | 0.414 |
| walker |  | 6788 | 4 | Fs::DirListing { dir: beetsplug/web/templates } |  |  | 0.414 |
| walker |  | 6807 | 19 | Fs::DirListing { dir: beetsplug/tidal } |  |  | 0.414 |
| ns | 6829 |  | 257 | ImportSession class contract (attributes + the four decision hooks) | 5.4 | 5.3 | 0.406 |
| walker |  | 6830 | 23 | Fs::DirListing { dir: beetsplug/lastgenre } |  |  | 0.406 |
| walker |  | 6869 | 39 | Code::CodeKey { rung: ModuleDoc, file: beetsplug/discogs/__init__.py, decl: 0, sub: 0, line: 0 } |  |  | 0.406 |
| walker |  | 6899 | 30 | Fs::DirListing { dir: beetsplug/_utils } |  |  | 0.406 |
| walker |  | 6931 | 32 | Code::CodeKey { rung: Names, file: beetsplug/_utils/__init__.py, decl: 0, sub: 0, line: 0 } |  |  | 0.406 |
| walker |  | 6994 | 63 | Code::CodeKey { rung: ModuleDoc, file: beetsplug/bpd/__init__.py, decl: 0, sub: 0, line: 0 } |  |  | 0.406 |
| walker |  | 7008 | 14 | Code::CodeKey { rung: Names, file: beetsplug/substitute.py, decl: 0, sub: 0, line: 0 } |  |  | 0.406 |
| walker |  | 7023 | 15 | Code::CodeKey { rung: Names, file: beetsplug/replace.py, decl: 0, sub: 0, line: 0 } |  |  | 0.406 |
| walker |  | 7038 | 15 | Code::CodeKey { rung: Names, file: beetsplug/types.py, decl: 0, sub: 0, line: 0 } |  |  | 0.406 |
| walker |  | 7054 | 16 | Code::CodeKey { rung: Names, file: beetsplug/albumtypes.py, decl: 0, sub: 0, line: 0 } |  |  | 0.406 |
| walker |  | 7070 | 16 | Code::CodeKey { rung: Names, file: beetsplug/filefilter.py, decl: 0, sub: 0, line: 0 } |  |  | 0.406 |
| walker |  | 7086 | 16 | Code::CodeKey { rung: Names, file: beetsplug/importadded.py, decl: 0, sub: 0, line: 0 } |  |  | 0.406 |
| walker |  | 7102 | 16 | Code::CodeKey { rung: Names, file: beetsplug/importsource.py, decl: 0, sub: 0, line: 0 } |  |  | 0.406 |
| ns | 7103 |  | 274 | importer/tasks.py: Action enum and the complete task class roster | 5.5 |  | 0.400 |
| walker |  | 7118 | 16 | Code::CodeKey { rung: Names, file: beetsplug/ipfs.py, decl: 0, sub: 0, line: 0 } |  |  | 0.400 |
| walker |  | 7134 | 16 | Code::CodeKey { rung: Names, file: beetsplug/keyfinder.py, decl: 0, sub: 0, line: 0 } |  |  | 0.400 |
| walker |  | 7150 | 16 | Code::CodeKey { rung: Names, file: beetsplug/loadext.py, decl: 0, sub: 0, line: 0 } |  |  | 0.400 |
| walker |  | 7166 | 16 | Code::CodeKey { rung: Names, file: beetsplug/mbsubmit.py, decl: 0, sub: 0, line: 0 } |  |  | 0.400 |
| walker |  | 7182 | 16 | Code::CodeKey { rung: Names, file: beetsplug/sonosupdate.py, decl: 0, sub: 0, line: 0 } |  |  | 0.400 |
| walker |  | 7198 | 16 | Code::CodeKey { rung: Names, file: beetsplug/unimported.py, decl: 0, sub: 0, line: 0 } |  |  | 0.400 |
| walker |  | 7215 | 17 | Code::CodeKey { rung: Names, file: beetsplug/autobpm.py, decl: 0, sub: 0, line: 0 } |  |  | 0.400 |
| walker |  | 7232 | 17 | Code::CodeKey { rung: Names, file: beetsplug/bpsync.py, decl: 0, sub: 0, line: 0 } |  |  | 0.400 |
| walker |  | 7249 | 17 | Code::CodeKey { rung: Names, file: beetsplug/freedesktop.py, decl: 0, sub: 0, line: 0 } |  |  | 0.400 |
| walker |  | 7266 | 17 | Code::CodeKey { rung: Names, file: beetsplug/mbsync.py, decl: 0, sub: 0, line: 0 } |  |  | 0.400 |
| walker |  | 7283 | 17 | Code::CodeKey { rung: Names, file: beetsplug/subsonicupdate.py, decl: 0, sub: 0, line: 0 } |  |  | 0.400 |
| ns | 7349 |  | 246 | BeetsPlugin base class: docstring and declared attributes | 6.1 |  | 0.394 |
| walker |  | 7540 | 257 | Code::CodeKey { rung: Names, file: beetsplug/bpd/__init__.py, decl: 0, sub: 0, line: 0 } |  |  | 0.394 |
| walker |  | 7559 | 19 | Code::CodeKey { rung: ModuleDoc, file: beetsplug/subsonicupdate.py, decl: 0, sub: 0, line: 0 } |  |  | 0.394 |
| walker |  | 7579 | 20 | Code::CodeKey { rung: ModuleDoc, file: beetsplug/unimported.py, decl: 0, sub: 0, line: 0 } |  |  | 0.394 |
| ns | 7595 |  | 246 | BeetsPlugin method roster (complete) | 6.2 | 6.1 | 0.390 |
| walker |  | 7601 | 22 | Code::CodeKey { rung: ModuleDoc, file: beetsplug/zero.py, decl: 0, sub: 0, line: 0 } |  |  | 0.390 |
| walker |  | 7706 | 105 | Code::CodeKey { rung: ModuleDoc, file: beetsplug/lastgenre/__init__.py, decl: 0, sub: 0, line: 0 } |  |  | 0.390 |
| walker |  | 7730 | 24 | Code::CodeKey { rung: ModuleDoc, file: beetsplug/subsonicplaylist.py, decl: 0, sub: 0, line: 0 } |  |  | 0.390 |
| walker |  | 7752 | 22 | Code::CodeKey { rung: Decl, file: beetsplug/unimported.py, decl: 1, sub: 0, line: 29 } |  |  | 0.390 |
| walker |  | 7776 | 24 | Code::CodeKey { rung: Names, file: beetsplug/parentwork.py, decl: 0, sub: 0, line: 0 } |  |  | 0.390 |
| walker |  | 7801 | 25 | Code::CodeKey { rung: Names, file: beetsplug/listenbrainz.py, decl: 0, sub: 0, line: 0 } |  |  | 0.390 |
| walker |  | 7827 | 26 | Code::CodeKey { rung: Names, file: beetsplug/ihate.py, decl: 0, sub: 0, line: 0 } |  |  | 0.390 |
| walker |  | 7854 | 27 | Code::CodeKey { rung: Names, file: beetsplug/badfiles.py, decl: 0, sub: 0, line: 0 } |  |  | 0.390 |
| walker |  | 7873 | 19 | Code::CodeKey { rung: Decl, file: beetsplug/badfiles.py, decl: 1, sub: 0, line: 32 } |  |  | 0.390 |
| walker |  | 7900 | 27 | Code::CodeKey { rung: Names, file: beetsplug/bpm.py, decl: 0, sub: 0, line: 0 } |  |  | 0.390 |
| walker |  | 7927 | 27 | Code::CodeKey { rung: Names, file: beetsplug/mpdupdate.py, decl: 0, sub: 0, line: 0 } |  |  | 0.390 |
| walker |  | 7953 | 26 | Code::CodeKey { rung: Decl, file: beetsplug/loadext.py, decl: 1, sub: 0, line: 23 } |  |  | 0.390 |
| walker |  | 7984 | 31 | Code::CodeKey { rung: ModuleDoc, file: beetsplug/ihate.py, decl: 0, sub: 0, line: 0 } |  |  | 0.390 |
| walker |  | 8013 | 29 | Code::CodeKey { rung: Names, file: beetsplug/subsonicplaylist.py, decl: 0, sub: 0, line: 0 } |  |  | 0.390 |
| ns | 8017 |  | 422 | The complete `EventType` literal — every plugin event name | 6.3 | 6.1 | 0.379 |
| walker |  | 8041 | 28 | Code::CodeKey { rung: Decl, file: beetsplug/freedesktop.py, decl: 1, sub: 0, line: 21 } |  |  | 0.379 |
| walker |  | 8069 | 28 | Code::CodeKey { rung: Decl, file: beetsplug/substitute.py, decl: 1, sub: 0, line: 25 } |  |  | 0.379 |
| walker |  | 8091 | 22 | Fs::DirListing { dir: beetsplug/web/static } |  |  | 0.379 |
| walker |  | 8121 | 30 | Code::CodeKey { rung: Names, file: beetsplug/hook.py, decl: 0, sub: 0, line: 0 } |  |  | 0.379 |
| walker |  | 8146 | 25 | Code::CodeKey { rung: Decl, file: beetsplug/hook.py, decl: 1, sub: 0, line: 28 } |  |  | 0.379 |
| walker |  | 8176 | 30 | Code::CodeKey { rung: Names, file: beetsplug/scrub.py, decl: 0, sub: 0, line: 0 } |  |  | 0.379 |
| walker |  | 8206 | 30 | Code::CodeKey { rung: Names, file: beetsplug/titlecase.py, decl: 0, sub: 0, line: 0 } |  |  | 0.379 |
| walker |  | 8230 | 24 | Code::CodeKey { rung: Decl, file: beetsplug/titlecase.py, decl: 1, sub: 0, line: 40 } |  |  | 0.379 |
| walker |  | 8260 | 30 | Code::CodeKey { rung: ModuleDoc, file: beetsplug/titlecase.py, decl: 0, sub: 0, line: 0 } |  |  | 0.379 |
| ns | 8321 |  | 304 | plugins.py module-level API roster (complete) | 6.4 |  | 0.376 |
| ns | 8633 |  | 312 | MetadataSourcePlugin: the metadata-backend contract | 6.5 |  | 0.371 |
| walker |  | 8704 | 444 | Code::CodeKey { rung: Names, file: beetsplug/web/__init__.py, decl: 0, sub: 0, line: 0 } |  |  | 0.371 |
| walker |  | 8712 | 8 | Code::CodeKey { rung: Decl, file: beetsplug/web/__init__.py, decl: 32, sub: 0, line: 439 } |  |  | 0.371 |
| walker |  | 8721 | 9 | Code::CodeKey { rung: Decl, file: beetsplug/web/__init__.py, decl: 18, sub: 0, line: 296 } |  |  | 0.371 |
| walker |  | 8731 | 10 | Code::CodeKey { rung: Decl, file: beetsplug/web/__init__.py, decl: 30, sub: 0, line: 412 } |  |  | 0.371 |
| walker |  | 8741 | 10 | Code::CodeKey { rung: Decl, file: beetsplug/web/__init__.py, decl: 31, sub: 0, line: 423 } |  |  | 0.371 |
| walker |  | 8755 | 14 | Code::CodeKey { rung: Decl, file: beetsplug/web/__init__.py, decl: 23, sub: 0, line: 344 } |  |  | 0.371 |
| walker |  | 8770 | 15 | Code::CodeKey { rung: Decl, file: beetsplug/web/__init__.py, decl: 24, sub: 0, line: 354 } |  |  | 0.371 |
| walker |  | 8785 | 15 | Code::CodeKey { rung: Decl, file: beetsplug/web/__init__.py, decl: 29, sub: 0, line: 397 } |  |  | 0.371 |
| walker |  | 8801 | 16 | Code::CodeKey { rung: Decl, file: beetsplug/web/__init__.py, decl: 21, sub: 0, line: 317 } |  |  | 0.371 |
| walker |  | 8818 | 17 | Code::CodeKey { rung: Decl, file: beetsplug/web/__init__.py, decl: 28, sub: 0, line: 388 } |  |  | 0.371 |
| ns | 8830 |  | 197 | Query-string prefixes: how `beet ls artist:foo` is parsed | 7.1 |  | 0.367 |
| walker |  | 8840 | 22 | Code::CodeKey { rung: Decl, file: beetsplug/web/__init__.py, decl: 16, sub: 0, line: 283 } |  |  | 0.367 |
| walker |  | 8862 | 22 | Code::CodeKey { rung: Decl, file: beetsplug/web/__init__.py, decl: 33, sub: 0, line: 447 } |  |  | 0.367 |
| walker |  | 8890 | 28 | Code::CodeKey { rung: Decl, file: beetsplug/web/__init__.py, decl: 10, sub: 0, line: 252 } |  |  | 0.367 |
| walker |  | 8918 | 28 | Code::CodeKey { rung: Decl, file: beetsplug/web/__init__.py, decl: 13, sub: 0, line: 268 } |  |  | 0.367 |
| walker |  | 8949 | 31 | Code::CodeKey { rung: Decl, file: beetsplug/web/__init__.py, decl: 20, sub: 0, line: 310 } |  |  | 0.367 |
| walker |  | 8980 | 31 | Code::CodeKey { rung: Decl, file: beetsplug/web/__init__.py, decl: 25, sub: 0, line: 369 } |  |  | 0.367 |
| walker |  | 9011 | 31 | Code::CodeKey { rung: Decl, file: beetsplug/web/__init__.py, decl: 26, sub: 0, line: 375 } |  |  | 0.367 |
| walker |  | 9043 | 32 | Code::CodeKey { rung: Decl, file: beetsplug/web/__init__.py, decl: 27, sub: 0, line: 382 } |  |  | 0.367 |
| walker |  | 9076 | 33 | Code::CodeKey { rung: Decl, file: beetsplug/web/__init__.py, decl: 36, sub: 0, line: 520 } |  |  | 0.367 |
| walker |  | 9114 | 38 | Code::CodeKey { rung: Decl, file: beetsplug/web/__init__.py, decl: 19, sub: 0, line: 304 } |  |  | 0.367 |
| walker |  | 9153 | 39 | Code::CodeKey { rung: Decl, file: beetsplug/web/__init__.py, decl: 22, sub: 0, line: 338 } |  |  | 0.367 |
| walker |  | 9182 | 29 | Code::CodeKey { rung: Decl, file: beetsplug/albumtypes.py, decl: 1, sub: 0, line: 29 } |  |  | 0.367 |
| ns | 9187 |  | 357 | dbcore/query.py class roster (complete) | 7.2 |  | 0.359 |
| walker |  | 9213 | 31 | Code::CodeKey { rung: Names, file: beetsplug/bareasc.py, decl: 0, sub: 0, line: 0 } |  |  | 0.359 |
| walker |  | 9241 | 28 | Code::CodeKey { rung: Decl, file: beetsplug/bareasc.py, decl: 1, sub: 0, line: 29 } |  |  | 0.359 |
| walker |  | 9249 | 8 | Code::CodeKey { rung: Decl, file: beetsplug/bareasc.py, decl: 2, sub: 0, line: 32 } |  |  | 0.359 |
| walker |  | 9280 | 31 | Code::CodeKey { rung: Names, file: beetsplug/embedart.py, decl: 0, sub: 0, line: 0 } |  |  | 0.359 |
| walker |  | 9311 | 31 | Code::CodeKey { rung: Names, file: beetsplug/fuzzy.py, decl: 0, sub: 0, line: 0 } |  |  | 0.359 |
| walker |  | 9336 | 25 | Code::CodeKey { rung: Decl, file: beetsplug/fuzzy.py, decl: 4, sub: 0, line: 51 } |  |  | 0.359 |
| walker |  | 9371 | 35 | Code::CodeKey { rung: ModuleDoc, file: beetsplug/the.py, decl: 0, sub: 0, line: 0 } |  |  | 0.359 |
| walker |  | 9402 | 31 | Code::CodeKey { rung: Decl, file: beetsplug/hook.py, decl: 3, sub: 0, line: 44 } |  |  | 0.359 |
| walker |  | 9435 | 33 | Code::CodeKey { rung: Names, file: beetsplug/advancedrewrite.py, decl: 0, sub: 0, line: 0 } |  |  | 0.359 |
| walker |  | 9459 | 24 | Code::CodeKey { rung: Decl, file: beetsplug/advancedrewrite.py, decl: 2, sub: 0, line: 57 } |  |  | 0.359 |
| walker |  | 9492 | 33 | Code::CodeKey { rung: Names, file: beetsplug/kodiupdate.py, decl: 0, sub: 0, line: 0 } |  |  | 0.359 |
| ns | 9539 |  | 352 | beets.autotag exports, Recommendation, Proposal | 8.1 |  | 0.353 |
| walker |  | 9647 | 155 | Code::CodeKey { rung: Names, file: beetsplug/tidal/__init__.py, decl: 0, sub: 0, line: 0 } |  |  | 0.353 |
| walker |  | 9726 | 79 | Code::CodeKey { rung: Decl, file: beetsplug/tidal/__init__.py, decl: 34, sub: 0, line: 494 } |  |  | 0.353 |
| ns | 9775 |  | 236 | autotag/hooks.py and distance.py symbol roster | 8.2 |  | 0.350 |
| walker |  | 9912 | 186 | Code::CodeKey { rung: Names, file: beetsplug/metasync/__init__.py, decl: 0, sub: 0, line: 0 } |  |  | 0.350 |
| walker |  | 9941 | 29 | Code::CodeKey { rung: Decl, file: beetsplug/metasync/__init__.py, decl: 2, sub: 0, line: 34 } |  |  | 0.350 |
| ns | 9970 |  | 195 | beets/util/__init__.py: classes and the core path/file-operation helpers | 9.1 |  | 0.349 |
| walker |  | 9993 | 52 | Code::CodeKey { rung: Decl, file: beetsplug/metasync/__init__.py, decl: 3, sub: 0, line: 40 } |  |  | 0.349 |
