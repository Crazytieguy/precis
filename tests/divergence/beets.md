Score(3000)=0.644 I=0.787 C=0.527 ns_rows≤3K=17/44 grid(1000/1442/2080/3000/4327/6240/9000)=0.594/0.757/0.723/0.644/0.549/0.477/0.409

| source | ns_cum | walker_cum | marginal | descriptor | id | predecessor | Score(B=cum) |
|:-------|-------:|-----------:|---------:|:-----------|:---|:------------|-------------:|
| walker |  | 83 | 83 | Fs::DirListing { dir: . } |  |  | 0.000 |
| ns | 94 |  | 94 | README identity lede | 1.1 |  | 0.000 |
| walker |  | 121 | 38 | Markdown::ReadmeHeadline { file: README.rst } |  |  | 0.245 |
| walker |  | 140 | 19 | Fs::DirListing { dir: extra } |  |  | 0.245 |
| ns | 177 |  | 83 | Repository root listing (complete) | 1.2 |  | 0.603 |
| walker |  | 211 | 71 | Toml::Identity { file: pyproject.toml } |  |  | 0.611 |
| ns | 246 |  | 69 | beets/ core package listing (complete) | 1.3 |  | 0.454 |
| walker |  | 280 | 69 | Fs::DirListing { dir: beets } |  |  | 0.679 |
| walker |  | 289 | 9 | Fs::DirListing { dir: beets/ui } |  |  | 0.680 |
| walker |  | 307 | 18 | Fs::DirListing { dir: beets/autotag } |  |  | 0.681 |
| walker |  | 329 | 22 | Fs::DirListing { dir: beets/importer } |  |  | 0.687 |
| ns | 344 |  | 98 | Shipped packages, Python floor, `beet` console entry point | 1.4 |  | 0.610 |
| walker |  | 347 | 18 | Code::CodeKey { rung: ModuleDoc, file: beets/autotag/__init__.py, decl: 0, sub: 0, line: 0 } |  |  | 0.610 |
| walker |  | 375 | 28 | Fs::DirListing { dir: beets/dbcore } |  |  | 0.625 |
| walker |  | 405 | 30 | Fs::DirListing { dir: beets/library } |  |  | 0.648 |
| walker |  | 440 | 35 | Code::CodeKey { rung: ModuleDoc, file: beets/dbcore/__init__.py, decl: 0, sub: 0, line: 0 } |  |  | 0.649 |
| ns | 442 |  | 98 | Core subpackage listings: dbcore, library, autotag, importer | 1.5 |  | 0.661 |
| walker |  | 478 | 38 | Code::CodeKey { rung: ModuleDoc, file: beets/importer/__init__.py, decl: 0, sub: 0, line: 0 } |  |  | 0.661 |
| walker |  | 518 | 40 | Code::CodeKey { rung: ModuleDoc, file: beets/__init__.py, decl: 0, sub: 0, line: 0 } |  |  | 0.661 |
| ns | 542 |  | 100 | Project metadata header (poetry, version, license, URLs) | 1.6 |  | 0.641 |
| walker |  | 559 | 41 | Code::CodeKey { rung: ModuleDoc, file: beets/__main__.py, decl: 0, sub: 0, line: 0 } |  |  | 0.641 |
| walker |  | 626 | 67 | Fs::DirListing { dir: beets/ui/commands } |  |  | 0.650 |
| walker |  | 640 | 14 | Fs::DirListing { dir: beets/ui/commands/import_ } |  |  | 0.650 |
| walker |  | 660 | 20 | Code::CodeKey { rung: ModuleDoc, file: beets/ui/commands/import_/__init__.py, decl: 0, sub: 0, line: 0 } |  |  | 0.650 |
| ns | 690 |  | 148 | README capability bullets, part 1 (plugin framing + metadata sources) | 1.7 |  | 0.615 |
| walker |  | 693 | 33 | Code::CodeKey { rung: ModuleDoc, file: beets/ui/commands/__init__.py, decl: 0, sub: 0, line: 0 } |  |  | 0.615 |
| walker |  | 768 | 75 | Fs::DirListing { dir: beets/util } |  |  | 0.635 |
| walker |  | 782 | 14 | Code::CodeKey { rung: ModuleDoc, file: beets/util/__init__.py, decl: 0, sub: 0, line: 0 } |  |  | 0.635 |
| walker |  | 859 | 77 | Fs::DirListing { dir: docs } |  |  | 0.639 |
| walker |  | 863 | 4 | Fs::DirListing { dir: docs/extensions } |  |  | 0.639 |
| walker |  | 877 | 14 | Fs::DirListing { dir: docs/_static } |  |  | 0.639 |
| ns | 885 |  | 195 | README capability bullets, part 2 (files, art, web, MPD) | 1.8 |  | 0.593 |
| walker |  | 899 | 22 | Fs::DirListing { dir: docs/api } |  |  | 0.593 |
| walker |  | 925 | 26 | Fs::DirListing { dir: docs/guides } |  |  | 0.593 |
| walker |  | 951 | 26 | Fs::DirListing { dir: docs/reference } |  |  | 0.593 |
| walker |  | 979 | 28 | Fs::DirListing { dir: docs/dev } |  |  | 0.593 |
| walker |  | 1006 | 27 | Toml::Operational { file: pyproject.toml } |  |  | 0.596 |
| ns | 1051 |  | 166 | UI, util and test-helper subpackage listings | 1.9 |  | 0.606 |
| walker |  | 1070 | 64 | Code::CodeKey { rung: ModuleDoc, file: beets/ui/__init__.py, decl: 0, sub: 0, line: 0 } |  |  | 0.606 |
| walker |  | 1094 | 24 | Fs::DirListing { dir: docs/_templates/autosummary } |  |  | 0.606 |
| walker |  | 1118 | 24 | Fs::DirListing { dir: docs/dev/plugins } |  |  | 0.606 |
| ns | 1227 |  | 176 | Canonical test / lint / typecheck commands | 1.10 |  | 0.568 |
| walker |  | 1323 | 205 | Markdown::Section { file: README.rst, section_index: 0, keeps_default_concavity: false } |  |  | 0.732 |
| walker |  | 1338 | 15 | Fs::DirListing { dir: beets/test } |  |  | 0.757 |
| walker |  | 1377 | 39 | Fs::DirListing { dir: .github } |  |  | 0.757 |
| walker |  | 1408 | 31 | Fs::DirListing { dir: .github/workflows } |  |  | 0.757 |
| walker |  | 1422 | 14 | Fs::DirListing { dir: .github/ISSUE_TEMPLATE } |  |  | 0.757 |
| ns | 1445 |  | 218 | test/ and docs/ top-level listings (complete) | 1.11 |  | 0.671 |
| walker |  | 1552 | 130 | Code::CodeKey { rung: Names, file: beets/dbcore/__init__.py, decl: 0, sub: 0, line: 0 } |  |  | 0.671 |
| walker |  | 1593 | 41 | Fs::DirListing { dir: docs/dev/plugins/other } |  |  | 0.671 |
| walker |  | 1664 | 71 | Code::CodeKey { rung: Names, file: beets/importer/__init__.py, decl: 0, sub: 0, line: 0 } |  |  | 0.672 |
| ns | 1828 |  | 383 | beetsplug/ bundled plugin listing (complete, 81 entries) | 1.12 |  | 0.551 |
| ns | 2006 |  | 178 | beets.library public export block (complete) | 2.1 |  | 0.532 |
| walker |  | 2047 | 383 | Fs::DirListing { dir: beetsplug } |  |  | 0.723 |
| walker |  | 2058 | 11 | Fs::DirListing { dir: beetsplug/bpd } |  |  | 0.723 |
| walker |  | 2070 | 12 | Fs::DirListing { dir: beetsplug/web } |  |  | 0.723 |
| walker |  | 2084 | 14 | Fs::DirListing { dir: beetsplug/discogs } |  |  | 0.723 |
| walker |  | 2100 | 16 | Fs::DirListing { dir: beetsplug/metasync } |  |  | 0.723 |
| walker |  | 2104 | 4 | Fs::DirListing { dir: beetsplug/web/templates } |  |  | 0.723 |
| walker |  | 2123 | 19 | Fs::DirListing { dir: beetsplug/tidal } |  |  | 0.723 |
| walker |  | 2139 | 16 | Code::CodeKey { rung: ModuleDoc, file: beetsplug/web/__init__.py, decl: 0, sub: 0, line: 0 } |  |  | 0.723 |
| walker |  | 2156 | 17 | Code::CodeKey { rung: ModuleDoc, file: beetsplug/metasync/__init__.py, decl: 0, sub: 0, line: 0 } |  |  | 0.723 |
| walker |  | 2179 | 23 | Fs::DirListing { dir: beetsplug/lastgenre } |  |  | 0.723 |
| ns | 2183 |  | 177 | The `beet` subcommand roster (default_commands) | 2.2 |  | 0.697 |
| walker |  | 2209 | 30 | Fs::DirListing { dir: beetsplug/_utils } |  |  | 0.697 |
| walker |  | 2248 | 39 | Code::CodeKey { rung: ModuleDoc, file: beetsplug/discogs/__init__.py, decl: 0, sub: 0, line: 0 } |  |  | 0.697 |
| walker |  | 2311 | 63 | Code::CodeKey { rung: ModuleDoc, file: beetsplug/bpd/__init__.py, decl: 0, sub: 0, line: 0 } |  |  | 0.697 |
| walker |  | 2333 | 22 | Fs::DirListing { dir: beetsplug/web/static } |  |  | 0.697 |
| ns | 2373 |  | 190 | Library class: models and schema migrations | 2.3 |  | 0.681 |
| walker |  | 2438 | 105 | Code::CodeKey { rung: ModuleDoc, file: beetsplug/lastgenre/__init__.py, decl: 0, sub: 0, line: 0 } |  |  | 0.681 |
| ns | 2528 |  | 155 | Library method roster (complete) | 2.4 | 2.3 | 0.668 |
| walker |  | 2649 | 211 | Code::CodeKey { rung: Names, file: beets/ui/commands/__init__.py, decl: 0, sub: 0, line: 0 } |  |  | 0.668 |
| walker |  | 2756 | 107 | Code::CodeKey { rung: Decl, file: beets/ui/commands/__init__.py, decl: 2, sub: 0, line: 50 } |  |  | 0.688 |
| walker |  | 2766 | 10 | Code::CodeKey { rung: Doc, file: beets/ui/commands/__init__.py, decl: 1, sub: 0, line: 36 } |  |  | 0.688 |
| ns | 2952 |  | 424 | models.py class headers: LibModel, FormattedItemMapping, Album, Item | 2.5 |  | 0.644 |
| walker |  | 2978 | 212 | Code::CodeKey { rung: Names, file: beetsplug/bpd/__init__.py, decl: 0, sub: 0, line: 0 } |  |  | 0.644 |
| ns | 3125 |  | 173 | beets.dbcore public export block + package docstring | 2.6 |  | 0.627 |
| walker |  | 3176 | 198 | Code::CodeKey { rung: Names, file: beets/ui/__init__.py, decl: 0, sub: 0, line: 0 } |  |  | 0.627 |
| walker |  | 3203 | 27 | Code::CodeKey { rung: Decl, file: beets/ui/__init__.py, decl: 2, sub: 0, line: 65 } |  |  | 0.627 |
| ns | 3298 |  | 173 | config_default.yaml: library, directory, plugins, ignore rules | 3.1 |  | 0.611 |
| walker |  | 3475 | 272 | Code::CodeKey { rung: Names, file: beetsplug/web/__init__.py, decl: 0, sub: 0, line: 0 } |  |  | 0.611 |
| walker |  | 3487 | 12 | Code::CodeKey { rung: Decl, file: beetsplug/web/__init__.py, decl: 1, sub: 0, line: 34 } |  |  | 0.611 |
| walker |  | 3509 | 22 | Code::CodeKey { rung: Decl, file: beetsplug/web/__init__.py, decl: 17, sub: 0, line: 283 } |  |  | 0.611 |
| walker |  | 3537 | 28 | Code::CodeKey { rung: Decl, file: beetsplug/web/__init__.py, decl: 11, sub: 0, line: 252 } |  |  | 0.611 |
| walker |  | 3565 | 28 | Code::CodeKey { rung: Decl, file: beetsplug/web/__init__.py, decl: 14, sub: 0, line: 268 } |  |  | 0.611 |
| ns | 3687 |  | 389 | config_default.yaml: the complete `import:` option block | 3.2 |  | 0.580 |
| walker |  | 3781 | 216 | Code::CodeKey { rung: Names, file: beetsplug/web/__init__.py, decl: 0, sub: 1, line: 0 } |  |  | 0.580 |
| ns | 3866 |  | 179 | config_default.yaml: tagging defaults, `paths:` templates, threading | 3.4 |  | 0.567 |
| walker |  | 4006 | 225 | Code::CodeKey { rung: Names, file: beetsplug/web/__init__.py, decl: 0, sub: 2, line: 0 } |  |  | 0.567 |
| walker |  | 4028 | 22 | Code::CodeKey { rung: Decl, file: beetsplug/web/__init__.py, decl: 34, sub: 0, line: 447 } |  |  | 0.567 |
| walker |  | 4061 | 33 | Code::CodeKey { rung: Decl, file: beetsplug/web/__init__.py, decl: 37, sub: 0, line: 520 } |  |  | 0.567 |
| ns | 4171 |  | 305 | Path-template function roster: DefaultTemplateFunctions (complete) | 3.5 |  | 0.555 |
| ns | 4291 |  | 120 | config_default.yaml: display formats and default sorts | 3.6 |  | 0.549 |
| walker |  | 4424 | 363 | Code::CodeKey { rung: Names, file: beetsplug/bpd/__init__.py, decl: 0, sub: 1, line: 0 } |  |  | 0.549 |
| walker |  | 4440 | 16 | Code::CodeKey { rung: Decl, file: beetsplug/bpd/__init__.py, decl: 36, sub: 0, line: 180 } |  |  | 0.549 |
| walker |  | 4474 | 34 | Code::CodeKey { rung: Decl, file: beetsplug/bpd/__init__.py, decl: 118, sub: 0, line: 1064 } |  |  | 0.549 |
| walker |  | 4527 | 53 | Code::CodeKey { rung: Decl, file: beetsplug/bpd/__init__.py, decl: 168, sub: 0, line: 1583 } |  |  | 0.549 |
| walker |  | 4588 | 61 | Code::CodeKey { rung: Decl, file: beetsplug/bpd/__init__.py, decl: 25, sub: 0, line: 80 } |  |  | 0.549 |
| walker |  | 4650 | 62 | Code::CodeKey { rung: Decl, file: beetsplug/bpd/__init__.py, decl: 27, sub: 0, line: 110 } |  |  | 0.549 |
| ns | 4673 |  | 382 | config_default.yaml: autotagger thresholds and distance weights | 3.7 |  | 0.528 |
| walker |  | 4729 | 79 | Code::CodeKey { rung: Decl, file: beetsplug/bpd/__init__.py, decl: 94, sub: 0, line: 754 } |  |  | 0.528 |
| walker |  | 4812 | 83 | Code::CodeKey { rung: Decl, file: beetsplug/bpd/__init__.py, decl: 100, sub: 0, line: 794 } |  |  | 0.528 |
| ns | 4813 |  | 140 | config_default.yaml: terminal UI, colour names, import layout | 3.8 |  | 0.519 |
| walker |  | 4901 | 89 | Code::CodeKey { rung: Decl, file: beetsplug/bpd/__init__.py, decl: 107, sub: 0, line: 905 } |  |  | 0.519 |
| ns | 4986 |  | 173 | Album method roster (complete, names only) | 4.1 | 2.5 | 0.511 |
| walker |  | 5004 | 103 | Code::CodeKey { rung: Decl, file: beetsplug/bpd/__init__.py, decl: 114, sub: 0, line: 960 } |  |  | 0.511 |
| walker |  | 5119 | 115 | Code::CodeKey { rung: Decl, file: beetsplug/bpd/__init__.py, decl: 26, sub: 0, line: 90 } |  |  | 0.511 |
| ns | 5290 |  | 304 | Item method roster (complete, names only) | 4.3 | 2.5 | 0.498 |
| walker |  | 5326 | 207 | Code::CodeKey { rung: Decl, file: beetsplug/bpd/__init__.py, decl: 38, sub: 0, line: 193 } |  |  | 0.498 |
| ns | 5458 |  | 168 | Item search fields and MediaFile-backed field sets | 4.4 | 2.5 | 0.491 |
| walker |  | 5529 | 203 | Code::CodeKey { rung: Decl, file: beetsplug/bpd/__init__.py, decl: 38, sub: 1, line: 193 } |  |  | 0.491 |
| ns | 5629 |  | 171 | LibModel shared-method roster (complete after `_fields`) | 4.5 | 2.5 | 0.485 |
| walker |  | 5727 | 198 | Code::CodeKey { rung: Decl, file: beetsplug/bpd/__init__.py, decl: 38, sub: 2, line: 193 } |  |  | 0.485 |
| ns | 5787 |  | 158 | beets.importer public exports + package docstring | 5.1 |  | 0.487 |
| walker |  | 6020 | 293 | Code::CodeKey { rung: Decl, file: beetsplug/bpd/__init__.py, decl: 38, sub: 3, line: 193 } |  |  | 0.487 |
| walker |  | 6029 | 9 | Code::CodeKey { rung: Doc, file: beetsplug/bpd/__init__.py, decl: 52, sub: 0, line: 346 } |  |  | 0.487 |
| walker |  | 6038 | 9 | Code::CodeKey { rung: Doc, file: beetsplug/bpd/__init__.py, decl: 90, sub: 0, line: 725 } |  |  | 0.487 |
| walker |  | 6048 | 10 | Code::CodeKey { rung: Doc, file: beetsplug/bpd/__init__.py, decl: 56, sub: 0, line: 365 } |  |  | 0.487 |
| walker |  | 6058 | 10 | Code::CodeKey { rung: Doc, file: beetsplug/bpd/__init__.py, decl: 72, sub: 0, line: 526 } |  |  | 0.487 |
| ns | 6147 |  | 360 | importer/stages.py: complete stage-function roster with section banners | 5.2 |  | 0.477 |
| walker |  | 6258 | 200 | Code::CodeKey { rung: Decl, file: beetsplug/bpd/__init__.py, decl: 121, sub: 0, line: 1099 } |  |  | 0.477 |
| ns | 6572 |  | 425 | ImportSession.run(): how the pipeline is assembled | 5.3 | 5.2 | 0.463 |
| walker |  | 6747 | 489 | Code::CodeKey { rung: Decl, file: beetsplug/bpd/__init__.py, decl: 121, sub: 1, line: 1099 } |  |  | 0.463 |
| ns | 6829 |  | 257 | ImportSession class contract (attributes + the four decision hooks) | 5.4 | 5.3 | 0.455 |
| walker |  | 6935 | 188 | Code::CodeKey { rung: Decl, file: beetsplug/bpd/__init__.py, decl: 121, sub: 2, line: 1099 } |  |  | 0.455 |
| ns | 7103 |  | 274 | importer/tasks.py: Action enum and the complete task class roster | 5.5 |  | 0.447 |
| walker |  | 7231 | 296 | Code::CodeKey { rung: Decl, file: beetsplug/bpd/__init__.py, decl: 121, sub: 3, line: 1099 } |  |  | 0.447 |
| walker |  | 7242 | 11 | Code::CodeKey { rung: Doc, file: beetsplug/bpd/__init__.py, decl: 55, sub: 0, line: 361 } |  |  | 0.447 |
| walker |  | 7253 | 11 | Code::CodeKey { rung: Doc, file: beetsplug/bpd/__init__.py, decl: 58, sub: 0, line: 386 } |  |  | 0.447 |
| walker |  | 7264 | 11 | Code::CodeKey { rung: Doc, file: beetsplug/bpd/__init__.py, decl: 112, sub: 0, line: 943 } |  |  | 0.447 |
| walker |  | 7276 | 12 | Code::CodeKey { rung: Doc, file: beetsplug/bpd/__init__.py, decl: 40, sub: 0, line: 241 } |  |  | 0.447 |
| walker |  | 7288 | 12 | Code::CodeKey { rung: Doc, file: beetsplug/bpd/__init__.py, decl: 54, sub: 0, line: 357 } |  |  | 0.447 |
| walker |  | 7300 | 12 | Code::CodeKey { rung: Doc, file: beetsplug/bpd/__init__.py, decl: 62, sub: 0, line: 458 } |  |  | 0.447 |
| walker |  | 7312 | 12 | Code::CodeKey { rung: Doc, file: beetsplug/bpd/__init__.py, decl: 63, sub: 0, line: 463 } |  |  | 0.447 |
| walker |  | 7324 | 12 | Code::CodeKey { rung: Doc, file: beetsplug/bpd/__init__.py, decl: 64, sub: 0, line: 468 } |  |  | 0.447 |
| walker |  | 7336 | 12 | Code::CodeKey { rung: Doc, file: beetsplug/bpd/__init__.py, decl: 70, sub: 0, line: 514 } |  |  | 0.447 |
| walker |  | 7348 | 12 | Code::CodeKey { rung: Doc, file: beetsplug/bpd/__init__.py, decl: 71, sub: 0, line: 522 } |  |  | 0.447 |
| ns | 7349 |  | 246 | BeetsPlugin base class: docstring and declared attributes | 6.1 |  | 0.441 |
| walker |  | 7360 | 12 | Code::CodeKey { rung: Doc, file: beetsplug/bpd/__init__.py, decl: 87, sub: 0, line: 689 } |  |  | 0.441 |
| walker |  | 7372 | 12 | Code::CodeKey { rung: Doc, file: beetsplug/bpd/__init__.py, decl: 160, sub: 0, line: 1518 } |  |  | 0.441 |
| walker |  | 7385 | 13 | Code::CodeKey { rung: Doc, file: beetsplug/bpd/__init__.py, decl: 45, sub: 0, line: 286 } |  |  | 0.441 |
| walker |  | 7398 | 13 | Code::CodeKey { rung: Doc, file: beetsplug/bpd/__init__.py, decl: 75, sub: 0, line: 552 } |  |  | 0.441 |
| walker |  | 7411 | 13 | Code::CodeKey { rung: Doc, file: beetsplug/bpd/__init__.py, decl: 86, sub: 0, line: 676 } |  |  | 0.441 |
| walker |  | 7424 | 13 | Code::CodeKey { rung: Doc, file: beetsplug/bpd/__init__.py, decl: 94, sub: 0, line: 754 } |  |  | 0.441 |
| walker |  | 7437 | 13 | Code::CodeKey { rung: Doc, file: beetsplug/bpd/__init__.py, decl: 170, sub: 0, line: 1601 } |  |  | 0.441 |
| walker |  | 7451 | 14 | Code::CodeKey { rung: Doc, file: beetsplug/bpd/__init__.py, decl: 57, sub: 0, line: 373 } |  |  | 0.441 |
| walker |  | 7465 | 14 | Code::CodeKey { rung: Doc, file: beetsplug/bpd/__init__.py, decl: 77, sub: 0, line: 577 } |  |  | 0.441 |
| walker |  | 7479 | 14 | Code::CodeKey { rung: Doc, file: beetsplug/bpd/__init__.py, decl: 96, sub: 0, line: 763 } |  |  | 0.441 |
| walker |  | 7493 | 14 | Code::CodeKey { rung: Doc, file: beetsplug/bpd/__init__.py, decl: 103, sub: 0, line: 816 } |  |  | 0.441 |
| walker |  | 7507 | 14 | Code::CodeKey { rung: Doc, file: beetsplug/bpd/__init__.py, decl: 105, sub: 0, line: 825 } |  |  | 0.441 |
| walker |  | 7521 | 14 | Code::CodeKey { rung: Doc, file: beetsplug/bpd/__init__.py, decl: 113, sub: 0, line: 950 } |  |  | 0.441 |
| walker |  | 7535 | 14 | Code::CodeKey { rung: Doc, file: beetsplug/bpd/__init__.py, decl: 145, sub: 0, line: 1416 } |  |  | 0.441 |
| walker |  | 7549 | 14 | Code::CodeKey { rung: Doc, file: beetsplug/bpd/__init__.py, decl: 146, sub: 0, line: 1424 } |  |  | 0.441 |
| walker |  | 7564 | 15 | Code::CodeKey { rung: Doc, file: beetsplug/bpd/__init__.py, decl: 41, sub: 0, line: 245 } |  |  | 0.441 |
| walker |  | 7579 | 15 | Code::CodeKey { rung: Doc, file: beetsplug/bpd/__init__.py, decl: 61, sub: 0, line: 453 } |  |  | 0.441 |
| walker |  | 7594 | 15 | Code::CodeKey { rung: Doc, file: beetsplug/bpd/__init__.py, decl: 69, sub: 0, line: 505 } |  |  | 0.441 |
| ns | 7595 |  | 246 | BeetsPlugin method roster (complete) | 6.2 | 6.1 | 0.436 |
| walker |  | 7609 | 15 | Code::CodeKey { rung: Doc, file: beetsplug/bpd/__init__.py, decl: 73, sub: 0, line: 533 } |  |  | 0.436 |
| walker |  | 7624 | 15 | Code::CodeKey { rung: Doc, file: beetsplug/bpd/__init__.py, decl: 84, sub: 0, line: 648 } |  |  | 0.436 |
| walker |  | 7639 | 15 | Code::CodeKey { rung: Doc, file: beetsplug/bpd/__init__.py, decl: 85, sub: 0, line: 654 } |  |  | 0.436 |
| walker |  | 7654 | 15 | Code::CodeKey { rung: Doc, file: beetsplug/bpd/__init__.py, decl: 130, sub: 0, line: 1204 } |  |  | 0.436 |
| walker |  | 7669 | 15 | Code::CodeKey { rung: Doc, file: beetsplug/bpd/__init__.py, decl: 140, sub: 0, line: 1314 } |  |  | 0.436 |
| walker |  | 7684 | 15 | Code::CodeKey { rung: Doc, file: beetsplug/web/__init__.py, decl: 6, sub: 0, line: 117 } |  |  | 0.436 |
| walker |  | 7698 | 14 | Code::CodeKey { rung: Names, file: beetsplug/_utils/__init__.py, decl: 0, sub: 0, line: 0 } |  |  | 0.436 |
| walker |  | 7800 | 102 | Code::CodeKey { rung: Names, file: beetsplug/metasync/__init__.py, decl: 0, sub: 0, line: 0 } |  |  | 0.436 |
| walker |  | 7829 | 29 | Code::CodeKey { rung: Decl, file: beetsplug/metasync/__init__.py, decl: 2, sub: 0, line: 34 } |  |  | 0.436 |
| walker |  | 7885 | 56 | Code::CodeKey { rung: Decl, file: beetsplug/metasync/__init__.py, decl: 9, sub: 0, line: 76 } |  |  | 0.436 |
| walker |  | 7946 | 61 | Code::CodeKey { rung: Decl, file: beetsplug/metasync/__init__.py, decl: 3, sub: 0, line: 40 } |  |  | 0.436 |
| walker |  | 7951 | 5 | Code::CodeKey { rung: Body, file: beetsplug/metasync/__init__.py, decl: 5, sub: 0, line: 47 } |  |  | 0.436 |
| walker |  | 7958 | 7 | Code::CodeKey { rung: Body, file: beetsplug/metasync/__init__.py, decl: 10, sub: 0, line: 79 } |  |  | 0.436 |
| walker |  | 7972 | 14 | Code::CodeKey { rung: Doc, file: beetsplug/metasync/__init__.py, decl: 12, sub: 0, line: 104 } |  |  | 0.436 |
| walker |  | 7988 | 16 | Code::CodeKey { rung: Doc, file: beetsplug/bpd/__init__.py, decl: 67, sub: 0, line: 487 } |  |  | 0.436 |
| walker |  | 8004 | 16 | Code::CodeKey { rung: Doc, file: beetsplug/bpd/__init__.py, decl: 79, sub: 0, line: 604 } |  |  | 0.436 |
| ns | 8017 |  | 422 | The complete `EventType` literal — every plugin event name | 6.3 | 6.1 | 0.424 |
| walker |  | 8020 | 16 | Code::CodeKey { rung: Doc, file: beetsplug/bpd/__init__.py, decl: 88, sub: 0, line: 697 } |  |  | 0.424 |
| walker |  | 8036 | 16 | Code::CodeKey { rung: Doc, file: beetsplug/bpd/__init__.py, decl: 91, sub: 0, line: 731 } |  |  | 0.424 |
| walker |  | 8052 | 16 | Code::CodeKey { rung: Doc, file: beetsplug/bpd/__init__.py, decl: 100, sub: 0, line: 794 } |  |  | 0.424 |
| walker |  | 8068 | 16 | Code::CodeKey { rung: Doc, file: beetsplug/bpd/__init__.py, decl: 104, sub: 0, line: 821 } |  |  | 0.424 |
| walker |  | 8084 | 16 | Code::CodeKey { rung: Doc, file: beetsplug/bpd/__init__.py, decl: 111, sub: 0, line: 939 } |  |  | 0.424 |
| walker |  | 8100 | 16 | Code::CodeKey { rung: Doc, file: beetsplug/bpd/__init__.py, decl: 114, sub: 0, line: 960 } |  |  | 0.424 |
| walker |  | 8116 | 16 | Code::CodeKey { rung: Doc, file: beetsplug/bpd/__init__.py, decl: 120, sub: 0, line: 1079 } |  |  | 0.424 |
| walker |  | 8132 | 16 | Code::CodeKey { rung: Doc, file: beetsplug/bpd/__init__.py, decl: 141, sub: 0, line: 1336 } |  |  | 0.424 |
| ns | 8321 |  | 304 | plugins.py module-level API roster (complete) | 6.4 |  | 0.419 |
| walker |  | 8354 | 222 | Code::CodeKey { rung: Names, file: beets/ui/__init__.py, decl: 0, sub: 1, line: 0 } |  |  | 0.419 |
| walker |  | 8411 | 57 | Code::CodeKey { rung: Decl, file: beets/ui/__init__.py, decl: 13, sub: 0, line: 207 } |  |  | 0.419 |
| walker |  | 8473 | 62 | Code::CodeKey { rung: Decl, file: beets/ui/__init__.py, decl: 19, sub: 0, line: 466 } |  |  | 0.419 |
| walker |  | 8594 | 121 | Code::CodeKey { rung: Decl, file: beets/ui/__init__.py, decl: 20, sub: 0, line: 498 } |  |  | 0.419 |
| ns | 8633 |  | 312 | MetadataSourcePlugin: the metadata-backend contract | 6.5 |  | 0.413 |
| walker |  | 8653 | 59 | Code::CodeKey { rung: Decl, file: beets/ui/__init__.py, decl: 23, sub: 0, line: 534 } |  |  | 0.413 |
| walker |  | 8667 | 14 | Code::CodeKey { rung: Doc, file: beets/ui/__init__.py, decl: 26, sub: 0, line: 621 } |  |  | 0.413 |
| walker |  | 8807 | 140 | Code::CodeKey { rung: Names, file: beets/ui/__init__.py, decl: 0, sub: 2, line: 0 } |  |  | 0.413 |
| walker |  | 8816 | 9 | Code::CodeKey { rung: Decl, file: beets/ui/__init__.py, decl: 40, sub: 0, line: 807 } |  |  | 0.413 |
| ns | 8830 |  | 197 | Query-string prefixes: how `beet ls artist:foo` is parsed | 7.1 |  | 0.409 |
| walker |  | 8914 | 98 | Code::CodeKey { rung: Decl, file: beets/ui/__init__.py, decl: 33, sub: 0, line: 676 } |  |  | 0.409 |
| walker |  | 9037 | 123 | Code::CodeKey { rung: Decl, file: beets/ui/__init__.py, decl: 27, sub: 0, line: 637 } |  |  | 0.409 |
| walker |  | 9052 | 15 | Code::CodeKey { rung: Doc, file: beets/ui/__init__.py, decl: 43, sub: 0, line: 879 } |  |  | 0.409 |
| walker |  | 9068 | 16 | Code::CodeKey { rung: Doc, file: beets/ui/__init__.py, decl: 17, sub: 0, line: 445 } |  |  | 0.409 |
| walker |  | 9084 | 16 | Code::CodeKey { rung: Doc, file: beets/ui/__init__.py, decl: 18, sub: 0, line: 459 } |  |  | 0.409 |
| walker |  | 9145 | 61 | Code::CodeKey { rung: Names, file: beets/__init__.py, decl: 0, sub: 0, line: 0 } |  |  | 0.409 |
| walker |  | 9171 | 26 | Code::CodeKey { rung: Decl, file: beets/__init__.py, decl: 2, sub: 0, line: 35 } |  |  | 0.409 |
| walker |  | 9181 | 10 | Code::CodeKey { rung: Doc, file: beets/__init__.py, decl: 1, sub: 0, line: 26 } |  |  | 0.409 |
| ns | 9187 |  | 357 | dbcore/query.py class roster (complete) | 7.2 |  | 0.401 |
| walker |  | 9198 | 17 | Code::CodeKey { rung: Doc, file: beets/ui/__init__.py, decl: 36, sub: 0, line: 708 } |  |  | 0.401 |
| walker |  | 9215 | 17 | Code::CodeKey { rung: Doc, file: beets/ui/__init__.py, decl: 41, sub: 0, line: 830 } |  |  | 0.401 |
| walker |  | 9232 | 17 | Code::CodeKey { rung: Doc, file: beetsplug/bpd/__init__.py, decl: 43, sub: 0, line: 269 } |  |  | 0.401 |
| walker |  | 9249 | 17 | Code::CodeKey { rung: Doc, file: beetsplug/bpd/__init__.py, decl: 47, sub: 0, line: 297 } |  |  | 0.401 |
| walker |  | 9266 | 17 | Code::CodeKey { rung: Doc, file: beetsplug/bpd/__init__.py, decl: 65, sub: 0, line: 474 } |  |  | 0.401 |
| walker |  | 9283 | 17 | Code::CodeKey { rung: Doc, file: beetsplug/bpd/__init__.py, decl: 66, sub: 0, line: 482 } |  |  | 0.401 |
| walker |  | 9300 | 17 | Code::CodeKey { rung: Doc, file: beetsplug/bpd/__init__.py, decl: 107, sub: 0, line: 905 } |  |  | 0.401 |
| walker |  | 9317 | 17 | Code::CodeKey { rung: Doc, file: beetsplug/bpd/__init__.py, decl: 128, sub: 0, line: 1167 } |  |  | 0.401 |
| walker |  | 9334 | 17 | Code::CodeKey { rung: Doc, file: beetsplug/bpd/__init__.py, decl: 135, sub: 0, line: 1258 } |  |  | 0.401 |
| walker |  | 9351 | 17 | Code::CodeKey { rung: Doc, file: beetsplug/web/__init__.py, decl: 4, sub: 0, line: 103 } |  |  | 0.401 |
| walker |  | 9368 | 17 | Code::CodeKey { rung: Doc, file: beetsplug/web/__init__.py, decl: 11, sub: 0, line: 252 } |  |  | 0.401 |
| walker |  | 9496 | 128 | Code::CodeKey { rung: Names, file: beetsplug/discogs/__init__.py, decl: 0, sub: 0, line: 0 } |  |  | 0.401 |
| ns | 9539 |  | 352 | beets.autotag exports, Recommendation, Proposal | 8.1 |  | 0.394 |
| walker |  | 9552 | 56 | Code::CodeKey { rung: Decl, file: beetsplug/discogs/__init__.py, decl: 4, sub: 0, line: 60 } |  |  | 0.394 |
| walker |  | 9623 | 71 | Code::CodeKey { rung: Decl, file: beetsplug/discogs/__init__.py, decl: 6, sub: 0, line: 83 } |  |  | 0.394 |
| ns | 9775 |  | 236 | autotag/hooks.py and distance.py symbol roster | 8.2 |  | 0.390 |
| walker |  | 9813 | 190 | Code::CodeKey { rung: Decl, file: beetsplug/discogs/__init__.py, decl: 5, sub: 0, line: 68 } |  |  | 0.390 |
| ns | 9970 |  | 195 | beets/util/__init__.py: classes and the core path/file-operation helpers | 9.1 |  | 0.385 |
| walker |  | 9982 | 169 | Code::CodeKey { rung: Decl, file: beetsplug/discogs/__init__.py, decl: 7, sub: 0, line: 93 } |  |  | 0.385 |
