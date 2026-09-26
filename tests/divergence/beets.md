Score(3000)=0.624 I=0.780 C=0.500 ns_rows≤3K=17/44 grid(1000/1442/2080/3000/4327/6240/9000)=0.596/0.757/0.722/0.624/0.531/0.454/0.389

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
| walker |  | 357 | 28 | Fs::DirListing { dir: beets/dbcore } |  |  | 0.625 |
| walker |  | 387 | 30 | Fs::DirListing { dir: beets/library } |  |  | 0.648 |
| walker |  | 427 | 40 | Code::CodeKey { rung: ModuleDoc, file: beets/__init__.py, decl: 0, sub: 0, line: 0 } |  |  | 0.648 |
| ns | 442 |  | 98 | Core subpackage listings: dbcore, library, autotag, importer | 1.5 |  | 0.661 |
| walker |  | 468 | 41 | Code::CodeKey { rung: ModuleDoc, file: beets/__main__.py, decl: 0, sub: 0, line: 0 } |  |  | 0.661 |
| walker |  | 535 | 67 | Fs::DirListing { dir: beets/ui/commands } |  |  | 0.670 |
| ns | 542 |  | 100 | Project metadata header (poetry, version, license, URLs) | 1.6 |  | 0.650 |
| walker |  | 549 | 14 | Fs::DirListing { dir: beets/ui/commands/import_ } |  |  | 0.650 |
| walker |  | 624 | 75 | Fs::DirListing { dir: beets/util } |  |  | 0.672 |
| ns | 690 |  | 148 | README capability bullets, part 1 (plugin framing + metadata sources) | 1.7 |  | 0.635 |
| walker |  | 701 | 77 | Fs::DirListing { dir: docs } |  |  | 0.639 |
| walker |  | 705 | 4 | Fs::DirListing { dir: docs/extensions } |  |  | 0.639 |
| walker |  | 719 | 14 | Fs::DirListing { dir: docs/_static } |  |  | 0.639 |
| walker |  | 741 | 22 | Fs::DirListing { dir: docs/api } |  |  | 0.639 |
| walker |  | 767 | 26 | Fs::DirListing { dir: docs/guides } |  |  | 0.639 |
| walker |  | 793 | 26 | Fs::DirListing { dir: docs/reference } |  |  | 0.639 |
| walker |  | 821 | 28 | Fs::DirListing { dir: docs/dev } |  |  | 0.639 |
| walker |  | 848 | 27 | Toml::Operational { file: pyproject.toml } |  |  | 0.643 |
| walker |  | 862 | 14 | Code::CodeKey { rung: ModuleDoc, file: beets/util/__init__.py, decl: 0, sub: 0, line: 0 } |  |  | 0.643 |
| walker |  | 880 | 18 | Code::CodeKey { rung: ModuleDoc, file: beets/autotag/__init__.py, decl: 0, sub: 0, line: 0 } |  |  | 0.643 |
| ns | 885 |  | 195 | README capability bullets, part 2 (files, art, web, MPD) | 1.8 |  | 0.596 |
| walker |  | 904 | 24 | Fs::DirListing { dir: docs/_templates/autosummary } |  |  | 0.596 |
| walker |  | 928 | 24 | Fs::DirListing { dir: docs/dev/plugins } |  |  | 0.596 |
| walker |  | 963 | 35 | Code::CodeKey { rung: ModuleDoc, file: beets/dbcore/__init__.py, decl: 0, sub: 0, line: 0 } |  |  | 0.596 |
| walker |  | 1001 | 38 | Code::CodeKey { rung: ModuleDoc, file: beets/importer/__init__.py, decl: 0, sub: 0, line: 0 } |  |  | 0.596 |
| ns | 1051 |  | 166 | UI, util and test-helper subpackage listings | 1.9 |  | 0.606 |
| walker |  | 1206 | 205 | Markdown::Section { file: README.rst, section_index: 0, keeps_default_concavity: false } |  |  | 0.781 |
| walker |  | 1221 | 15 | Fs::DirListing { dir: beets/test } |  |  | 0.808 |
| ns | 1227 |  | 176 | Canonical test / lint / typecheck commands | 1.10 |  | 0.757 |
| walker |  | 1260 | 39 | Fs::DirListing { dir: .github } |  |  | 0.757 |
| walker |  | 1291 | 31 | Fs::DirListing { dir: .github/workflows } |  |  | 0.757 |
| walker |  | 1305 | 14 | Fs::DirListing { dir: .github/ISSUE_TEMPLATE } |  |  | 0.757 |
| walker |  | 1325 | 20 | Code::CodeKey { rung: ModuleDoc, file: beets/ui/commands/import_/__init__.py, decl: 0, sub: 0, line: 0 } |  |  | 0.757 |
| walker |  | 1366 | 41 | Fs::DirListing { dir: docs/dev/plugins/other } |  |  | 0.757 |
| walker |  | 1399 | 33 | Code::CodeKey { rung: ModuleDoc, file: beets/ui/commands/__init__.py, decl: 0, sub: 0, line: 0 } |  |  | 0.757 |
| ns | 1445 |  | 218 | test/ and docs/ top-level listings (complete) | 1.11 |  | 0.671 |
| walker |  | 1463 | 64 | Code::CodeKey { rung: ModuleDoc, file: beets/ui/__init__.py, decl: 0, sub: 0, line: 0 } |  |  | 0.671 |
| ns | 1828 |  | 383 | beetsplug/ bundled plugin listing (complete, 81 entries) | 1.12 |  | 0.550 |
| walker |  | 1846 | 383 | Fs::DirListing { dir: beetsplug } |  |  | 0.749 |
| walker |  | 1857 | 11 | Fs::DirListing { dir: beetsplug/bpd } |  |  | 0.749 |
| walker |  | 1869 | 12 | Fs::DirListing { dir: beetsplug/web } |  |  | 0.749 |
| walker |  | 1883 | 14 | Fs::DirListing { dir: beetsplug/discogs } |  |  | 0.749 |
| walker |  | 1899 | 16 | Fs::DirListing { dir: beetsplug/metasync } |  |  | 0.749 |
| walker |  | 1903 | 4 | Fs::DirListing { dir: beetsplug/web/templates } |  |  | 0.749 |
| walker |  | 1922 | 19 | Fs::DirListing { dir: beetsplug/tidal } |  |  | 0.749 |
| walker |  | 1938 | 16 | Code::CodeKey { rung: ModuleDoc, file: beetsplug/web/__init__.py, decl: 0, sub: 0, line: 0 } |  |  | 0.749 |
| walker |  | 1955 | 17 | Code::CodeKey { rung: ModuleDoc, file: beetsplug/metasync/__init__.py, decl: 0, sub: 0, line: 0 } |  |  | 0.749 |
| walker |  | 1978 | 23 | Fs::DirListing { dir: beetsplug/lastgenre } |  |  | 0.749 |
| ns | 2006 |  | 178 | beets.library public export block (complete) | 2.1 |  | 0.722 |
| walker |  | 2008 | 30 | Fs::DirListing { dir: beetsplug/_utils } |  |  | 0.722 |
| walker |  | 2047 | 39 | Code::CodeKey { rung: ModuleDoc, file: beetsplug/discogs/__init__.py, decl: 0, sub: 0, line: 0 } |  |  | 0.722 |
| walker |  | 2110 | 63 | Code::CodeKey { rung: ModuleDoc, file: beetsplug/bpd/__init__.py, decl: 0, sub: 0, line: 0 } |  |  | 0.722 |
| walker |  | 2132 | 22 | Fs::DirListing { dir: beetsplug/web/static } |  |  | 0.722 |
| ns | 2183 |  | 177 | The `beet` subcommand roster (default_commands) | 2.2 |  | 0.696 |
| walker |  | 2237 | 105 | Code::CodeKey { rung: ModuleDoc, file: beetsplug/lastgenre/__init__.py, decl: 0, sub: 0, line: 0 } |  |  | 0.696 |
| ns | 2373 |  | 190 | Library class: models and schema migrations | 2.3 |  | 0.680 |
| ns | 2528 |  | 155 | Library method roster (complete) | 2.4 | 2.3 | 0.667 |
| walker |  | 2634 | 397 | Code::CodeKey { rung: Names, file: beetsplug/web/__init__.py, decl: 0, sub: 0, line: 0 } |  |  | 0.667 |
| walker |  | 2642 | 8 | Code::CodeKey { rung: Decl, file: beetsplug/web/__init__.py, decl: 33, sub: 0, line: 439 } |  |  | 0.667 |
| walker |  | 2651 | 9 | Code::CodeKey { rung: Decl, file: beetsplug/web/__init__.py, decl: 19, sub: 0, line: 296 } |  |  | 0.667 |
| walker |  | 2661 | 10 | Code::CodeKey { rung: Decl, file: beetsplug/web/__init__.py, decl: 31, sub: 0, line: 412 } |  |  | 0.667 |
| walker |  | 2671 | 10 | Code::CodeKey { rung: Decl, file: beetsplug/web/__init__.py, decl: 32, sub: 0, line: 423 } |  |  | 0.667 |
| walker |  | 2683 | 12 | Code::CodeKey { rung: Decl, file: beetsplug/web/__init__.py, decl: 1, sub: 0, line: 34 } |  |  | 0.667 |
| walker |  | 2697 | 14 | Code::CodeKey { rung: Decl, file: beetsplug/web/__init__.py, decl: 24, sub: 0, line: 344 } |  |  | 0.667 |
| walker |  | 2712 | 15 | Code::CodeKey { rung: Decl, file: beetsplug/web/__init__.py, decl: 25, sub: 0, line: 354 } |  |  | 0.667 |
| walker |  | 2727 | 15 | Code::CodeKey { rung: Decl, file: beetsplug/web/__init__.py, decl: 30, sub: 0, line: 397 } |  |  | 0.667 |
| walker |  | 2743 | 16 | Code::CodeKey { rung: Decl, file: beetsplug/web/__init__.py, decl: 22, sub: 0, line: 317 } |  |  | 0.667 |
| walker |  | 2760 | 17 | Code::CodeKey { rung: Decl, file: beetsplug/web/__init__.py, decl: 29, sub: 0, line: 388 } |  |  | 0.667 |
| walker |  | 2782 | 22 | Code::CodeKey { rung: Decl, file: beetsplug/web/__init__.py, decl: 17, sub: 0, line: 283 } |  |  | 0.667 |
| walker |  | 2804 | 22 | Code::CodeKey { rung: Decl, file: beetsplug/web/__init__.py, decl: 34, sub: 0, line: 447 } |  |  | 0.667 |
| walker |  | 2832 | 28 | Code::CodeKey { rung: Decl, file: beetsplug/web/__init__.py, decl: 11, sub: 0, line: 252 } |  |  | 0.667 |
| walker |  | 2860 | 28 | Code::CodeKey { rung: Decl, file: beetsplug/web/__init__.py, decl: 14, sub: 0, line: 268 } |  |  | 0.667 |
| walker |  | 2891 | 31 | Code::CodeKey { rung: Decl, file: beetsplug/web/__init__.py, decl: 21, sub: 0, line: 310 } |  |  | 0.667 |
| walker |  | 2922 | 31 | Code::CodeKey { rung: Decl, file: beetsplug/web/__init__.py, decl: 26, sub: 0, line: 369 } |  |  | 0.667 |
| ns | 2952 |  | 424 | models.py class headers: LibModel, FormattedItemMapping, Album, Item | 2.5 |  | 0.624 |
| walker |  | 2953 | 31 | Code::CodeKey { rung: Decl, file: beetsplug/web/__init__.py, decl: 27, sub: 0, line: 375 } |  |  | 0.624 |
| walker |  | 2985 | 32 | Code::CodeKey { rung: Decl, file: beetsplug/web/__init__.py, decl: 28, sub: 0, line: 382 } |  |  | 0.624 |
| walker |  | 3018 | 33 | Code::CodeKey { rung: Decl, file: beetsplug/web/__init__.py, decl: 37, sub: 0, line: 520 } |  |  | 0.624 |
| walker |  | 3056 | 38 | Code::CodeKey { rung: Decl, file: beetsplug/web/__init__.py, decl: 20, sub: 0, line: 304 } |  |  | 0.624 |
| walker |  | 3095 | 39 | Code::CodeKey { rung: Decl, file: beetsplug/web/__init__.py, decl: 23, sub: 0, line: 338 } |  |  | 0.624 |
| ns | 3125 |  | 173 | beets.dbcore public export block + package docstring | 2.6 |  | 0.607 |
| ns | 3298 |  | 173 | config_default.yaml: library, directory, plugins, ignore rules | 3.1 |  | 0.592 |
| walker |  | 3307 | 212 | Code::CodeKey { rung: Names, file: beetsplug/bpd/__init__.py, decl: 0, sub: 0, line: 0 } |  |  | 0.592 |
| walker |  | 3670 | 363 | Code::CodeKey { rung: Names, file: beetsplug/bpd/__init__.py, decl: 0, sub: 1, line: 0 } |  |  | 0.592 |
| walker |  | 3686 | 16 | Code::CodeKey { rung: Decl, file: beetsplug/bpd/__init__.py, decl: 36, sub: 0, line: 180 } |  |  | 0.592 |
| ns | 3687 |  | 389 | config_default.yaml: the complete `import:` option block | 3.2 |  | 0.562 |
| walker |  | 3720 | 34 | Code::CodeKey { rung: Decl, file: beetsplug/bpd/__init__.py, decl: 118, sub: 0, line: 1064 } |  |  | 0.562 |
| walker |  | 3773 | 53 | Code::CodeKey { rung: Decl, file: beetsplug/bpd/__init__.py, decl: 168, sub: 0, line: 1583 } |  |  | 0.562 |
| walker |  | 3834 | 61 | Code::CodeKey { rung: Decl, file: beetsplug/bpd/__init__.py, decl: 25, sub: 0, line: 80 } |  |  | 0.562 |
| ns | 3866 |  | 179 | config_default.yaml: tagging defaults, `paths:` templates, threading | 3.4 |  | 0.550 |
| walker |  | 3896 | 62 | Code::CodeKey { rung: Decl, file: beetsplug/bpd/__init__.py, decl: 27, sub: 0, line: 110 } |  |  | 0.550 |
| walker |  | 3975 | 79 | Code::CodeKey { rung: Decl, file: beetsplug/bpd/__init__.py, decl: 94, sub: 0, line: 754 } |  |  | 0.550 |
| walker |  | 4058 | 83 | Code::CodeKey { rung: Decl, file: beetsplug/bpd/__init__.py, decl: 100, sub: 0, line: 794 } |  |  | 0.550 |
| walker |  | 4147 | 89 | Code::CodeKey { rung: Decl, file: beetsplug/bpd/__init__.py, decl: 107, sub: 0, line: 905 } |  |  | 0.550 |
| ns | 4171 |  | 305 | Path-template function roster: DefaultTemplateFunctions (complete) | 3.5 |  | 0.537 |
| walker |  | 4250 | 103 | Code::CodeKey { rung: Decl, file: beetsplug/bpd/__init__.py, decl: 114, sub: 0, line: 960 } |  |  | 0.537 |
| ns | 4291 |  | 120 | config_default.yaml: display formats and default sorts | 3.6 |  | 0.531 |
| walker |  | 4365 | 115 | Code::CodeKey { rung: Decl, file: beetsplug/bpd/__init__.py, decl: 26, sub: 0, line: 90 } |  |  | 0.531 |
| walker |  | 4572 | 207 | Code::CodeKey { rung: Decl, file: beetsplug/bpd/__init__.py, decl: 38, sub: 0, line: 193 } |  |  | 0.531 |
| ns | 4673 |  | 382 | config_default.yaml: autotagger thresholds and distance weights | 3.7 |  | 0.512 |
| walker |  | 4775 | 203 | Code::CodeKey { rung: Decl, file: beetsplug/bpd/__init__.py, decl: 38, sub: 1, line: 193 } |  |  | 0.512 |
| ns | 4813 |  | 140 | config_default.yaml: terminal UI, colour names, import layout | 3.8 |  | 0.503 |
| walker |  | 4973 | 198 | Code::CodeKey { rung: Decl, file: beetsplug/bpd/__init__.py, decl: 38, sub: 2, line: 193 } |  |  | 0.503 |
| ns | 4986 |  | 173 | Album method roster (complete, names only) | 4.1 | 2.5 | 0.495 |
| walker |  | 5266 | 293 | Code::CodeKey { rung: Decl, file: beetsplug/bpd/__init__.py, decl: 38, sub: 3, line: 193 } |  |  | 0.495 |
| walker |  | 5275 | 9 | Code::CodeKey { rung: Doc, file: beetsplug/bpd/__init__.py, decl: 52, sub: 0, line: 346 } |  |  | 0.495 |
| walker |  | 5284 | 9 | Code::CodeKey { rung: Doc, file: beetsplug/bpd/__init__.py, decl: 90, sub: 0, line: 725 } |  |  | 0.495 |
| ns | 5290 |  | 304 | Item method roster (complete, names only) | 4.3 | 2.5 | 0.482 |
| walker |  | 5294 | 10 | Code::CodeKey { rung: Doc, file: beetsplug/bpd/__init__.py, decl: 56, sub: 0, line: 365 } |  |  | 0.482 |
| walker |  | 5304 | 10 | Code::CodeKey { rung: Doc, file: beetsplug/bpd/__init__.py, decl: 72, sub: 0, line: 526 } |  |  | 0.482 |
| ns | 5458 |  | 168 | Item search fields and MediaFile-backed field sets | 4.4 | 2.5 | 0.476 |
| walker |  | 5504 | 200 | Code::CodeKey { rung: Decl, file: beetsplug/bpd/__init__.py, decl: 121, sub: 0, line: 1099 } |  |  | 0.476 |
| ns | 5629 |  | 171 | LibModel shared-method roster (complete after `_fields`) | 4.5 | 2.5 | 0.469 |
| ns | 5787 |  | 158 | beets.importer public exports + package docstring | 5.1 |  | 0.463 |
| walker |  | 5993 | 489 | Code::CodeKey { rung: Decl, file: beetsplug/bpd/__init__.py, decl: 121, sub: 1, line: 1099 } |  |  | 0.463 |
| ns | 6147 |  | 360 | importer/stages.py: complete stage-function roster with section banners | 5.2 |  | 0.454 |
| walker |  | 6181 | 188 | Code::CodeKey { rung: Decl, file: beetsplug/bpd/__init__.py, decl: 121, sub: 2, line: 1099 } |  |  | 0.454 |
| walker |  | 6477 | 296 | Code::CodeKey { rung: Decl, file: beetsplug/bpd/__init__.py, decl: 121, sub: 3, line: 1099 } |  |  | 0.454 |
| walker |  | 6488 | 11 | Code::CodeKey { rung: Doc, file: beetsplug/bpd/__init__.py, decl: 55, sub: 0, line: 361 } |  |  | 0.454 |
| walker |  | 6499 | 11 | Code::CodeKey { rung: Doc, file: beetsplug/bpd/__init__.py, decl: 58, sub: 0, line: 386 } |  |  | 0.454 |
| walker |  | 6510 | 11 | Code::CodeKey { rung: Doc, file: beetsplug/bpd/__init__.py, decl: 112, sub: 0, line: 943 } |  |  | 0.454 |
| walker |  | 6522 | 12 | Code::CodeKey { rung: Doc, file: beetsplug/bpd/__init__.py, decl: 40, sub: 0, line: 241 } |  |  | 0.454 |
| walker |  | 6534 | 12 | Code::CodeKey { rung: Doc, file: beetsplug/bpd/__init__.py, decl: 54, sub: 0, line: 357 } |  |  | 0.454 |
| walker |  | 6546 | 12 | Code::CodeKey { rung: Doc, file: beetsplug/bpd/__init__.py, decl: 62, sub: 0, line: 458 } |  |  | 0.454 |
| walker |  | 6558 | 12 | Code::CodeKey { rung: Doc, file: beetsplug/bpd/__init__.py, decl: 63, sub: 0, line: 463 } |  |  | 0.454 |
| walker |  | 6570 | 12 | Code::CodeKey { rung: Doc, file: beetsplug/bpd/__init__.py, decl: 64, sub: 0, line: 468 } |  |  | 0.454 |
| ns | 6572 |  | 425 | ImportSession.run(): how the pipeline is assembled | 5.3 | 5.2 | 0.441 |
| walker |  | 6582 | 12 | Code::CodeKey { rung: Doc, file: beetsplug/bpd/__init__.py, decl: 70, sub: 0, line: 514 } |  |  | 0.441 |
| walker |  | 6594 | 12 | Code::CodeKey { rung: Doc, file: beetsplug/bpd/__init__.py, decl: 71, sub: 0, line: 522 } |  |  | 0.441 |
| walker |  | 6606 | 12 | Code::CodeKey { rung: Doc, file: beetsplug/bpd/__init__.py, decl: 87, sub: 0, line: 689 } |  |  | 0.441 |
| walker |  | 6618 | 12 | Code::CodeKey { rung: Doc, file: beetsplug/bpd/__init__.py, decl: 160, sub: 0, line: 1518 } |  |  | 0.441 |
| walker |  | 6631 | 13 | Code::CodeKey { rung: Doc, file: beetsplug/bpd/__init__.py, decl: 45, sub: 0, line: 286 } |  |  | 0.441 |
| walker |  | 6644 | 13 | Code::CodeKey { rung: Doc, file: beetsplug/bpd/__init__.py, decl: 75, sub: 0, line: 552 } |  |  | 0.441 |
| walker |  | 6657 | 13 | Code::CodeKey { rung: Doc, file: beetsplug/bpd/__init__.py, decl: 86, sub: 0, line: 676 } |  |  | 0.441 |
| walker |  | 6670 | 13 | Code::CodeKey { rung: Doc, file: beetsplug/bpd/__init__.py, decl: 94, sub: 0, line: 754 } |  |  | 0.441 |
| walker |  | 6683 | 13 | Code::CodeKey { rung: Doc, file: beetsplug/bpd/__init__.py, decl: 170, sub: 0, line: 1601 } |  |  | 0.441 |
| walker |  | 6697 | 14 | Code::CodeKey { rung: Doc, file: beetsplug/bpd/__init__.py, decl: 57, sub: 0, line: 373 } |  |  | 0.441 |
| walker |  | 6711 | 14 | Code::CodeKey { rung: Doc, file: beetsplug/bpd/__init__.py, decl: 77, sub: 0, line: 577 } |  |  | 0.441 |
| walker |  | 6725 | 14 | Code::CodeKey { rung: Doc, file: beetsplug/bpd/__init__.py, decl: 96, sub: 0, line: 763 } |  |  | 0.441 |
| walker |  | 6739 | 14 | Code::CodeKey { rung: Doc, file: beetsplug/bpd/__init__.py, decl: 103, sub: 0, line: 816 } |  |  | 0.441 |
| walker |  | 6753 | 14 | Code::CodeKey { rung: Doc, file: beetsplug/bpd/__init__.py, decl: 105, sub: 0, line: 825 } |  |  | 0.441 |
| walker |  | 6767 | 14 | Code::CodeKey { rung: Doc, file: beetsplug/bpd/__init__.py, decl: 113, sub: 0, line: 950 } |  |  | 0.441 |
| walker |  | 6781 | 14 | Code::CodeKey { rung: Doc, file: beetsplug/bpd/__init__.py, decl: 145, sub: 0, line: 1416 } |  |  | 0.441 |
| walker |  | 6795 | 14 | Code::CodeKey { rung: Doc, file: beetsplug/bpd/__init__.py, decl: 146, sub: 0, line: 1424 } |  |  | 0.441 |
| walker |  | 6810 | 15 | Code::CodeKey { rung: Doc, file: beetsplug/bpd/__init__.py, decl: 41, sub: 0, line: 245 } |  |  | 0.441 |
| walker |  | 6825 | 15 | Code::CodeKey { rung: Doc, file: beetsplug/bpd/__init__.py, decl: 61, sub: 0, line: 453 } |  |  | 0.441 |
| ns | 6829 |  | 257 | ImportSession class contract (attributes + the four decision hooks) | 5.4 | 5.3 | 0.432 |
| walker |  | 6840 | 15 | Code::CodeKey { rung: Doc, file: beetsplug/bpd/__init__.py, decl: 69, sub: 0, line: 505 } |  |  | 0.432 |
| walker |  | 6855 | 15 | Code::CodeKey { rung: Doc, file: beetsplug/bpd/__init__.py, decl: 73, sub: 0, line: 533 } |  |  | 0.432 |
| walker |  | 6870 | 15 | Code::CodeKey { rung: Doc, file: beetsplug/bpd/__init__.py, decl: 84, sub: 0, line: 648 } |  |  | 0.432 |
| walker |  | 6885 | 15 | Code::CodeKey { rung: Doc, file: beetsplug/bpd/__init__.py, decl: 85, sub: 0, line: 654 } |  |  | 0.432 |
| walker |  | 6900 | 15 | Code::CodeKey { rung: Doc, file: beetsplug/bpd/__init__.py, decl: 130, sub: 0, line: 1204 } |  |  | 0.432 |
| walker |  | 6915 | 15 | Code::CodeKey { rung: Doc, file: beetsplug/bpd/__init__.py, decl: 140, sub: 0, line: 1314 } |  |  | 0.432 |
| walker |  | 6930 | 15 | Code::CodeKey { rung: Doc, file: beetsplug/web/__init__.py, decl: 6, sub: 0, line: 117 } |  |  | 0.432 |
| walker |  | 6944 | 14 | Code::CodeKey { rung: Names, file: beetsplug/_utils/__init__.py, decl: 0, sub: 0, line: 0 } |  |  | 0.432 |
| walker |  | 7046 | 102 | Code::CodeKey { rung: Names, file: beetsplug/metasync/__init__.py, decl: 0, sub: 0, line: 0 } |  |  | 0.432 |
| walker |  | 7075 | 29 | Code::CodeKey { rung: Decl, file: beetsplug/metasync/__init__.py, decl: 2, sub: 0, line: 34 } |  |  | 0.432 |
| ns | 7103 |  | 274 | importer/tasks.py: Action enum and the complete task class roster | 5.5 |  | 0.425 |
| walker |  | 7131 | 56 | Code::CodeKey { rung: Decl, file: beetsplug/metasync/__init__.py, decl: 9, sub: 0, line: 76 } |  |  | 0.425 |
| walker |  | 7192 | 61 | Code::CodeKey { rung: Decl, file: beetsplug/metasync/__init__.py, decl: 3, sub: 0, line: 40 } |  |  | 0.425 |
| walker |  | 7197 | 5 | Code::CodeKey { rung: Body, file: beetsplug/metasync/__init__.py, decl: 5, sub: 0, line: 47 } |  |  | 0.425 |
| walker |  | 7204 | 7 | Code::CodeKey { rung: Body, file: beetsplug/metasync/__init__.py, decl: 10, sub: 0, line: 79 } |  |  | 0.425 |
| walker |  | 7218 | 14 | Code::CodeKey { rung: Doc, file: beetsplug/metasync/__init__.py, decl: 12, sub: 0, line: 104 } |  |  | 0.425 |
| walker |  | 7234 | 16 | Code::CodeKey { rung: Doc, file: beetsplug/bpd/__init__.py, decl: 67, sub: 0, line: 487 } |  |  | 0.425 |
| walker |  | 7250 | 16 | Code::CodeKey { rung: Doc, file: beetsplug/bpd/__init__.py, decl: 79, sub: 0, line: 604 } |  |  | 0.425 |
| walker |  | 7266 | 16 | Code::CodeKey { rung: Doc, file: beetsplug/bpd/__init__.py, decl: 88, sub: 0, line: 697 } |  |  | 0.425 |
| walker |  | 7282 | 16 | Code::CodeKey { rung: Doc, file: beetsplug/bpd/__init__.py, decl: 91, sub: 0, line: 731 } |  |  | 0.425 |
| walker |  | 7298 | 16 | Code::CodeKey { rung: Doc, file: beetsplug/bpd/__init__.py, decl: 100, sub: 0, line: 794 } |  |  | 0.425 |
| walker |  | 7314 | 16 | Code::CodeKey { rung: Doc, file: beetsplug/bpd/__init__.py, decl: 104, sub: 0, line: 821 } |  |  | 0.425 |
| walker |  | 7330 | 16 | Code::CodeKey { rung: Doc, file: beetsplug/bpd/__init__.py, decl: 111, sub: 0, line: 939 } |  |  | 0.425 |
| walker |  | 7346 | 16 | Code::CodeKey { rung: Doc, file: beetsplug/bpd/__init__.py, decl: 114, sub: 0, line: 960 } |  |  | 0.425 |
| ns | 7349 |  | 246 | BeetsPlugin base class: docstring and declared attributes | 6.1 |  | 0.419 |
| walker |  | 7362 | 16 | Code::CodeKey { rung: Doc, file: beetsplug/bpd/__init__.py, decl: 120, sub: 0, line: 1079 } |  |  | 0.419 |
| walker |  | 7378 | 16 | Code::CodeKey { rung: Doc, file: beetsplug/bpd/__init__.py, decl: 141, sub: 0, line: 1336 } |  |  | 0.419 |
| walker |  | 7439 | 61 | Code::CodeKey { rung: Names, file: beets/__init__.py, decl: 0, sub: 0, line: 0 } |  |  | 0.419 |
| walker |  | 7465 | 26 | Code::CodeKey { rung: Decl, file: beets/__init__.py, decl: 2, sub: 0, line: 35 } |  |  | 0.419 |
| walker |  | 7475 | 10 | Code::CodeKey { rung: Doc, file: beets/__init__.py, decl: 1, sub: 0, line: 26 } |  |  | 0.419 |
| walker |  | 7492 | 17 | Code::CodeKey { rung: Doc, file: beetsplug/bpd/__init__.py, decl: 43, sub: 0, line: 269 } |  |  | 0.419 |
| walker |  | 7509 | 17 | Code::CodeKey { rung: Doc, file: beetsplug/bpd/__init__.py, decl: 47, sub: 0, line: 297 } |  |  | 0.419 |
| walker |  | 7526 | 17 | Code::CodeKey { rung: Doc, file: beetsplug/bpd/__init__.py, decl: 65, sub: 0, line: 474 } |  |  | 0.419 |
| walker |  | 7543 | 17 | Code::CodeKey { rung: Doc, file: beetsplug/bpd/__init__.py, decl: 66, sub: 0, line: 482 } |  |  | 0.419 |
| walker |  | 7560 | 17 | Code::CodeKey { rung: Doc, file: beetsplug/bpd/__init__.py, decl: 107, sub: 0, line: 905 } |  |  | 0.419 |
| walker |  | 7577 | 17 | Code::CodeKey { rung: Doc, file: beetsplug/bpd/__init__.py, decl: 128, sub: 0, line: 1167 } |  |  | 0.419 |
| walker |  | 7594 | 17 | Code::CodeKey { rung: Doc, file: beetsplug/bpd/__init__.py, decl: 135, sub: 0, line: 1258 } |  |  | 0.419 |
| ns | 7595 |  | 246 | BeetsPlugin method roster (complete) | 6.2 | 6.1 | 0.415 |
| walker |  | 7611 | 17 | Code::CodeKey { rung: Doc, file: beetsplug/web/__init__.py, decl: 4, sub: 0, line: 103 } |  |  | 0.415 |
| walker |  | 7628 | 17 | Code::CodeKey { rung: Doc, file: beetsplug/web/__init__.py, decl: 11, sub: 0, line: 252 } |  |  | 0.415 |
| walker |  | 7756 | 128 | Code::CodeKey { rung: Names, file: beetsplug/discogs/__init__.py, decl: 0, sub: 0, line: 0 } |  |  | 0.415 |
| walker |  | 7812 | 56 | Code::CodeKey { rung: Decl, file: beetsplug/discogs/__init__.py, decl: 4, sub: 0, line: 60 } |  |  | 0.415 |
| walker |  | 7883 | 71 | Code::CodeKey { rung: Decl, file: beetsplug/discogs/__init__.py, decl: 6, sub: 0, line: 83 } |  |  | 0.415 |
| ns | 8017 |  | 422 | The complete `EventType` literal — every plugin event name | 6.3 | 6.1 | 0.403 |
| walker |  | 8073 | 190 | Code::CodeKey { rung: Decl, file: beetsplug/discogs/__init__.py, decl: 5, sub: 0, line: 68 } |  |  | 0.403 |
| walker |  | 8266 | 193 | Code::CodeKey { rung: Decl, file: beetsplug/discogs/__init__.py, decl: 7, sub: 0, line: 93 } |  |  | 0.403 |
| walker |  | 8283 | 17 | Code::CodeKey { rung: Decl, file: beetsplug/discogs/__init__.py, decl: 15, sub: 0, line: 217 } |  |  | 0.403 |
| walker |  | 8303 | 20 | Code::CodeKey { rung: Decl, file: beetsplug/discogs/__init__.py, decl: 14, sub: 0, line: 203 } |  |  | 0.403 |
| ns | 8321 |  | 304 | plugins.py module-level API roster (complete) | 6.4 |  | 0.398 |
| walker |  | 8498 | 195 | Code::CodeKey { rung: Decl, file: beetsplug/discogs/__init__.py, decl: 7, sub: 1, line: 93 } |  |  | 0.398 |
| walker |  | 8511 | 13 | Code::CodeKey { rung: Decl, file: beetsplug/discogs/__init__.py, decl: 21, sub: 0, line: 341 } |  |  | 0.398 |
| walker |  | 8568 | 57 | Code::CodeKey { rung: Decl, file: beetsplug/discogs/__init__.py, decl: 18, sub: 0, line: 264 } |  |  | 0.398 |
| ns | 8633 |  | 312 | MetadataSourcePlugin: the metadata-backend contract | 6.5 |  | 0.393 |
| walker |  | 8765 | 197 | Code::CodeKey { rung: Decl, file: beetsplug/discogs/__init__.py, decl: 7, sub: 2, line: 93 } |  |  | 0.393 |
| walker |  | 8772 | 7 | Code::CodeKey { rung: Decl, file: beetsplug/discogs/__init__.py, decl: 29, sub: 0, line: 705 } |  |  | 0.393 |
| walker |  | 8801 | 29 | Code::CodeKey { rung: Decl, file: beetsplug/discogs/__init__.py, decl: 24, sub: 0, line: 488 } |  |  | 0.393 |
| walker |  | 8830 | 29 | Code::CodeKey { rung: Decl, file: beetsplug/discogs/__init__.py, decl: 26, sub: 0, line: 615 } |  |  | 0.389 |
| ns | 8830 |  | 197 | Query-string prefixes: how `beet ls artist:foo` is parsed | 7.1 |  | 0.389 |
| walker |  | 8875 | 45 | Code::CodeKey { rung: Decl, file: beetsplug/discogs/__init__.py, decl: 28, sub: 0, line: 666 } |  |  | 0.389 |
| walker |  | 8890 | 15 | Code::CodeKey { rung: Doc, file: beetsplug/discogs/__init__.py, decl: 11, sub: 0, line: 165 } |  |  | 0.389 |
| walker |  | 8905 | 15 | Code::CodeKey { rung: Doc, file: beetsplug/discogs/__init__.py, decl: 14, sub: 0, line: 203 } |  |  | 0.389 |
| walker |  | 8922 | 17 | Code::CodeKey { rung: Doc, file: beetsplug/discogs/__init__.py, decl: 28, sub: 0, line: 666 } |  |  | 0.389 |
| walker |  | 8940 | 18 | Code::CodeKey { rung: Doc, file: beetsplug/bpd/__init__.py, decl: 95, sub: 0, line: 757 } |  |  | 0.389 |
| walker |  | 8958 | 18 | Code::CodeKey { rung: Doc, file: beetsplug/bpd/__init__.py, decl: 101, sub: 0, line: 797 } |  |  | 0.389 |
| walker |  | 8976 | 18 | Code::CodeKey { rung: Doc, file: beetsplug/bpd/__init__.py, decl: 108, sub: 0, line: 908 } |  |  | 0.389 |
| walker |  | 8994 | 18 | Code::CodeKey { rung: Doc, file: beetsplug/bpd/__init__.py, decl: 124, sub: 0, line: 1117 } |  |  | 0.389 |
| walker |  | 9012 | 18 | Code::CodeKey { rung: Doc, file: beetsplug/bpd/__init__.py, decl: 131, sub: 0, line: 1209 } |  |  | 0.389 |
| walker |  | 9030 | 18 | Code::CodeKey { rung: Doc, file: beetsplug/bpd/__init__.py, decl: 133, sub: 0, line: 1248 } |  |  | 0.389 |
| walker |  | 9048 | 18 | Code::CodeKey { rung: Doc, file: beetsplug/bpd/__init__.py, decl: 166, sub: 0, line: 1565 } |  |  | 0.389 |
| walker |  | 9066 | 18 | Code::CodeKey { rung: Doc, file: beetsplug/discogs/__init__.py, decl: 19, sub: 0, line: 308 } |  |  | 0.389 |
| walker |  | 9084 | 18 | Code::CodeKey { rung: Doc, file: beetsplug/discogs/__init__.py, decl: 22, sub: 0, line: 353 } |  |  | 0.389 |
| walker |  | 9102 | 18 | Code::CodeKey { rung: Doc, file: beetsplug/discogs/__init__.py, decl: 30, sub: 0, line: 722 } |  |  | 0.389 |
| walker |  | 9120 | 18 | Code::CodeKey { rung: Doc, file: beetsplug/web/__init__.py, decl: 10, sub: 0, line: 241 } |  |  | 0.389 |
| walker |  | 9138 | 18 | Code::CodeKey { rung: Doc, file: beetsplug/web/__init__.py, decl: 14, sub: 0, line: 268 } |  |  | 0.389 |
| ns | 9187 |  | 357 | dbcore/query.py class roster (complete) | 7.2 |  | 0.381 |
| walker |  | 9208 | 70 | Code::CodeKey { rung: Names, file: beetsplug/tidal/__init__.py, decl: 0, sub: 0, line: 0 } |  |  | 0.381 |
| walker |  | 9287 | 79 | Code::CodeKey { rung: Decl, file: beetsplug/tidal/__init__.py, decl: 28, sub: 0, line: 494 } |  |  | 0.381 |
| walker |  | 9473 | 186 | Code::CodeKey { rung: Decl, file: beetsplug/tidal/__init__.py, decl: 2, sub: 0, line: 37 } |  |  | 0.381 |
| ns | 9539 |  | 352 | beets.autotag exports, Recommendation, Proposal | 8.1 |  | 0.374 |
| walker |  | 9686 | 213 | Code::CodeKey { rung: Decl, file: beetsplug/tidal/__init__.py, decl: 2, sub: 1, line: 37 } |  |  | 0.374 |
| walker |  | 9703 | 17 | Code::CodeKey { rung: Decl, file: beetsplug/tidal/__init__.py, decl: 13, sub: 0, line: 142 } |  |  | 0.374 |
| walker |  | 9729 | 26 | Code::CodeKey { rung: Decl, file: beetsplug/tidal/__init__.py, decl: 12, sub: 0, line: 119 } |  |  | 0.374 |
| ns | 9775 |  | 236 | autotag/hooks.py and distance.py symbol roster | 8.2 |  | 0.371 |
| walker |  | 9781 | 52 | Code::CodeKey { rung: Decl, file: beetsplug/tidal/__init__.py, decl: 19, sub: 0, line: 295 } |  |  | 0.371 |
| walker |  | 9834 | 53 | Code::CodeKey { rung: Decl, file: beetsplug/tidal/__init__.py, decl: 18, sub: 0, line: 236 } |  |  | 0.371 |
| walker |  | 9845 | 11 | Code::CodeKey { rung: Doc, file: beetsplug/tidal/__init__.py, decl: 14, sub: 0, line: 161 } |  |  | 0.371 |
| walker |  | 9856 | 11 | Code::CodeKey { rung: Doc, file: beetsplug/tidal/__init__.py, decl: 15, sub: 0, line: 169 } |  |  | 0.371 |
| walker |  | 9970 | 114 | Code::CodeKey { rung: Decl, file: beetsplug/tidal/__init__.py, decl: 2, sub: 2, line: 37 } |  |  | 0.366 |
| ns | 9970 |  | 195 | beets/util/__init__.py: classes and the core path/file-operation helpers | 9.1 |  | 0.366 |
