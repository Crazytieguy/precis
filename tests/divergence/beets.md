Score(3000)=0.623 I=0.778 C=0.498 ns_rows≤3K=17/44 grid(1000/1442/2080/3000/4327/6240/9000)=0.647/0.754/0.721/0.623/0.530/0.453/0.388

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
| walker |  | 835 | 14 | Code::CodeKey { rung: ModuleDoc, file: beets/util/__init__.py, decl: 0, sub: 0, line: 0 } |  |  | 0.639 |
| walker |  | 853 | 18 | Code::CodeKey { rung: ModuleDoc, file: beets/autotag/__init__.py, decl: 0, sub: 0, line: 0 } |  |  | 0.639 |
| walker |  | 877 | 24 | Fs::DirListing { dir: docs/_templates/autosummary } |  |  | 0.639 |
| ns | 885 |  | 195 | README capability bullets, part 2 (files, art, web, MPD) | 1.8 |  | 0.592 |
| walker |  | 901 | 24 | Fs::DirListing { dir: docs/dev/plugins } |  |  | 0.592 |
| walker |  | 936 | 35 | Code::CodeKey { rung: ModuleDoc, file: beets/dbcore/__init__.py, decl: 0, sub: 0, line: 0 } |  |  | 0.593 |
| walker |  | 974 | 38 | Code::CodeKey { rung: ModuleDoc, file: beets/importer/__init__.py, decl: 0, sub: 0, line: 0 } |  |  | 0.593 |
| ns | 1051 |  | 166 | UI, util and test-helper subpackage listings | 1.9 |  | 0.604 |
| walker |  | 1179 | 205 | Markdown::Section { file: README.rst, section_index: 0, keeps_default_concavity: false } |  |  | 0.778 |
| walker |  | 1194 | 15 | Fs::DirListing { dir: beets/test } |  |  | 0.805 |
| ns | 1227 |  | 176 | Canonical test / lint / typecheck commands | 1.10 |  | 0.754 |
| walker |  | 1233 | 39 | Fs::DirListing { dir: .github } |  |  | 0.754 |
| walker |  | 1264 | 31 | Fs::DirListing { dir: .github/workflows } |  |  | 0.754 |
| walker |  | 1278 | 14 | Fs::DirListing { dir: .github/ISSUE_TEMPLATE } |  |  | 0.754 |
| walker |  | 1298 | 20 | Code::CodeKey { rung: ModuleDoc, file: beets/ui/commands/import_/__init__.py, decl: 0, sub: 0, line: 0 } |  |  | 0.754 |
| walker |  | 1339 | 41 | Fs::DirListing { dir: docs/dev/plugins/other } |  |  | 0.754 |
| walker |  | 1372 | 33 | Code::CodeKey { rung: ModuleDoc, file: beets/ui/commands/__init__.py, decl: 0, sub: 0, line: 0 } |  |  | 0.754 |
| walker |  | 1436 | 64 | Code::CodeKey { rung: ModuleDoc, file: beets/ui/__init__.py, decl: 0, sub: 0, line: 0 } |  |  | 0.754 |
| ns | 1445 |  | 218 | test/ and docs/ top-level listings (complete) | 1.11 |  | 0.669 |
| walker |  | 1819 | 383 | Fs::DirListing { dir: beetsplug } |  |  | 0.692 |
| ns | 1828 |  | 383 | beetsplug/ bundled plugin listing (complete, 81 entries) | 1.12 |  | 0.747 |
| walker |  | 1830 | 11 | Fs::DirListing { dir: beetsplug/bpd } |  |  | 0.747 |
| walker |  | 1842 | 12 | Fs::DirListing { dir: beetsplug/web } |  |  | 0.747 |
| walker |  | 1856 | 14 | Fs::DirListing { dir: beetsplug/discogs } |  |  | 0.747 |
| walker |  | 1872 | 16 | Fs::DirListing { dir: beetsplug/metasync } |  |  | 0.747 |
| walker |  | 1876 | 4 | Fs::DirListing { dir: beetsplug/web/templates } |  |  | 0.747 |
| walker |  | 1895 | 19 | Fs::DirListing { dir: beetsplug/tidal } |  |  | 0.747 |
| walker |  | 1911 | 16 | Code::CodeKey { rung: ModuleDoc, file: beetsplug/web/__init__.py, decl: 0, sub: 0, line: 0 } |  |  | 0.747 |
| walker |  | 1928 | 17 | Code::CodeKey { rung: ModuleDoc, file: beetsplug/metasync/__init__.py, decl: 0, sub: 0, line: 0 } |  |  | 0.747 |
| walker |  | 1951 | 23 | Fs::DirListing { dir: beetsplug/lastgenre } |  |  | 0.747 |
| walker |  | 1981 | 30 | Fs::DirListing { dir: beetsplug/_utils } |  |  | 0.747 |
| ns | 2006 |  | 178 | beets.library public export block (complete) | 2.1 |  | 0.721 |
| walker |  | 2020 | 39 | Code::CodeKey { rung: ModuleDoc, file: beetsplug/discogs/__init__.py, decl: 0, sub: 0, line: 0 } |  |  | 0.721 |
| walker |  | 2083 | 63 | Code::CodeKey { rung: ModuleDoc, file: beetsplug/bpd/__init__.py, decl: 0, sub: 0, line: 0 } |  |  | 0.721 |
| walker |  | 2105 | 22 | Fs::DirListing { dir: beetsplug/web/static } |  |  | 0.721 |
| ns | 2183 |  | 177 | The `beet` subcommand roster (default_commands) | 2.2 |  | 0.694 |
| walker |  | 2210 | 105 | Code::CodeKey { rung: ModuleDoc, file: beetsplug/lastgenre/__init__.py, decl: 0, sub: 0, line: 0 } |  |  | 0.694 |
| ns | 2373 |  | 190 | Library class: models and schema migrations | 2.3 |  | 0.678 |
| walker |  | 2422 | 212 | Code::CodeKey { rung: Names, file: beetsplug/bpd/__init__.py, decl: 0, sub: 0, line: 0 } |  |  | 0.678 |
| ns | 2528 |  | 155 | Library method roster (complete) | 2.4 | 2.3 | 0.666 |
| walker |  | 2785 | 363 | Code::CodeKey { rung: Names, file: beetsplug/bpd/__init__.py, decl: 0, sub: 1, line: 0 } |  |  | 0.666 |
| walker |  | 2801 | 16 | Code::CodeKey { rung: Decl, file: beetsplug/bpd/__init__.py, decl: 36, sub: 0, line: 180 } |  |  | 0.666 |
| walker |  | 2835 | 34 | Code::CodeKey { rung: Decl, file: beetsplug/bpd/__init__.py, decl: 118, sub: 0, line: 1064 } |  |  | 0.666 |
| walker |  | 2888 | 53 | Code::CodeKey { rung: Decl, file: beetsplug/bpd/__init__.py, decl: 168, sub: 0, line: 1583 } |  |  | 0.666 |
| walker |  | 2949 | 61 | Code::CodeKey { rung: Decl, file: beetsplug/bpd/__init__.py, decl: 25, sub: 0, line: 80 } |  |  | 0.666 |
| ns | 2952 |  | 424 | models.py class headers: LibModel, FormattedItemMapping, Album, Item | 2.5 |  | 0.623 |
| walker |  | 3011 | 62 | Code::CodeKey { rung: Decl, file: beetsplug/bpd/__init__.py, decl: 27, sub: 0, line: 110 } |  |  | 0.623 |
| walker |  | 3090 | 79 | Code::CodeKey { rung: Decl, file: beetsplug/bpd/__init__.py, decl: 94, sub: 0, line: 754 } |  |  | 0.623 |
| ns | 3125 |  | 173 | beets.dbcore public export block + package docstring | 2.6 |  | 0.606 |
| walker |  | 3173 | 83 | Code::CodeKey { rung: Decl, file: beetsplug/bpd/__init__.py, decl: 100, sub: 0, line: 794 } |  |  | 0.606 |
| walker |  | 3262 | 89 | Code::CodeKey { rung: Decl, file: beetsplug/bpd/__init__.py, decl: 107, sub: 0, line: 905 } |  |  | 0.606 |
| ns | 3298 |  | 173 | config_default.yaml: library, directory, plugins, ignore rules | 3.1 |  | 0.590 |
| walker |  | 3365 | 103 | Code::CodeKey { rung: Decl, file: beetsplug/bpd/__init__.py, decl: 114, sub: 0, line: 960 } |  |  | 0.590 |
| walker |  | 3480 | 115 | Code::CodeKey { rung: Decl, file: beetsplug/bpd/__init__.py, decl: 26, sub: 0, line: 90 } |  |  | 0.590 |
| walker |  | 3687 | 207 | Code::CodeKey { rung: Decl, file: beetsplug/bpd/__init__.py, decl: 38, sub: 0, line: 193 } |  |  | 0.560 |
| ns | 3687 |  | 389 | config_default.yaml: the complete `import:` option block | 3.2 |  | 0.560 |
| ns | 3866 |  | 179 | config_default.yaml: tagging defaults, `paths:` templates, threading | 3.4 |  | 0.548 |
| walker |  | 3890 | 203 | Code::CodeKey { rung: Decl, file: beetsplug/bpd/__init__.py, decl: 38, sub: 1, line: 193 } |  |  | 0.548 |
| walker |  | 4088 | 198 | Code::CodeKey { rung: Decl, file: beetsplug/bpd/__init__.py, decl: 38, sub: 2, line: 193 } |  |  | 0.548 |
| ns | 4171 |  | 305 | Path-template function roster: DefaultTemplateFunctions (complete) | 3.5 |  | 0.536 |
| ns | 4291 |  | 120 | config_default.yaml: display formats and default sorts | 3.6 |  | 0.530 |
| walker |  | 4381 | 293 | Code::CodeKey { rung: Decl, file: beetsplug/bpd/__init__.py, decl: 38, sub: 3, line: 193 } |  |  | 0.530 |
| walker |  | 4390 | 9 | Code::CodeKey { rung: Doc, file: beetsplug/bpd/__init__.py, decl: 52, sub: 0, line: 346 } |  |  | 0.530 |
| walker |  | 4399 | 9 | Code::CodeKey { rung: Doc, file: beetsplug/bpd/__init__.py, decl: 90, sub: 0, line: 725 } |  |  | 0.530 |
| walker |  | 4409 | 10 | Code::CodeKey { rung: Doc, file: beetsplug/bpd/__init__.py, decl: 56, sub: 0, line: 365 } |  |  | 0.530 |
| walker |  | 4419 | 10 | Code::CodeKey { rung: Doc, file: beetsplug/bpd/__init__.py, decl: 72, sub: 0, line: 526 } |  |  | 0.530 |
| walker |  | 4619 | 200 | Code::CodeKey { rung: Decl, file: beetsplug/bpd/__init__.py, decl: 121, sub: 0, line: 1099 } |  |  | 0.530 |
| ns | 4673 |  | 382 | config_default.yaml: autotagger thresholds and distance weights | 3.7 |  | 0.510 |
| ns | 4813 |  | 140 | config_default.yaml: terminal UI, colour names, import layout | 3.8 |  | 0.502 |
| ns | 4986 |  | 173 | Album method roster (complete, names only) | 4.1 | 2.5 | 0.494 |
| walker |  | 5108 | 489 | Code::CodeKey { rung: Decl, file: beetsplug/bpd/__init__.py, decl: 121, sub: 1, line: 1099 } |  |  | 0.494 |
| ns | 5290 |  | 304 | Item method roster (complete, names only) | 4.3 | 2.5 | 0.481 |
| walker |  | 5296 | 188 | Code::CodeKey { rung: Decl, file: beetsplug/bpd/__init__.py, decl: 121, sub: 2, line: 1099 } |  |  | 0.481 |
| ns | 5458 |  | 168 | Item search fields and MediaFile-backed field sets | 4.4 | 2.5 | 0.475 |
| walker |  | 5592 | 296 | Code::CodeKey { rung: Decl, file: beetsplug/bpd/__init__.py, decl: 121, sub: 3, line: 1099 } |  |  | 0.475 |
| walker |  | 5603 | 11 | Code::CodeKey { rung: Doc, file: beetsplug/bpd/__init__.py, decl: 55, sub: 0, line: 361 } |  |  | 0.475 |
| walker |  | 5614 | 11 | Code::CodeKey { rung: Doc, file: beetsplug/bpd/__init__.py, decl: 58, sub: 0, line: 386 } |  |  | 0.475 |
| walker |  | 5625 | 11 | Code::CodeKey { rung: Doc, file: beetsplug/bpd/__init__.py, decl: 112, sub: 0, line: 943 } |  |  | 0.475 |
| ns | 5629 |  | 171 | LibModel shared-method roster (complete after `_fields`) | 4.5 | 2.5 | 0.468 |
| walker |  | 5637 | 12 | Code::CodeKey { rung: Doc, file: beetsplug/bpd/__init__.py, decl: 40, sub: 0, line: 241 } |  |  | 0.468 |
| walker |  | 5649 | 12 | Code::CodeKey { rung: Doc, file: beetsplug/bpd/__init__.py, decl: 54, sub: 0, line: 357 } |  |  | 0.468 |
| walker |  | 5661 | 12 | Code::CodeKey { rung: Doc, file: beetsplug/bpd/__init__.py, decl: 62, sub: 0, line: 458 } |  |  | 0.468 |
| walker |  | 5673 | 12 | Code::CodeKey { rung: Doc, file: beetsplug/bpd/__init__.py, decl: 63, sub: 0, line: 463 } |  |  | 0.468 |
| walker |  | 5685 | 12 | Code::CodeKey { rung: Doc, file: beetsplug/bpd/__init__.py, decl: 64, sub: 0, line: 468 } |  |  | 0.468 |
| walker |  | 5697 | 12 | Code::CodeKey { rung: Doc, file: beetsplug/bpd/__init__.py, decl: 70, sub: 0, line: 514 } |  |  | 0.468 |
| walker |  | 5709 | 12 | Code::CodeKey { rung: Doc, file: beetsplug/bpd/__init__.py, decl: 71, sub: 0, line: 522 } |  |  | 0.468 |
| walker |  | 5721 | 12 | Code::CodeKey { rung: Doc, file: beetsplug/bpd/__init__.py, decl: 87, sub: 0, line: 689 } |  |  | 0.468 |
| walker |  | 5733 | 12 | Code::CodeKey { rung: Doc, file: beetsplug/bpd/__init__.py, decl: 160, sub: 0, line: 1518 } |  |  | 0.468 |
| walker |  | 5746 | 13 | Code::CodeKey { rung: Doc, file: beetsplug/bpd/__init__.py, decl: 45, sub: 0, line: 286 } |  |  | 0.468 |
| walker |  | 5759 | 13 | Code::CodeKey { rung: Doc, file: beetsplug/bpd/__init__.py, decl: 75, sub: 0, line: 552 } |  |  | 0.468 |
| walker |  | 5772 | 13 | Code::CodeKey { rung: Doc, file: beetsplug/bpd/__init__.py, decl: 86, sub: 0, line: 676 } |  |  | 0.468 |
| walker |  | 5785 | 13 | Code::CodeKey { rung: Doc, file: beetsplug/bpd/__init__.py, decl: 94, sub: 0, line: 754 } |  |  | 0.468 |
| ns | 5787 |  | 158 | beets.importer public exports + package docstring | 5.1 |  | 0.462 |
| walker |  | 5798 | 13 | Code::CodeKey { rung: Doc, file: beetsplug/bpd/__init__.py, decl: 170, sub: 0, line: 1601 } |  |  | 0.462 |
| walker |  | 5812 | 14 | Code::CodeKey { rung: Doc, file: beetsplug/bpd/__init__.py, decl: 57, sub: 0, line: 373 } |  |  | 0.462 |
| walker |  | 5826 | 14 | Code::CodeKey { rung: Doc, file: beetsplug/bpd/__init__.py, decl: 77, sub: 0, line: 577 } |  |  | 0.462 |
| walker |  | 5840 | 14 | Code::CodeKey { rung: Doc, file: beetsplug/bpd/__init__.py, decl: 96, sub: 0, line: 763 } |  |  | 0.462 |
| walker |  | 5854 | 14 | Code::CodeKey { rung: Doc, file: beetsplug/bpd/__init__.py, decl: 103, sub: 0, line: 816 } |  |  | 0.462 |
| walker |  | 5868 | 14 | Code::CodeKey { rung: Doc, file: beetsplug/bpd/__init__.py, decl: 105, sub: 0, line: 825 } |  |  | 0.462 |
| walker |  | 5882 | 14 | Code::CodeKey { rung: Doc, file: beetsplug/bpd/__init__.py, decl: 113, sub: 0, line: 950 } |  |  | 0.462 |
| walker |  | 5896 | 14 | Code::CodeKey { rung: Doc, file: beetsplug/bpd/__init__.py, decl: 145, sub: 0, line: 1416 } |  |  | 0.462 |
| walker |  | 5910 | 14 | Code::CodeKey { rung: Doc, file: beetsplug/bpd/__init__.py, decl: 146, sub: 0, line: 1424 } |  |  | 0.462 |
| walker |  | 5925 | 15 | Code::CodeKey { rung: Doc, file: beetsplug/bpd/__init__.py, decl: 41, sub: 0, line: 245 } |  |  | 0.462 |
| walker |  | 5940 | 15 | Code::CodeKey { rung: Doc, file: beetsplug/bpd/__init__.py, decl: 61, sub: 0, line: 453 } |  |  | 0.462 |
| walker |  | 5955 | 15 | Code::CodeKey { rung: Doc, file: beetsplug/bpd/__init__.py, decl: 69, sub: 0, line: 505 } |  |  | 0.462 |
| walker |  | 5970 | 15 | Code::CodeKey { rung: Doc, file: beetsplug/bpd/__init__.py, decl: 73, sub: 0, line: 533 } |  |  | 0.462 |
| walker |  | 5985 | 15 | Code::CodeKey { rung: Doc, file: beetsplug/bpd/__init__.py, decl: 84, sub: 0, line: 648 } |  |  | 0.462 |
| walker |  | 6000 | 15 | Code::CodeKey { rung: Doc, file: beetsplug/bpd/__init__.py, decl: 85, sub: 0, line: 654 } |  |  | 0.462 |
| walker |  | 6015 | 15 | Code::CodeKey { rung: Doc, file: beetsplug/bpd/__init__.py, decl: 130, sub: 0, line: 1204 } |  |  | 0.462 |
| walker |  | 6030 | 15 | Code::CodeKey { rung: Doc, file: beetsplug/bpd/__init__.py, decl: 140, sub: 0, line: 1314 } |  |  | 0.462 |
| ns | 6147 |  | 360 | importer/stages.py: complete stage-function roster with section banners | 5.2 |  | 0.453 |
| walker |  | 6291 | 261 | Code::CodeKey { rung: Names, file: beetsplug/web/__init__.py, decl: 0, sub: 0, line: 0 } |  |  | 0.453 |
| walker |  | 6302 | 11 | Code::CodeKey { rung: Decl, file: beetsplug/web/__init__.py, decl: 20, sub: 0, line: 304 } |  |  | 0.453 |
| walker |  | 6314 | 12 | Code::CodeKey { rung: Decl, file: beetsplug/web/__init__.py, decl: 1, sub: 0, line: 34 } |  |  | 0.453 |
| walker |  | 6336 | 22 | Code::CodeKey { rung: Decl, file: beetsplug/web/__init__.py, decl: 17, sub: 0, line: 283 } |  |  | 0.453 |
| walker |  | 6364 | 28 | Code::CodeKey { rung: Decl, file: beetsplug/web/__init__.py, decl: 11, sub: 0, line: 252 } |  |  | 0.453 |
| walker |  | 6392 | 28 | Code::CodeKey { rung: Decl, file: beetsplug/web/__init__.py, decl: 14, sub: 0, line: 268 } |  |  | 0.453 |
| ns | 6572 |  | 425 | ImportSession.run(): how the pipeline is assembled | 5.3 | 5.2 | 0.440 |
| walker |  | 6593 | 201 | Code::CodeKey { rung: Names, file: beetsplug/web/__init__.py, decl: 0, sub: 1, line: 0 } |  |  | 0.440 |
| walker |  | 6600 | 7 | Code::CodeKey { rung: Decl, file: beetsplug/web/__init__.py, decl: 26, sub: 0, line: 369 } |  |  | 0.440 |
| walker |  | 6612 | 12 | Code::CodeKey { rung: Decl, file: beetsplug/web/__init__.py, decl: 23, sub: 0, line: 338 } |  |  | 0.440 |
| walker |  | 6631 | 19 | Code::CodeKey { rung: Decl, file: beetsplug/web/__init__.py, decl: 21, sub: 0, line: 310 } |  |  | 0.440 |
| walker |  | 6650 | 19 | Code::CodeKey { rung: Decl, file: beetsplug/web/__init__.py, decl: 27, sub: 0, line: 375 } |  |  | 0.440 |
| walker |  | 6825 | 175 | Code::CodeKey { rung: Names, file: beetsplug/web/__init__.py, decl: 0, sub: 2, line: 0 } |  |  | 0.440 |
| ns | 6829 |  | 257 | ImportSession class contract (attributes + the four decision hooks) | 5.4 | 5.3 | 0.431 |
| walker |  | 6833 | 8 | Code::CodeKey { rung: Decl, file: beetsplug/web/__init__.py, decl: 28, sub: 0, line: 382 } |  |  | 0.431 |
| walker |  | 6855 | 22 | Code::CodeKey { rung: Decl, file: beetsplug/web/__init__.py, decl: 34, sub: 0, line: 447 } |  |  | 0.431 |
| walker |  | 6888 | 33 | Code::CodeKey { rung: Decl, file: beetsplug/web/__init__.py, decl: 37, sub: 0, line: 520 } |  |  | 0.431 |
| walker |  | 6903 | 15 | Code::CodeKey { rung: Doc, file: beetsplug/web/__init__.py, decl: 6, sub: 0, line: 117 } |  |  | 0.431 |
| walker |  | 6917 | 14 | Code::CodeKey { rung: Names, file: beetsplug/_utils/__init__.py, decl: 0, sub: 0, line: 0 } |  |  | 0.431 |
| walker |  | 7019 | 102 | Code::CodeKey { rung: Names, file: beetsplug/metasync/__init__.py, decl: 0, sub: 0, line: 0 } |  |  | 0.431 |
| walker |  | 7048 | 29 | Code::CodeKey { rung: Decl, file: beetsplug/metasync/__init__.py, decl: 2, sub: 0, line: 34 } |  |  | 0.431 |
| ns | 7103 |  | 274 | importer/tasks.py: Action enum and the complete task class roster | 5.5 |  | 0.424 |
| walker |  | 7104 | 56 | Code::CodeKey { rung: Decl, file: beetsplug/metasync/__init__.py, decl: 9, sub: 0, line: 76 } |  |  | 0.424 |
| walker |  | 7165 | 61 | Code::CodeKey { rung: Decl, file: beetsplug/metasync/__init__.py, decl: 3, sub: 0, line: 40 } |  |  | 0.424 |
| walker |  | 7170 | 5 | Code::CodeKey { rung: Body, file: beetsplug/metasync/__init__.py, decl: 5, sub: 0, line: 47 } |  |  | 0.424 |
| walker |  | 7177 | 7 | Code::CodeKey { rung: Body, file: beetsplug/metasync/__init__.py, decl: 10, sub: 0, line: 79 } |  |  | 0.424 |
| walker |  | 7191 | 14 | Code::CodeKey { rung: Doc, file: beetsplug/metasync/__init__.py, decl: 12, sub: 0, line: 104 } |  |  | 0.424 |
| walker |  | 7207 | 16 | Code::CodeKey { rung: Doc, file: beetsplug/bpd/__init__.py, decl: 67, sub: 0, line: 487 } |  |  | 0.424 |
| walker |  | 7223 | 16 | Code::CodeKey { rung: Doc, file: beetsplug/bpd/__init__.py, decl: 79, sub: 0, line: 604 } |  |  | 0.424 |
| walker |  | 7239 | 16 | Code::CodeKey { rung: Doc, file: beetsplug/bpd/__init__.py, decl: 88, sub: 0, line: 697 } |  |  | 0.424 |
| walker |  | 7255 | 16 | Code::CodeKey { rung: Doc, file: beetsplug/bpd/__init__.py, decl: 91, sub: 0, line: 731 } |  |  | 0.424 |
| walker |  | 7271 | 16 | Code::CodeKey { rung: Doc, file: beetsplug/bpd/__init__.py, decl: 100, sub: 0, line: 794 } |  |  | 0.424 |
| walker |  | 7287 | 16 | Code::CodeKey { rung: Doc, file: beetsplug/bpd/__init__.py, decl: 104, sub: 0, line: 821 } |  |  | 0.424 |
| walker |  | 7303 | 16 | Code::CodeKey { rung: Doc, file: beetsplug/bpd/__init__.py, decl: 111, sub: 0, line: 939 } |  |  | 0.424 |
| walker |  | 7319 | 16 | Code::CodeKey { rung: Doc, file: beetsplug/bpd/__init__.py, decl: 114, sub: 0, line: 960 } |  |  | 0.424 |
| walker |  | 7335 | 16 | Code::CodeKey { rung: Doc, file: beetsplug/bpd/__init__.py, decl: 120, sub: 0, line: 1079 } |  |  | 0.424 |
| ns | 7349 |  | 246 | BeetsPlugin base class: docstring and declared attributes | 6.1 |  | 0.418 |
| walker |  | 7351 | 16 | Code::CodeKey { rung: Doc, file: beetsplug/bpd/__init__.py, decl: 141, sub: 0, line: 1336 } |  |  | 0.418 |
| walker |  | 7412 | 61 | Code::CodeKey { rung: Names, file: beets/__init__.py, decl: 0, sub: 0, line: 0 } |  |  | 0.418 |
| walker |  | 7438 | 26 | Code::CodeKey { rung: Decl, file: beets/__init__.py, decl: 2, sub: 0, line: 35 } |  |  | 0.418 |
| walker |  | 7448 | 10 | Code::CodeKey { rung: Doc, file: beets/__init__.py, decl: 1, sub: 0, line: 26 } |  |  | 0.418 |
| walker |  | 7465 | 17 | Code::CodeKey { rung: Doc, file: beetsplug/bpd/__init__.py, decl: 43, sub: 0, line: 269 } |  |  | 0.418 |
| walker |  | 7482 | 17 | Code::CodeKey { rung: Doc, file: beetsplug/bpd/__init__.py, decl: 47, sub: 0, line: 297 } |  |  | 0.418 |
| walker |  | 7499 | 17 | Code::CodeKey { rung: Doc, file: beetsplug/bpd/__init__.py, decl: 65, sub: 0, line: 474 } |  |  | 0.418 |
| walker |  | 7516 | 17 | Code::CodeKey { rung: Doc, file: beetsplug/bpd/__init__.py, decl: 66, sub: 0, line: 482 } |  |  | 0.418 |
| walker |  | 7533 | 17 | Code::CodeKey { rung: Doc, file: beetsplug/bpd/__init__.py, decl: 107, sub: 0, line: 905 } |  |  | 0.418 |
| walker |  | 7550 | 17 | Code::CodeKey { rung: Doc, file: beetsplug/bpd/__init__.py, decl: 128, sub: 0, line: 1167 } |  |  | 0.418 |
| walker |  | 7567 | 17 | Code::CodeKey { rung: Doc, file: beetsplug/bpd/__init__.py, decl: 135, sub: 0, line: 1258 } |  |  | 0.418 |
| walker |  | 7584 | 17 | Code::CodeKey { rung: Doc, file: beetsplug/web/__init__.py, decl: 4, sub: 0, line: 103 } |  |  | 0.418 |
| ns | 7595 |  | 246 | BeetsPlugin method roster (complete) | 6.2 | 6.1 | 0.414 |
| walker |  | 7601 | 17 | Code::CodeKey { rung: Doc, file: beetsplug/web/__init__.py, decl: 11, sub: 0, line: 252 } |  |  | 0.414 |
| walker |  | 7729 | 128 | Code::CodeKey { rung: Names, file: beetsplug/discogs/__init__.py, decl: 0, sub: 0, line: 0 } |  |  | 0.414 |
| walker |  | 7785 | 56 | Code::CodeKey { rung: Decl, file: beetsplug/discogs/__init__.py, decl: 4, sub: 0, line: 60 } |  |  | 0.414 |
| walker |  | 7856 | 71 | Code::CodeKey { rung: Decl, file: beetsplug/discogs/__init__.py, decl: 6, sub: 0, line: 83 } |  |  | 0.414 |
| ns | 8017 |  | 422 | The complete `EventType` literal — every plugin event name | 6.3 | 6.1 | 0.402 |
| walker |  | 8046 | 190 | Code::CodeKey { rung: Decl, file: beetsplug/discogs/__init__.py, decl: 5, sub: 0, line: 68 } |  |  | 0.402 |
| walker |  | 8239 | 193 | Code::CodeKey { rung: Decl, file: beetsplug/discogs/__init__.py, decl: 7, sub: 0, line: 93 } |  |  | 0.402 |
| walker |  | 8256 | 17 | Code::CodeKey { rung: Decl, file: beetsplug/discogs/__init__.py, decl: 15, sub: 0, line: 217 } |  |  | 0.402 |
| walker |  | 8276 | 20 | Code::CodeKey { rung: Decl, file: beetsplug/discogs/__init__.py, decl: 14, sub: 0, line: 203 } |  |  | 0.402 |
| ns | 8321 |  | 304 | plugins.py module-level API roster (complete) | 6.4 |  | 0.397 |
| walker |  | 8471 | 195 | Code::CodeKey { rung: Decl, file: beetsplug/discogs/__init__.py, decl: 7, sub: 1, line: 93 } |  |  | 0.397 |
| walker |  | 8484 | 13 | Code::CodeKey { rung: Decl, file: beetsplug/discogs/__init__.py, decl: 21, sub: 0, line: 341 } |  |  | 0.397 |
| walker |  | 8541 | 57 | Code::CodeKey { rung: Decl, file: beetsplug/discogs/__init__.py, decl: 18, sub: 0, line: 264 } |  |  | 0.397 |
| ns | 8633 |  | 312 | MetadataSourcePlugin: the metadata-backend contract | 6.5 |  | 0.392 |
| walker |  | 8738 | 197 | Code::CodeKey { rung: Decl, file: beetsplug/discogs/__init__.py, decl: 7, sub: 2, line: 93 } |  |  | 0.392 |
| walker |  | 8745 | 7 | Code::CodeKey { rung: Decl, file: beetsplug/discogs/__init__.py, decl: 29, sub: 0, line: 705 } |  |  | 0.392 |
| walker |  | 8774 | 29 | Code::CodeKey { rung: Decl, file: beetsplug/discogs/__init__.py, decl: 24, sub: 0, line: 488 } |  |  | 0.392 |
| walker |  | 8803 | 29 | Code::CodeKey { rung: Decl, file: beetsplug/discogs/__init__.py, decl: 26, sub: 0, line: 615 } |  |  | 0.392 |
| ns | 8830 |  | 197 | Query-string prefixes: how `beet ls artist:foo` is parsed | 7.1 |  | 0.388 |
| walker |  | 8848 | 45 | Code::CodeKey { rung: Decl, file: beetsplug/discogs/__init__.py, decl: 28, sub: 0, line: 666 } |  |  | 0.388 |
| walker |  | 8863 | 15 | Code::CodeKey { rung: Doc, file: beetsplug/discogs/__init__.py, decl: 11, sub: 0, line: 165 } |  |  | 0.388 |
| walker |  | 8878 | 15 | Code::CodeKey { rung: Doc, file: beetsplug/discogs/__init__.py, decl: 14, sub: 0, line: 203 } |  |  | 0.388 |
| walker |  | 8895 | 17 | Code::CodeKey { rung: Doc, file: beetsplug/discogs/__init__.py, decl: 28, sub: 0, line: 666 } |  |  | 0.388 |
| walker |  | 8913 | 18 | Code::CodeKey { rung: Doc, file: beetsplug/bpd/__init__.py, decl: 95, sub: 0, line: 757 } |  |  | 0.388 |
| walker |  | 8931 | 18 | Code::CodeKey { rung: Doc, file: beetsplug/bpd/__init__.py, decl: 101, sub: 0, line: 797 } |  |  | 0.388 |
| walker |  | 8949 | 18 | Code::CodeKey { rung: Doc, file: beetsplug/bpd/__init__.py, decl: 108, sub: 0, line: 908 } |  |  | 0.388 |
| walker |  | 8967 | 18 | Code::CodeKey { rung: Doc, file: beetsplug/bpd/__init__.py, decl: 124, sub: 0, line: 1117 } |  |  | 0.388 |
| walker |  | 8985 | 18 | Code::CodeKey { rung: Doc, file: beetsplug/bpd/__init__.py, decl: 131, sub: 0, line: 1209 } |  |  | 0.388 |
| walker |  | 9003 | 18 | Code::CodeKey { rung: Doc, file: beetsplug/bpd/__init__.py, decl: 133, sub: 0, line: 1248 } |  |  | 0.388 |
| walker |  | 9021 | 18 | Code::CodeKey { rung: Doc, file: beetsplug/bpd/__init__.py, decl: 166, sub: 0, line: 1565 } |  |  | 0.388 |
| walker |  | 9039 | 18 | Code::CodeKey { rung: Doc, file: beetsplug/discogs/__init__.py, decl: 19, sub: 0, line: 308 } |  |  | 0.388 |
| walker |  | 9057 | 18 | Code::CodeKey { rung: Doc, file: beetsplug/discogs/__init__.py, decl: 22, sub: 0, line: 353 } |  |  | 0.388 |
| walker |  | 9075 | 18 | Code::CodeKey { rung: Doc, file: beetsplug/discogs/__init__.py, decl: 30, sub: 0, line: 722 } |  |  | 0.388 |
| walker |  | 9093 | 18 | Code::CodeKey { rung: Doc, file: beetsplug/web/__init__.py, decl: 10, sub: 0, line: 241 } |  |  | 0.388 |
| walker |  | 9111 | 18 | Code::CodeKey { rung: Doc, file: beetsplug/web/__init__.py, decl: 14, sub: 0, line: 268 } |  |  | 0.388 |
| walker |  | 9181 | 70 | Code::CodeKey { rung: Names, file: beetsplug/tidal/__init__.py, decl: 0, sub: 0, line: 0 } |  |  | 0.388 |
| ns | 9187 |  | 357 | dbcore/query.py class roster (complete) | 7.2 |  | 0.380 |
| walker |  | 9260 | 79 | Code::CodeKey { rung: Decl, file: beetsplug/tidal/__init__.py, decl: 28, sub: 0, line: 494 } |  |  | 0.380 |
| walker |  | 9446 | 186 | Code::CodeKey { rung: Decl, file: beetsplug/tidal/__init__.py, decl: 2, sub: 0, line: 37 } |  |  | 0.380 |
| ns | 9539 |  | 352 | beets.autotag exports, Recommendation, Proposal | 8.1 |  | 0.373 |
| walker |  | 9659 | 213 | Code::CodeKey { rung: Decl, file: beetsplug/tidal/__init__.py, decl: 2, sub: 1, line: 37 } |  |  | 0.373 |
| walker |  | 9676 | 17 | Code::CodeKey { rung: Decl, file: beetsplug/tidal/__init__.py, decl: 13, sub: 0, line: 142 } |  |  | 0.373 |
| walker |  | 9702 | 26 | Code::CodeKey { rung: Decl, file: beetsplug/tidal/__init__.py, decl: 12, sub: 0, line: 119 } |  |  | 0.373 |
| walker |  | 9754 | 52 | Code::CodeKey { rung: Decl, file: beetsplug/tidal/__init__.py, decl: 19, sub: 0, line: 295 } |  |  | 0.373 |
| ns | 9775 |  | 236 | autotag/hooks.py and distance.py symbol roster | 8.2 |  | 0.370 |
| walker |  | 9807 | 53 | Code::CodeKey { rung: Decl, file: beetsplug/tidal/__init__.py, decl: 18, sub: 0, line: 236 } |  |  | 0.370 |
| walker |  | 9818 | 11 | Code::CodeKey { rung: Doc, file: beetsplug/tidal/__init__.py, decl: 14, sub: 0, line: 161 } |  |  | 0.370 |
| walker |  | 9829 | 11 | Code::CodeKey { rung: Doc, file: beetsplug/tidal/__init__.py, decl: 15, sub: 0, line: 169 } |  |  | 0.370 |
| ns | 9970 |  | 195 | beets/util/__init__.py: classes and the core path/file-operation helpers | 9.1 |  | 0.365 |
| walker |  | 9977 | 148 | Code::CodeKey { rung: Decl, file: beetsplug/tidal/__init__.py, decl: 2, sub: 2, line: 37 } |  |  | 0.365 |
