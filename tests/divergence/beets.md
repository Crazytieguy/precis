Score(3000)=0.661 I=0.799 C=0.546 ns_rows≤3K=17/44 grid(1000/1442/2080/3000/4327/6240/9000)=0.594/0.757/0.585/0.661/0.603/0.525/0.450

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
| walker |  | 1693 | 271 | Code::CodeKey { rung: Names, file: beets/dbcore/__init__.py, decl: 0, sub: 0, line: 0 } |  |  | 0.674 |
| walker |  | 1734 | 41 | Fs::DirListing { dir: docs/dev/plugins/other } |  |  | 0.674 |
| ns | 1828 |  | 383 | beetsplug/ bundled plugin listing (complete, 81 entries) | 1.12 |  | 0.553 |
| walker |  | 1886 | 152 | Code::CodeKey { rung: Names, file: beets/importer/__init__.py, decl: 0, sub: 0, line: 0 } |  |  | 0.554 |
| ns | 2006 |  | 178 | beets.library public export block (complete) | 2.1 |  | 0.535 |
| ns | 2183 |  | 177 | The `beet` subcommand roster (default_commands) | 2.2 |  | 0.515 |
| walker |  | 2269 | 383 | Fs::DirListing { dir: beetsplug } |  |  | 0.700 |
| walker |  | 2280 | 11 | Fs::DirListing { dir: beetsplug/bpd } |  |  | 0.700 |
| walker |  | 2292 | 12 | Fs::DirListing { dir: beetsplug/web } |  |  | 0.700 |
| walker |  | 2306 | 14 | Fs::DirListing { dir: beetsplug/discogs } |  |  | 0.700 |
| walker |  | 2322 | 16 | Fs::DirListing { dir: beetsplug/metasync } |  |  | 0.700 |
| walker |  | 2326 | 4 | Fs::DirListing { dir: beetsplug/web/templates } |  |  | 0.700 |
| walker |  | 2345 | 19 | Fs::DirListing { dir: beetsplug/tidal } |  |  | 0.700 |
| walker |  | 2361 | 16 | Code::CodeKey { rung: ModuleDoc, file: beetsplug/web/__init__.py, decl: 0, sub: 0, line: 0 } |  |  | 0.700 |
| ns | 2373 |  | 190 | Library class: models and schema migrations | 2.3 |  | 0.684 |
| walker |  | 2378 | 17 | Code::CodeKey { rung: ModuleDoc, file: beetsplug/metasync/__init__.py, decl: 0, sub: 0, line: 0 } |  |  | 0.684 |
| walker |  | 2401 | 23 | Fs::DirListing { dir: beetsplug/lastgenre } |  |  | 0.684 |
| walker |  | 2431 | 30 | Fs::DirListing { dir: beetsplug/_utils } |  |  | 0.684 |
| walker |  | 2470 | 39 | Code::CodeKey { rung: ModuleDoc, file: beetsplug/discogs/__init__.py, decl: 0, sub: 0, line: 0 } |  |  | 0.684 |
| ns | 2528 |  | 155 | Library method roster (complete) | 2.4 | 2.3 | 0.671 |
| walker |  | 2533 | 63 | Code::CodeKey { rung: ModuleDoc, file: beetsplug/bpd/__init__.py, decl: 0, sub: 0, line: 0 } |  |  | 0.671 |
| walker |  | 2555 | 22 | Fs::DirListing { dir: beetsplug/web/static } |  |  | 0.671 |
| walker |  | 2660 | 105 | Code::CodeKey { rung: ModuleDoc, file: beetsplug/lastgenre/__init__.py, decl: 0, sub: 0, line: 0 } |  |  | 0.671 |
| walker |  | 2871 | 211 | Code::CodeKey { rung: Names, file: beets/library/__init__.py, decl: 0, sub: 0, line: 0 } |  |  | 0.706 |
| walker |  | 2938 | 67 | Code::CodeKey { rung: Decl, file: beets/library/__init__.py, decl: 1, sub: 0, line: 8 } |  |  | 0.706 |
| ns | 2952 |  | 424 | models.py class headers: LibModel, FormattedItemMapping, Album, Item | 2.5 |  | 0.661 |
| ns | 3125 |  | 173 | beets.dbcore public export block + package docstring | 2.6 |  | 0.672 |
| walker |  | 3149 | 211 | Code::CodeKey { rung: Names, file: beets/ui/commands/__init__.py, decl: 0, sub: 0, line: 0 } |  |  | 0.672 |
| walker |  | 3256 | 107 | Code::CodeKey { rung: Decl, file: beets/ui/commands/__init__.py, decl: 2, sub: 0, line: 50 } |  |  | 0.689 |
| walker |  | 3266 | 10 | Code::CodeKey { rung: Doc, file: beets/ui/commands/__init__.py, decl: 1, sub: 0, line: 36 } |  |  | 0.689 |
| ns | 3298 |  | 173 | config_default.yaml: library, directory, plugins, ignore rules | 3.1 |  | 0.671 |
| walker |  | 3443 | 177 | Code::CodeKey { rung: Names, file: beets/autotag/__init__.py, decl: 0, sub: 0, line: 0 } |  |  | 0.671 |
| walker |  | 3655 | 212 | Code::CodeKey { rung: Names, file: beetsplug/bpd/__init__.py, decl: 0, sub: 0, line: 0 } |  |  | 0.671 |
| ns | 3687 |  | 389 | config_default.yaml: the complete `import:` option block | 3.2 |  | 0.637 |
| walker |  | 3853 | 198 | Code::CodeKey { rung: Names, file: beets/ui/__init__.py, decl: 0, sub: 0, line: 0 } |  |  | 0.637 |
| ns | 3866 |  | 179 | config_default.yaml: tagging defaults, `paths:` templates, threading | 3.4 |  | 0.624 |
| walker |  | 3880 | 27 | Code::CodeKey { rung: Decl, file: beets/ui/__init__.py, decl: 2, sub: 0, line: 65 } |  |  | 0.624 |
| walker |  | 4152 | 272 | Code::CodeKey { rung: Names, file: beetsplug/web/__init__.py, decl: 0, sub: 0, line: 0 } |  |  | 0.624 |
| walker |  | 4164 | 12 | Code::CodeKey { rung: Decl, file: beetsplug/web/__init__.py, decl: 1, sub: 0, line: 34 } |  |  | 0.624 |
| ns | 4171 |  | 305 | Path-template function roster: DefaultTemplateFunctions (complete) | 3.5 |  | 0.610 |
| walker |  | 4186 | 22 | Code::CodeKey { rung: Decl, file: beetsplug/web/__init__.py, decl: 17, sub: 0, line: 283 } |  |  | 0.610 |
| walker |  | 4214 | 28 | Code::CodeKey { rung: Decl, file: beetsplug/web/__init__.py, decl: 11, sub: 0, line: 252 } |  |  | 0.610 |
| walker |  | 4242 | 28 | Code::CodeKey { rung: Decl, file: beetsplug/web/__init__.py, decl: 14, sub: 0, line: 268 } |  |  | 0.610 |
| ns | 4291 |  | 120 | config_default.yaml: display formats and default sorts | 3.6 |  | 0.603 |
| walker |  | 4458 | 216 | Code::CodeKey { rung: Names, file: beetsplug/web/__init__.py, decl: 0, sub: 1, line: 0 } |  |  | 0.603 |
| ns | 4673 |  | 382 | config_default.yaml: autotagger thresholds and distance weights | 3.7 |  | 0.581 |
| walker |  | 4683 | 225 | Code::CodeKey { rung: Names, file: beetsplug/web/__init__.py, decl: 0, sub: 2, line: 0 } |  |  | 0.581 |
| walker |  | 4705 | 22 | Code::CodeKey { rung: Decl, file: beetsplug/web/__init__.py, decl: 34, sub: 0, line: 447 } |  |  | 0.581 |
| walker |  | 4738 | 33 | Code::CodeKey { rung: Decl, file: beetsplug/web/__init__.py, decl: 37, sub: 0, line: 520 } |  |  | 0.581 |
| ns | 4813 |  | 140 | config_default.yaml: terminal UI, colour names, import layout | 3.8 |  | 0.570 |
| ns | 4986 |  | 173 | Album method roster (complete, names only) | 4.1 | 2.5 | 0.561 |
| walker |  | 5101 | 363 | Code::CodeKey { rung: Names, file: beetsplug/bpd/__init__.py, decl: 0, sub: 1, line: 0 } |  |  | 0.561 |
| walker |  | 5117 | 16 | Code::CodeKey { rung: Decl, file: beetsplug/bpd/__init__.py, decl: 36, sub: 0, line: 180 } |  |  | 0.561 |
| walker |  | 5151 | 34 | Code::CodeKey { rung: Decl, file: beetsplug/bpd/__init__.py, decl: 118, sub: 0, line: 1064 } |  |  | 0.561 |
| walker |  | 5204 | 53 | Code::CodeKey { rung: Decl, file: beetsplug/bpd/__init__.py, decl: 168, sub: 0, line: 1583 } |  |  | 0.561 |
| walker |  | 5265 | 61 | Code::CodeKey { rung: Decl, file: beetsplug/bpd/__init__.py, decl: 25, sub: 0, line: 80 } |  |  | 0.561 |
| ns | 5290 |  | 304 | Item method roster (complete, names only) | 4.3 | 2.5 | 0.547 |
| walker |  | 5327 | 62 | Code::CodeKey { rung: Decl, file: beetsplug/bpd/__init__.py, decl: 27, sub: 0, line: 110 } |  |  | 0.547 |
| walker |  | 5406 | 79 | Code::CodeKey { rung: Decl, file: beetsplug/bpd/__init__.py, decl: 94, sub: 0, line: 754 } |  |  | 0.547 |
| ns | 5458 |  | 168 | Item search fields and MediaFile-backed field sets | 4.4 | 2.5 | 0.540 |
| walker |  | 5489 | 83 | Code::CodeKey { rung: Decl, file: beetsplug/bpd/__init__.py, decl: 100, sub: 0, line: 794 } |  |  | 0.540 |
| walker |  | 5578 | 89 | Code::CodeKey { rung: Decl, file: beetsplug/bpd/__init__.py, decl: 107, sub: 0, line: 905 } |  |  | 0.540 |
| ns | 5629 |  | 171 | LibModel shared-method roster (complete after `_fields`) | 4.5 | 2.5 | 0.533 |
| walker |  | 5681 | 103 | Code::CodeKey { rung: Decl, file: beetsplug/bpd/__init__.py, decl: 114, sub: 0, line: 960 } |  |  | 0.533 |
| ns | 5787 |  | 158 | beets.importer public exports + package docstring | 5.1 |  | 0.536 |
| walker |  | 5796 | 115 | Code::CodeKey { rung: Decl, file: beetsplug/bpd/__init__.py, decl: 26, sub: 0, line: 90 } |  |  | 0.536 |
| walker |  | 6003 | 207 | Code::CodeKey { rung: Decl, file: beetsplug/bpd/__init__.py, decl: 38, sub: 0, line: 193 } |  |  | 0.536 |
| ns | 6147 |  | 360 | importer/stages.py: complete stage-function roster with section banners | 5.2 |  | 0.525 |
| walker |  | 6206 | 203 | Code::CodeKey { rung: Decl, file: beetsplug/bpd/__init__.py, decl: 38, sub: 1, line: 193 } |  |  | 0.525 |
| walker |  | 6404 | 198 | Code::CodeKey { rung: Decl, file: beetsplug/bpd/__init__.py, decl: 38, sub: 2, line: 193 } |  |  | 0.525 |
| ns | 6572 |  | 425 | ImportSession.run(): how the pipeline is assembled | 5.3 | 5.2 | 0.510 |
| walker |  | 6697 | 293 | Code::CodeKey { rung: Decl, file: beetsplug/bpd/__init__.py, decl: 38, sub: 3, line: 193 } |  |  | 0.510 |
| walker |  | 6706 | 9 | Code::CodeKey { rung: Doc, file: beetsplug/bpd/__init__.py, decl: 52, sub: 0, line: 346 } |  |  | 0.510 |
| walker |  | 6715 | 9 | Code::CodeKey { rung: Doc, file: beetsplug/bpd/__init__.py, decl: 90, sub: 0, line: 725 } |  |  | 0.510 |
| walker |  | 6725 | 10 | Code::CodeKey { rung: Doc, file: beetsplug/bpd/__init__.py, decl: 56, sub: 0, line: 365 } |  |  | 0.510 |
| walker |  | 6735 | 10 | Code::CodeKey { rung: Doc, file: beetsplug/bpd/__init__.py, decl: 72, sub: 0, line: 526 } |  |  | 0.510 |
| ns | 6829 |  | 257 | ImportSession class contract (attributes + the four decision hooks) | 5.4 | 5.3 | 0.501 |
| walker |  | 6935 | 200 | Code::CodeKey { rung: Decl, file: beetsplug/bpd/__init__.py, decl: 121, sub: 0, line: 1099 } |  |  | 0.501 |
| ns | 7103 |  | 274 | importer/tasks.py: Action enum and the complete task class roster | 5.5 |  | 0.492 |
| ns | 7349 |  | 246 | BeetsPlugin base class: docstring and declared attributes | 6.1 |  | 0.485 |
| walker |  | 7424 | 489 | Code::CodeKey { rung: Decl, file: beetsplug/bpd/__init__.py, decl: 121, sub: 1, line: 1099 } |  |  | 0.485 |
| ns | 7595 |  | 246 | BeetsPlugin method roster (complete) | 6.2 | 6.1 | 0.480 |
| walker |  | 7612 | 188 | Code::CodeKey { rung: Decl, file: beetsplug/bpd/__init__.py, decl: 121, sub: 2, line: 1099 } |  |  | 0.480 |
| walker |  | 7908 | 296 | Code::CodeKey { rung: Decl, file: beetsplug/bpd/__init__.py, decl: 121, sub: 3, line: 1099 } |  |  | 0.480 |
| walker |  | 7919 | 11 | Code::CodeKey { rung: Doc, file: beetsplug/bpd/__init__.py, decl: 55, sub: 0, line: 361 } |  |  | 0.480 |
| walker |  | 7930 | 11 | Code::CodeKey { rung: Doc, file: beetsplug/bpd/__init__.py, decl: 58, sub: 0, line: 386 } |  |  | 0.480 |
| walker |  | 7941 | 11 | Code::CodeKey { rung: Doc, file: beetsplug/bpd/__init__.py, decl: 112, sub: 0, line: 943 } |  |  | 0.480 |
| walker |  | 7953 | 12 | Code::CodeKey { rung: Doc, file: beetsplug/bpd/__init__.py, decl: 40, sub: 0, line: 241 } |  |  | 0.480 |
| walker |  | 7965 | 12 | Code::CodeKey { rung: Doc, file: beetsplug/bpd/__init__.py, decl: 54, sub: 0, line: 357 } |  |  | 0.480 |
| walker |  | 7977 | 12 | Code::CodeKey { rung: Doc, file: beetsplug/bpd/__init__.py, decl: 62, sub: 0, line: 458 } |  |  | 0.480 |
| walker |  | 7989 | 12 | Code::CodeKey { rung: Doc, file: beetsplug/bpd/__init__.py, decl: 63, sub: 0, line: 463 } |  |  | 0.480 |
| walker |  | 8001 | 12 | Code::CodeKey { rung: Doc, file: beetsplug/bpd/__init__.py, decl: 64, sub: 0, line: 468 } |  |  | 0.480 |
| walker |  | 8013 | 12 | Code::CodeKey { rung: Doc, file: beetsplug/bpd/__init__.py, decl: 70, sub: 0, line: 514 } |  |  | 0.480 |
| ns | 8017 |  | 422 | The complete `EventType` literal — every plugin event name | 6.3 | 6.1 | 0.467 |
| walker |  | 8025 | 12 | Code::CodeKey { rung: Doc, file: beetsplug/bpd/__init__.py, decl: 71, sub: 0, line: 522 } |  |  | 0.467 |
| walker |  | 8037 | 12 | Code::CodeKey { rung: Doc, file: beetsplug/bpd/__init__.py, decl: 87, sub: 0, line: 689 } |  |  | 0.467 |
| walker |  | 8049 | 12 | Code::CodeKey { rung: Doc, file: beetsplug/bpd/__init__.py, decl: 160, sub: 0, line: 1518 } |  |  | 0.467 |
| walker |  | 8062 | 13 | Code::CodeKey { rung: Doc, file: beetsplug/bpd/__init__.py, decl: 45, sub: 0, line: 286 } |  |  | 0.467 |
| walker |  | 8075 | 13 | Code::CodeKey { rung: Doc, file: beetsplug/bpd/__init__.py, decl: 75, sub: 0, line: 552 } |  |  | 0.467 |
| walker |  | 8088 | 13 | Code::CodeKey { rung: Doc, file: beetsplug/bpd/__init__.py, decl: 86, sub: 0, line: 676 } |  |  | 0.467 |
| walker |  | 8101 | 13 | Code::CodeKey { rung: Doc, file: beetsplug/bpd/__init__.py, decl: 94, sub: 0, line: 754 } |  |  | 0.467 |
| walker |  | 8114 | 13 | Code::CodeKey { rung: Doc, file: beetsplug/bpd/__init__.py, decl: 170, sub: 0, line: 1601 } |  |  | 0.467 |
| walker |  | 8128 | 14 | Code::CodeKey { rung: Doc, file: beetsplug/bpd/__init__.py, decl: 57, sub: 0, line: 373 } |  |  | 0.467 |
| walker |  | 8142 | 14 | Code::CodeKey { rung: Doc, file: beetsplug/bpd/__init__.py, decl: 77, sub: 0, line: 577 } |  |  | 0.467 |
| walker |  | 8156 | 14 | Code::CodeKey { rung: Doc, file: beetsplug/bpd/__init__.py, decl: 96, sub: 0, line: 763 } |  |  | 0.467 |
| walker |  | 8170 | 14 | Code::CodeKey { rung: Doc, file: beetsplug/bpd/__init__.py, decl: 103, sub: 0, line: 816 } |  |  | 0.467 |
| walker |  | 8184 | 14 | Code::CodeKey { rung: Doc, file: beetsplug/bpd/__init__.py, decl: 105, sub: 0, line: 825 } |  |  | 0.467 |
| walker |  | 8198 | 14 | Code::CodeKey { rung: Doc, file: beetsplug/bpd/__init__.py, decl: 113, sub: 0, line: 950 } |  |  | 0.467 |
| walker |  | 8212 | 14 | Code::CodeKey { rung: Doc, file: beetsplug/bpd/__init__.py, decl: 145, sub: 0, line: 1416 } |  |  | 0.467 |
| walker |  | 8226 | 14 | Code::CodeKey { rung: Doc, file: beetsplug/bpd/__init__.py, decl: 146, sub: 0, line: 1424 } |  |  | 0.467 |
| walker |  | 8241 | 15 | Code::CodeKey { rung: Doc, file: beetsplug/bpd/__init__.py, decl: 41, sub: 0, line: 245 } |  |  | 0.467 |
| walker |  | 8256 | 15 | Code::CodeKey { rung: Doc, file: beetsplug/bpd/__init__.py, decl: 61, sub: 0, line: 453 } |  |  | 0.467 |
| walker |  | 8271 | 15 | Code::CodeKey { rung: Doc, file: beetsplug/bpd/__init__.py, decl: 69, sub: 0, line: 505 } |  |  | 0.467 |
| walker |  | 8286 | 15 | Code::CodeKey { rung: Doc, file: beetsplug/bpd/__init__.py, decl: 73, sub: 0, line: 533 } |  |  | 0.467 |
| walker |  | 8301 | 15 | Code::CodeKey { rung: Doc, file: beetsplug/bpd/__init__.py, decl: 84, sub: 0, line: 648 } |  |  | 0.467 |
| walker |  | 8316 | 15 | Code::CodeKey { rung: Doc, file: beetsplug/bpd/__init__.py, decl: 85, sub: 0, line: 654 } |  |  | 0.467 |
| ns | 8321 |  | 304 | plugins.py module-level API roster (complete) | 6.4 |  | 0.461 |
| walker |  | 8331 | 15 | Code::CodeKey { rung: Doc, file: beetsplug/bpd/__init__.py, decl: 130, sub: 0, line: 1204 } |  |  | 0.461 |
| walker |  | 8346 | 15 | Code::CodeKey { rung: Doc, file: beetsplug/bpd/__init__.py, decl: 140, sub: 0, line: 1314 } |  |  | 0.461 |
| walker |  | 8361 | 15 | Code::CodeKey { rung: Doc, file: beetsplug/web/__init__.py, decl: 6, sub: 0, line: 117 } |  |  | 0.461 |
| walker |  | 8463 | 102 | Code::CodeKey { rung: Names, file: beetsplug/metasync/__init__.py, decl: 0, sub: 0, line: 0 } |  |  | 0.461 |
| walker |  | 8492 | 29 | Code::CodeKey { rung: Decl, file: beetsplug/metasync/__init__.py, decl: 2, sub: 0, line: 34 } |  |  | 0.461 |
| walker |  | 8548 | 56 | Code::CodeKey { rung: Decl, file: beetsplug/metasync/__init__.py, decl: 9, sub: 0, line: 76 } |  |  | 0.461 |
| walker |  | 8609 | 61 | Code::CodeKey { rung: Decl, file: beetsplug/metasync/__init__.py, decl: 3, sub: 0, line: 40 } |  |  | 0.461 |
| walker |  | 8614 | 5 | Code::CodeKey { rung: Body, file: beetsplug/metasync/__init__.py, decl: 5, sub: 0, line: 47 } |  |  | 0.461 |
| walker |  | 8621 | 7 | Code::CodeKey { rung: Body, file: beetsplug/metasync/__init__.py, decl: 10, sub: 0, line: 79 } |  |  | 0.461 |
| ns | 8633 |  | 312 | MetadataSourcePlugin: the metadata-backend contract | 6.5 |  | 0.455 |
| walker |  | 8635 | 14 | Code::CodeKey { rung: Doc, file: beetsplug/metasync/__init__.py, decl: 12, sub: 0, line: 104 } |  |  | 0.455 |
| walker |  | 8651 | 16 | Code::CodeKey { rung: Doc, file: beetsplug/bpd/__init__.py, decl: 67, sub: 0, line: 487 } |  |  | 0.455 |
| walker |  | 8667 | 16 | Code::CodeKey { rung: Doc, file: beetsplug/bpd/__init__.py, decl: 79, sub: 0, line: 604 } |  |  | 0.455 |
| walker |  | 8683 | 16 | Code::CodeKey { rung: Doc, file: beetsplug/bpd/__init__.py, decl: 88, sub: 0, line: 697 } |  |  | 0.455 |
| walker |  | 8699 | 16 | Code::CodeKey { rung: Doc, file: beetsplug/bpd/__init__.py, decl: 91, sub: 0, line: 731 } |  |  | 0.455 |
| walker |  | 8715 | 16 | Code::CodeKey { rung: Doc, file: beetsplug/bpd/__init__.py, decl: 100, sub: 0, line: 794 } |  |  | 0.455 |
| walker |  | 8731 | 16 | Code::CodeKey { rung: Doc, file: beetsplug/bpd/__init__.py, decl: 104, sub: 0, line: 821 } |  |  | 0.455 |
| walker |  | 8747 | 16 | Code::CodeKey { rung: Doc, file: beetsplug/bpd/__init__.py, decl: 111, sub: 0, line: 939 } |  |  | 0.455 |
| walker |  | 8763 | 16 | Code::CodeKey { rung: Doc, file: beetsplug/bpd/__init__.py, decl: 114, sub: 0, line: 960 } |  |  | 0.455 |
| walker |  | 8779 | 16 | Code::CodeKey { rung: Doc, file: beetsplug/bpd/__init__.py, decl: 120, sub: 0, line: 1079 } |  |  | 0.455 |
| walker |  | 8795 | 16 | Code::CodeKey { rung: Doc, file: beetsplug/bpd/__init__.py, decl: 141, sub: 0, line: 1336 } |  |  | 0.455 |
| ns | 8830 |  | 197 | Query-string prefixes: how `beet ls artist:foo` is parsed | 7.1 |  | 0.450 |
| walker |  | 9017 | 222 | Code::CodeKey { rung: Names, file: beets/ui/__init__.py, decl: 0, sub: 1, line: 0 } |  |  | 0.450 |
| walker |  | 9074 | 57 | Code::CodeKey { rung: Decl, file: beets/ui/__init__.py, decl: 13, sub: 0, line: 207 } |  |  | 0.450 |
| walker |  | 9136 | 62 | Code::CodeKey { rung: Decl, file: beets/ui/__init__.py, decl: 19, sub: 0, line: 466 } |  |  | 0.450 |
| ns | 9187 |  | 357 | dbcore/query.py class roster (complete) | 7.2 |  | 0.441 |
| walker |  | 9257 | 121 | Code::CodeKey { rung: Decl, file: beets/ui/__init__.py, decl: 20, sub: 0, line: 498 } |  |  | 0.441 |
| walker |  | 9316 | 59 | Code::CodeKey { rung: Decl, file: beets/ui/__init__.py, decl: 23, sub: 0, line: 534 } |  |  | 0.441 |
| walker |  | 9330 | 14 | Code::CodeKey { rung: Doc, file: beets/ui/__init__.py, decl: 26, sub: 0, line: 621 } |  |  | 0.441 |
| walker |  | 9470 | 140 | Code::CodeKey { rung: Names, file: beets/ui/__init__.py, decl: 0, sub: 2, line: 0 } |  |  | 0.441 |
| walker |  | 9479 | 9 | Code::CodeKey { rung: Decl, file: beets/ui/__init__.py, decl: 40, sub: 0, line: 807 } |  |  | 0.441 |
| ns | 9539 |  | 352 | beets.autotag exports, Recommendation, Proposal | 8.1 |  | 0.434 |
| walker |  | 9577 | 98 | Code::CodeKey { rung: Decl, file: beets/ui/__init__.py, decl: 33, sub: 0, line: 676 } |  |  | 0.434 |
| walker |  | 9700 | 123 | Code::CodeKey { rung: Decl, file: beets/ui/__init__.py, decl: 27, sub: 0, line: 637 } |  |  | 0.434 |
| walker |  | 9715 | 15 | Code::CodeKey { rung: Doc, file: beets/ui/__init__.py, decl: 43, sub: 0, line: 879 } |  |  | 0.434 |
| walker |  | 9731 | 16 | Code::CodeKey { rung: Doc, file: beets/ui/__init__.py, decl: 17, sub: 0, line: 445 } |  |  | 0.434 |
| walker |  | 9747 | 16 | Code::CodeKey { rung: Doc, file: beets/ui/__init__.py, decl: 18, sub: 0, line: 459 } |  |  | 0.434 |
| ns | 9775 |  | 236 | autotag/hooks.py and distance.py symbol roster | 8.2 |  | 0.429 |
| walker |  | 9808 | 61 | Code::CodeKey { rung: Names, file: beets/__init__.py, decl: 0, sub: 0, line: 0 } |  |  | 0.429 |
| walker |  | 9834 | 26 | Code::CodeKey { rung: Decl, file: beets/__init__.py, decl: 2, sub: 0, line: 35 } |  |  | 0.429 |
| walker |  | 9844 | 10 | Code::CodeKey { rung: Doc, file: beets/__init__.py, decl: 1, sub: 0, line: 26 } |  |  | 0.429 |
| walker |  | 9861 | 17 | Code::CodeKey { rung: Doc, file: beets/ui/__init__.py, decl: 36, sub: 0, line: 708 } |  |  | 0.429 |
| walker |  | 9878 | 17 | Code::CodeKey { rung: Doc, file: beets/ui/__init__.py, decl: 41, sub: 0, line: 830 } |  |  | 0.429 |
| walker |  | 9895 | 17 | Code::CodeKey { rung: Doc, file: beetsplug/bpd/__init__.py, decl: 43, sub: 0, line: 269 } |  |  | 0.429 |
| walker |  | 9912 | 17 | Code::CodeKey { rung: Doc, file: beetsplug/bpd/__init__.py, decl: 47, sub: 0, line: 297 } |  |  | 0.429 |
| walker |  | 9929 | 17 | Code::CodeKey { rung: Doc, file: beetsplug/bpd/__init__.py, decl: 65, sub: 0, line: 474 } |  |  | 0.429 |
| walker |  | 9946 | 17 | Code::CodeKey { rung: Doc, file: beetsplug/bpd/__init__.py, decl: 66, sub: 0, line: 482 } |  |  | 0.429 |
| walker |  | 9963 | 17 | Code::CodeKey { rung: Doc, file: beetsplug/bpd/__init__.py, decl: 107, sub: 0, line: 905 } |  |  | 0.429 |
| ns | 9970 |  | 195 | beets/util/__init__.py: classes and the core path/file-operation helpers | 9.1 |  | 0.424 |
| walker |  | 9980 | 17 | Code::CodeKey { rung: Doc, file: beetsplug/bpd/__init__.py, decl: 128, sub: 0, line: 1167 } |  |  | 0.424 |
| walker |  | 9997 | 17 | Code::CodeKey { rung: Doc, file: beetsplug/bpd/__init__.py, decl: 135, sub: 0, line: 1258 } |  |  | 0.424 |
