Score(3000)=0.624 I=0.780 C=0.500 ns_rows≤3K=17/44 grid(1000/1442/2080/3000/4327/6240/9000)=0.594/0.599/0.579/0.624/0.548/0.468/0.401

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
| walker |  | 863 | 4 | Fs::DirListing { dir: docs/_templates } |  |  | 0.639 |
| walker |  | 867 | 4 | Fs::DirListing { dir: docs/extensions } |  |  | 0.639 |
| walker |  | 881 | 14 | Fs::DirListing { dir: docs/_static } |  |  | 0.639 |
| ns | 885 |  | 195 | README capability bullets, part 2 (files, art, web, MPD) | 1.8 |  | 0.593 |
| walker |  | 903 | 22 | Fs::DirListing { dir: docs/api } |  |  | 0.593 |
| walker |  | 929 | 26 | Fs::DirListing { dir: docs/guides } |  |  | 0.593 |
| walker |  | 955 | 26 | Fs::DirListing { dir: docs/reference } |  |  | 0.593 |
| walker |  | 983 | 28 | Fs::DirListing { dir: docs/dev } |  |  | 0.593 |
| walker |  | 1010 | 27 | Toml::Operational { file: pyproject.toml } |  |  | 0.596 |
| walker |  | 1031 | 21 | Fs::DirListing { dir: docs/_templates/autosummary } |  |  | 0.596 |
| ns | 1051 |  | 166 | UI, util and test-helper subpackage listings | 1.9 |  | 0.606 |
| walker |  | 1095 | 64 | Code::CodeKey { rung: ModuleDoc, file: beets/ui/__init__.py, decl: 0, sub: 0, line: 0 } |  |  | 0.606 |
| walker |  | 1119 | 24 | Fs::DirListing { dir: docs/dev/plugins } |  |  | 0.606 |
| walker |  | 1134 | 15 | Fs::DirListing { dir: beets/test } |  |  | 0.629 |
| walker |  | 1173 | 39 | Fs::DirListing { dir: .github } |  |  | 0.629 |
| walker |  | 1204 | 31 | Fs::DirListing { dir: .github/workflows } |  |  | 0.629 |
| ns | 1227 |  | 176 | Canonical test / lint / typecheck commands | 1.10 |  | 0.590 |
| walker |  | 1245 | 41 | Fs::DirListing { dir: docs/dev/plugins/other } |  |  | 0.590 |
| ns | 1445 |  | 218 | test/ and docs/ top-level listings (complete) | 1.11 |  | 0.524 |
| walker |  | 1628 | 383 | Fs::DirListing { dir: beetsplug } |  |  | 0.552 |
| walker |  | 1639 | 11 | Fs::DirListing { dir: beetsplug/bpd } |  |  | 0.552 |
| walker |  | 1651 | 12 | Fs::DirListing { dir: beetsplug/web } |  |  | 0.552 |
| walker |  | 1665 | 14 | Fs::DirListing { dir: beetsplug/discogs } |  |  | 0.552 |
| walker |  | 1681 | 16 | Fs::DirListing { dir: beetsplug/metasync } |  |  | 0.552 |
| walker |  | 1685 | 4 | Fs::DirListing { dir: beetsplug/web/templates } |  |  | 0.552 |
| walker |  | 1704 | 19 | Fs::DirListing { dir: beetsplug/tidal } |  |  | 0.552 |
| walker |  | 1727 | 23 | Fs::DirListing { dir: beetsplug/lastgenre } |  |  | 0.552 |
| walker |  | 1743 | 16 | Code::CodeKey { rung: ModuleDoc, file: beetsplug/web/__init__.py, decl: 0, sub: 0, line: 0 } |  |  | 0.552 |
| walker |  | 1760 | 17 | Code::CodeKey { rung: ModuleDoc, file: beetsplug/metasync/__init__.py, decl: 0, sub: 0, line: 0 } |  |  | 0.552 |
| walker |  | 1790 | 30 | Fs::DirListing { dir: beetsplug/_utils } |  |  | 0.552 |
| ns | 1828 |  | 383 | beetsplug/ bundled plugin listing (complete, 81 entries) | 1.12 |  | 0.601 |
| walker |  | 1829 | 39 | Code::CodeKey { rung: ModuleDoc, file: beetsplug/discogs/__init__.py, decl: 0, sub: 0, line: 0 } |  |  | 0.601 |
| walker |  | 1851 | 22 | Code::CodeKey { rung: ModuleDoc, file: beetsplug/zero.py, decl: 0, sub: 0, line: 0 } |  |  | 0.601 |
| walker |  | 1874 | 23 | Code::CodeKey { rung: ModuleDoc, file: beetsplug/subsonicupdate.py, decl: 0, sub: 0, line: 0 } |  |  | 0.601 |
| walker |  | 1898 | 24 | Code::CodeKey { rung: ModuleDoc, file: beetsplug/subsonicplaylist.py, decl: 0, sub: 0, line: 0 } |  |  | 0.601 |
| walker |  | 1922 | 24 | Code::CodeKey { rung: ModuleDoc, file: beetsplug/unimported.py, decl: 0, sub: 0, line: 0 } |  |  | 0.601 |
| walker |  | 1985 | 63 | Code::CodeKey { rung: ModuleDoc, file: beetsplug/bpd/__init__.py, decl: 0, sub: 0, line: 0 } |  |  | 0.601 |
| ns | 2006 |  | 178 | beets.library public export block (complete) | 2.1 |  | 0.579 |
| walker |  | 2007 | 22 | Fs::DirListing { dir: beetsplug/web/static } |  |  | 0.579 |
| walker |  | 2041 | 34 | Code::CodeKey { rung: ModuleDoc, file: beetsplug/titlecase.py, decl: 0, sub: 0, line: 0 } |  |  | 0.579 |
| walker |  | 2076 | 35 | Code::CodeKey { rung: ModuleDoc, file: beetsplug/ihate.py, decl: 0, sub: 0, line: 0 } |  |  | 0.579 |
| walker |  | 2111 | 35 | Code::CodeKey { rung: ModuleDoc, file: beetsplug/the.py, decl: 0, sub: 0, line: 0 } |  |  | 0.579 |
| ns | 2183 |  | 177 | The `beet` subcommand roster (default_commands) | 2.2 |  | 0.558 |
| walker |  | 2216 | 105 | Code::CodeKey { rung: ModuleDoc, file: beetsplug/lastgenre/__init__.py, decl: 0, sub: 0, line: 0 } |  |  | 0.558 |
| ns | 2373 |  | 190 | Library class: models and schema migrations | 2.3 |  | 0.545 |
| walker |  | 2421 | 205 | Markdown::Section { file: README.rst, section_index: 0, keeps_default_concavity: false } |  |  | 0.680 |
| ns | 2528 |  | 155 | Library method roster (complete) | 2.4 | 2.3 | 0.667 |
| walker |  | 2865 | 444 | Code::CodeKey { rung: Names, file: beetsplug/web/__init__.py, decl: 0, sub: 0, line: 0 } |  |  | 0.667 |
| walker |  | 2873 | 8 | Code::CodeKey { rung: Decl, file: beetsplug/web/__init__.py, decl: 32, sub: 0, line: 439 } |  |  | 0.667 |
| walker |  | 2882 | 9 | Code::CodeKey { rung: Decl, file: beetsplug/web/__init__.py, decl: 18, sub: 0, line: 296 } |  |  | 0.667 |
| walker |  | 2892 | 10 | Code::CodeKey { rung: Decl, file: beetsplug/web/__init__.py, decl: 30, sub: 0, line: 412 } |  |  | 0.667 |
| walker |  | 2902 | 10 | Code::CodeKey { rung: Decl, file: beetsplug/web/__init__.py, decl: 31, sub: 0, line: 423 } |  |  | 0.667 |
| walker |  | 2916 | 14 | Code::CodeKey { rung: Decl, file: beetsplug/web/__init__.py, decl: 23, sub: 0, line: 344 } |  |  | 0.667 |
| walker |  | 2931 | 15 | Code::CodeKey { rung: Decl, file: beetsplug/web/__init__.py, decl: 24, sub: 0, line: 354 } |  |  | 0.667 |
| walker |  | 2946 | 15 | Code::CodeKey { rung: Decl, file: beetsplug/web/__init__.py, decl: 29, sub: 0, line: 397 } |  |  | 0.667 |
| ns | 2952 |  | 424 | models.py class headers: LibModel, FormattedItemMapping, Album, Item | 2.5 |  | 0.624 |
| walker |  | 2962 | 16 | Code::CodeKey { rung: Decl, file: beetsplug/web/__init__.py, decl: 21, sub: 0, line: 317 } |  |  | 0.624 |
| walker |  | 2979 | 17 | Code::CodeKey { rung: Decl, file: beetsplug/web/__init__.py, decl: 28, sub: 0, line: 388 } |  |  | 0.624 |
| walker |  | 3001 | 22 | Code::CodeKey { rung: Decl, file: beetsplug/web/__init__.py, decl: 16, sub: 0, line: 283 } |  |  | 0.624 |
| walker |  | 3023 | 22 | Code::CodeKey { rung: Decl, file: beetsplug/web/__init__.py, decl: 33, sub: 0, line: 447 } |  |  | 0.624 |
| walker |  | 3051 | 28 | Code::CodeKey { rung: Decl, file: beetsplug/web/__init__.py, decl: 10, sub: 0, line: 252 } |  |  | 0.624 |
| walker |  | 3079 | 28 | Code::CodeKey { rung: Decl, file: beetsplug/web/__init__.py, decl: 13, sub: 0, line: 268 } |  |  | 0.624 |
| walker |  | 3110 | 31 | Code::CodeKey { rung: Decl, file: beetsplug/web/__init__.py, decl: 20, sub: 0, line: 310 } |  |  | 0.624 |
| ns | 3125 |  | 173 | beets.dbcore public export block + package docstring | 2.6 |  | 0.607 |
| walker |  | 3141 | 31 | Code::CodeKey { rung: Decl, file: beetsplug/web/__init__.py, decl: 25, sub: 0, line: 369 } |  |  | 0.607 |
| walker |  | 3172 | 31 | Code::CodeKey { rung: Decl, file: beetsplug/web/__init__.py, decl: 26, sub: 0, line: 375 } |  |  | 0.607 |
| walker |  | 3204 | 32 | Code::CodeKey { rung: Decl, file: beetsplug/web/__init__.py, decl: 27, sub: 0, line: 382 } |  |  | 0.607 |
| walker |  | 3237 | 33 | Code::CodeKey { rung: Decl, file: beetsplug/web/__init__.py, decl: 36, sub: 0, line: 520 } |  |  | 0.607 |
| walker |  | 3275 | 38 | Code::CodeKey { rung: Decl, file: beetsplug/web/__init__.py, decl: 19, sub: 0, line: 304 } |  |  | 0.607 |
| ns | 3298 |  | 173 | config_default.yaml: library, directory, plugins, ignore rules | 3.1 |  | 0.592 |
| walker |  | 3314 | 39 | Code::CodeKey { rung: Decl, file: beetsplug/web/__init__.py, decl: 22, sub: 0, line: 338 } |  |  | 0.592 |
| walker |  | 3329 | 15 | Code::CodeKey { rung: Doc, file: beetsplug/web/__init__.py, decl: 5, sub: 0, line: 117 } |  |  | 0.592 |
| walker |  | 3346 | 17 | Code::CodeKey { rung: Doc, file: beetsplug/web/__init__.py, decl: 3, sub: 0, line: 103 } |  |  | 0.592 |
| walker |  | 3363 | 17 | Code::CodeKey { rung: Doc, file: beetsplug/web/__init__.py, decl: 10, sub: 0, line: 252 } |  |  | 0.592 |
| walker |  | 3381 | 18 | Code::CodeKey { rung: Doc, file: beetsplug/web/__init__.py, decl: 9, sub: 0, line: 241 } |  |  | 0.592 |
| walker |  | 3399 | 18 | Code::CodeKey { rung: Doc, file: beetsplug/web/__init__.py, decl: 13, sub: 0, line: 268 } |  |  | 0.592 |
| walker |  | 3418 | 19 | Code::CodeKey { rung: Doc, file: beetsplug/web/__init__.py, decl: 7, sub: 0, line: 176 } |  |  | 0.592 |
| walker |  | 3438 | 20 | Code::CodeKey { rung: Doc, file: beetsplug/web/__init__.py, decl: 6, sub: 0, line: 122 } |  |  | 0.592 |
| walker |  | 3468 | 30 | Code::CodeKey { rung: Doc, file: beetsplug/web/__init__.py, decl: 4, sub: 0, line: 109 } |  |  | 0.592 |
| walker |  | 3502 | 34 | Code::CodeKey { rung: Doc, file: beetsplug/web/__init__.py, decl: 8, sub: 0, line: 223 } |  |  | 0.592 |
| walker |  | 3558 | 56 | Code::CodeKey { rung: Doc, file: beetsplug/web/__init__.py, decl: 1, sub: 0, line: 46 } |  |  | 0.592 |
| ns | 3687 |  | 389 | config_default.yaml: the complete `import:` option block | 3.2 |  | 0.562 |
| walker |  | 3769 | 211 | Code::CodeKey { rung: Names, file: beets/ui/commands/__init__.py, decl: 0, sub: 0, line: 0 } |  |  | 0.562 |
| walker |  | 3779 | 10 | Code::CodeKey { rung: Doc, file: beets/ui/commands/__init__.py, decl: 1, sub: 0, line: 36 } |  |  | 0.562 |
| ns | 3866 |  | 179 | config_default.yaml: tagging defaults, `paths:` templates, threading | 3.4 |  | 0.550 |
| walker |  | 3886 | 107 | Code::CodeKey { rung: Decl, file: beets/ui/commands/__init__.py, decl: 2, sub: 0, line: 50 } |  |  | 0.566 |
| walker |  | 4143 | 257 | Code::CodeKey { rung: Names, file: beetsplug/bpd/__init__.py, decl: 0, sub: 0, line: 0 } |  |  | 0.566 |
| ns | 4171 |  | 305 | Path-template function roster: DefaultTemplateFunctions (complete) | 3.5 |  | 0.554 |
| ns | 4291 |  | 120 | config_default.yaml: display formats and default sorts | 3.6 |  | 0.548 |
| walker |  | 4536 | 393 | Code::CodeKey { rung: Names, file: beetsplug/bpd/__init__.py, decl: 0, sub: 1, line: 0 } |  |  | 0.548 |
| walker |  | 4552 | 16 | Code::CodeKey { rung: Decl, file: beetsplug/bpd/__init__.py, decl: 36, sub: 0, line: 180 } |  |  | 0.548 |
| walker |  | 4586 | 34 | Code::CodeKey { rung: Decl, file: beetsplug/bpd/__init__.py, decl: 118, sub: 0, line: 1064 } |  |  | 0.548 |
| walker |  | 4639 | 53 | Code::CodeKey { rung: Decl, file: beetsplug/bpd/__init__.py, decl: 168, sub: 0, line: 1583 } |  |  | 0.548 |
| ns | 4673 |  | 382 | config_default.yaml: autotagger thresholds and distance weights | 3.7 |  | 0.527 |
| walker |  | 4700 | 61 | Code::CodeKey { rung: Decl, file: beetsplug/bpd/__init__.py, decl: 25, sub: 0, line: 80 } |  |  | 0.527 |
| walker |  | 4762 | 62 | Code::CodeKey { rung: Decl, file: beetsplug/bpd/__init__.py, decl: 27, sub: 0, line: 110 } |  |  | 0.527 |
| ns | 4813 |  | 140 | config_default.yaml: terminal UI, colour names, import layout | 3.8 |  | 0.518 |
| walker |  | 4833 | 71 | Code::CodeKey { rung: Decl, file: beetsplug/bpd/__init__.py, decl: 94, sub: 0, line: 754 } |  |  | 0.518 |
| walker |  | 4841 | 8 | Code::CodeKey { rung: Decl, file: beetsplug/bpd/__init__.py, decl: 99, sub: 0, line: 785 } |  |  | 0.518 |
| walker |  | 4854 | 13 | Code::CodeKey { rung: Doc, file: beetsplug/bpd/__init__.py, decl: 94, sub: 0, line: 754 } |  |  | 0.518 |
| walker |  | 4867 | 13 | Code::CodeKey { rung: Doc, file: beetsplug/bpd/__init__.py, decl: 170, sub: 0, line: 1601 } |  |  | 0.518 |
| walker |  | 4950 | 83 | Code::CodeKey { rung: Decl, file: beetsplug/bpd/__init__.py, decl: 100, sub: 0, line: 794 } |  |  | 0.518 |
| walker |  | 4964 | 14 | Code::CodeKey { rung: Doc, file: beetsplug/bpd/__init__.py, decl: 96, sub: 0, line: 763 } |  |  | 0.518 |
| walker |  | 4978 | 14 | Code::CodeKey { rung: Doc, file: beetsplug/bpd/__init__.py, decl: 103, sub: 0, line: 816 } |  |  | 0.518 |
| ns | 4986 |  | 173 | Album method roster (complete, names only) | 4.1 | 2.5 | 0.510 |
| walker |  | 4992 | 14 | Code::CodeKey { rung: Doc, file: beetsplug/bpd/__init__.py, decl: 105, sub: 0, line: 825 } |  |  | 0.510 |
| walker |  | 5081 | 89 | Code::CodeKey { rung: Decl, file: beetsplug/bpd/__init__.py, decl: 107, sub: 0, line: 905 } |  |  | 0.510 |
| walker |  | 5092 | 11 | Code::CodeKey { rung: Doc, file: beetsplug/bpd/__init__.py, decl: 112, sub: 0, line: 943 } |  |  | 0.510 |
| walker |  | 5106 | 14 | Code::CodeKey { rung: Doc, file: beetsplug/bpd/__init__.py, decl: 113, sub: 0, line: 950 } |  |  | 0.510 |
| walker |  | 5122 | 16 | Code::CodeKey { rung: Doc, file: beetsplug/bpd/__init__.py, decl: 100, sub: 0, line: 794 } |  |  | 0.510 |
| walker |  | 5138 | 16 | Code::CodeKey { rung: Doc, file: beetsplug/bpd/__init__.py, decl: 104, sub: 0, line: 821 } |  |  | 0.510 |
| walker |  | 5154 | 16 | Code::CodeKey { rung: Doc, file: beetsplug/bpd/__init__.py, decl: 111, sub: 0, line: 939 } |  |  | 0.510 |
| walker |  | 5170 | 16 | Code::CodeKey { rung: Doc, file: beetsplug/bpd/__init__.py, decl: 120, sub: 0, line: 1079 } |  |  | 0.510 |
| walker |  | 5273 | 103 | Code::CodeKey { rung: Decl, file: beetsplug/bpd/__init__.py, decl: 114, sub: 0, line: 960 } |  |  | 0.510 |
| walker |  | 5289 | 16 | Code::CodeKey { rung: Doc, file: beetsplug/bpd/__init__.py, decl: 114, sub: 0, line: 960 } |  |  | 0.510 |
| ns | 5290 |  | 304 | Item method roster (complete, names only) | 4.3 | 2.5 | 0.497 |
| walker |  | 5306 | 17 | Code::CodeKey { rung: Doc, file: beetsplug/bpd/__init__.py, decl: 107, sub: 0, line: 905 } |  |  | 0.497 |
| walker |  | 5324 | 18 | Code::CodeKey { rung: Doc, file: beetsplug/bpd/__init__.py, decl: 95, sub: 0, line: 757 } |  |  | 0.497 |
| walker |  | 5342 | 18 | Code::CodeKey { rung: Doc, file: beetsplug/bpd/__init__.py, decl: 101, sub: 0, line: 797 } |  |  | 0.497 |
| walker |  | 5360 | 18 | Code::CodeKey { rung: Doc, file: beetsplug/bpd/__init__.py, decl: 108, sub: 0, line: 908 } |  |  | 0.497 |
| ns | 5458 |  | 168 | Item search fields and MediaFile-backed field sets | 4.4 | 2.5 | 0.490 |
| walker |  | 5475 | 115 | Code::CodeKey { rung: Decl, file: beetsplug/bpd/__init__.py, decl: 26, sub: 0, line: 90 } |  |  | 0.490 |
| walker |  | 5494 | 19 | Code::CodeKey { rung: Doc, file: beetsplug/bpd/__init__.py, decl: 30, sub: 0, line: 138 } |  |  | 0.490 |
| walker |  | 5513 | 19 | Code::CodeKey { rung: Doc, file: beetsplug/bpd/__init__.py, decl: 110, sub: 0, line: 915 } |  |  | 0.490 |
| walker |  | 5538 | 25 | Code::CodeKey { rung: Doc, file: beetsplug/bpd/__init__.py, decl: 102, sub: 0, line: 804 } |  |  | 0.490 |
| walker |  | 5568 | 30 | Code::CodeKey { rung: Doc, file: beetsplug/bpd/__init__.py, decl: 35, sub: 0, line: 174 } |  |  | 0.490 |
| walker |  | 5599 | 31 | Code::CodeKey { rung: Doc, file: beetsplug/bpd/__init__.py, decl: 27, sub: 0, line: 110 } |  |  | 0.490 |
| ns | 5629 |  | 171 | LibModel shared-method roster (complete after `_fields`) | 4.5 | 2.5 | 0.484 |
| walker |  | 5631 | 32 | Code::CodeKey { rung: Doc, file: beetsplug/bpd/__init__.py, decl: 106, sub: 0, line: 838 } |  |  | 0.484 |
| walker |  | 5663 | 32 | Code::CodeKey { rung: Doc, file: beetsplug/bpd/__init__.py, decl: 117, sub: 0, line: 1017 } |  |  | 0.484 |
| walker |  | 5697 | 34 | Code::CodeKey { rung: Doc, file: beetsplug/bpd/__init__.py, decl: 29, sub: 0, line: 123 } |  |  | 0.484 |
| walker |  | 5732 | 35 | Code::CodeKey { rung: Doc, file: beetsplug/bpd/__init__.py, decl: 168, sub: 0, line: 1583 } |  |  | 0.484 |
| walker |  | 5772 | 40 | Code::CodeKey { rung: Doc, file: beetsplug/bpd/__init__.py, decl: 36, sub: 0, line: 180 } |  |  | 0.484 |
| ns | 5787 |  | 158 | beets.importer public exports + package docstring | 5.1 |  | 0.477 |
| walker |  | 5813 | 41 | Code::CodeKey { rung: Doc, file: beetsplug/bpd/__init__.py, decl: 115, sub: 0, line: 966 } |  |  | 0.477 |
| walker |  | 5861 | 48 | Code::CodeKey { rung: Doc, file: beetsplug/bpd/__init__.py, decl: 119, sub: 0, line: 1070 } |  |  | 0.477 |
| walker |  | 6119 | 258 | Code::CodeKey { rung: Names, file: beets/ui/__init__.py, decl: 0, sub: 0, line: 0 } |  |  | 0.477 |
| walker |  | 6146 | 27 | Code::CodeKey { rung: Decl, file: beets/ui/__init__.py, decl: 2, sub: 0, line: 65 } |  |  | 0.477 |
| ns | 6147 |  | 360 | importer/stages.py: complete stage-function roster with section banners | 5.2 |  | 0.468 |
| walker |  | 6158 | 12 | Code::CodeKey { rung: Body, file: beets/ui/__init__.py, decl: 4, sub: 0, line: 80 } |  |  | 0.468 |
| walker |  | 6170 | 12 | Code::CodeKey { rung: Body, file: beets/ui/__init__.py, decl: 5, sub: 0, line: 85 } |  |  | 0.468 |
| walker |  | 6189 | 19 | Code::CodeKey { rung: Doc, file: beets/ui/__init__.py, decl: 4, sub: 0, line: 80 } |  |  | 0.468 |
| walker |  | 6208 | 19 | Code::CodeKey { rung: Doc, file: beets/ui/__init__.py, decl: 5, sub: 0, line: 85 } |  |  | 0.468 |
| walker |  | 6245 | 37 | Code::CodeKey { rung: Doc, file: beets/ui/__init__.py, decl: 3, sub: 0, line: 71 } |  |  | 0.468 |
| walker |  | 6302 | 57 | Code::CodeKey { rung: Doc, file: beets/ui/__init__.py, decl: 6, sub: 0, line: 90 } |  |  | 0.468 |
| walker |  | 6368 | 66 | Code::CodeKey { rung: Doc, file: beetsplug/bpd/__init__.py, decl: 118, sub: 0, line: 1064 } |  |  | 0.468 |
| walker |  | 6554 | 186 | Code::CodeKey { rung: Names, file: beetsplug/metasync/__init__.py, decl: 0, sub: 0, line: 0 } |  |  | 0.468 |
| ns | 6572 |  | 425 | ImportSession.run(): how the pipeline is assembled | 5.3 | 5.2 | 0.454 |
| walker |  | 6583 | 29 | Code::CodeKey { rung: Decl, file: beetsplug/metasync/__init__.py, decl: 2, sub: 0, line: 34 } |  |  | 0.454 |
| walker |  | 6635 | 52 | Code::CodeKey { rung: Decl, file: beetsplug/metasync/__init__.py, decl: 3, sub: 0, line: 40 } |  |  | 0.454 |
| walker |  | 6644 | 9 | Code::CodeKey { rung: Decl, file: beetsplug/metasync/__init__.py, decl: 5, sub: 0, line: 47 } |  |  | 0.454 |
| walker |  | 6649 | 5 | Code::CodeKey { rung: Body, file: beetsplug/metasync/__init__.py, decl: 5, sub: 0, line: 47 } |  |  | 0.454 |
| walker |  | 6705 | 56 | Code::CodeKey { rung: Decl, file: beetsplug/metasync/__init__.py, decl: 9, sub: 0, line: 76 } |  |  | 0.454 |
| walker |  | 6719 | 14 | Code::CodeKey { rung: Doc, file: beetsplug/metasync/__init__.py, decl: 12, sub: 0, line: 104 } |  |  | 0.454 |
| walker |  | 6738 | 19 | Code::CodeKey { rung: Doc, file: beetsplug/metasync/__init__.py, decl: 8, sub: 0, line: 68 } |  |  | 0.454 |
| walker |  | 6745 | 7 | Code::CodeKey { rung: Body, file: beetsplug/metasync/__init__.py, decl: 10, sub: 0, line: 79 } |  |  | 0.454 |
| walker |  | 6790 | 45 | Code::CodeKey { rung: Doc, file: beetsplug/metasync/__init__.py, decl: 6, sub: 0, line: 52 } |  |  | 0.454 |
| ns | 6829 |  | 257 | ImportSession class contract (attributes + the four decision hooks) | 5.4 | 5.3 | 0.446 |
| walker |  | 6862 | 72 | Code::CodeKey { rung: Names, file: beets/__init__.py, decl: 0, sub: 0, line: 0 } |  |  | 0.446 |
| walker |  | 6888 | 26 | Code::CodeKey { rung: Decl, file: beets/__init__.py, decl: 2, sub: 0, line: 35 } |  |  | 0.446 |
| walker |  | 6898 | 10 | Code::CodeKey { rung: Doc, file: beets/__init__.py, decl: 1, sub: 0, line: 26 } |  |  | 0.446 |
| walker |  | 6938 | 40 | Code::CodeKey { rung: Doc, file: beets/__init__.py, decl: 2, sub: 0, line: 35 } |  |  | 0.446 |
| walker |  | 6995 | 57 | Code::CodeKey { rung: Body, file: beets/__init__.py, decl: 1, sub: 0, line: 26 } |  |  | 0.446 |
| ns | 7103 |  | 274 | importer/tasks.py: Action enum and the complete task class roster | 5.5 |  | 0.438 |
| walker |  | 7280 | 285 | Code::CodeKey { rung: Names, file: beets/ui/__init__.py, decl: 0, sub: 1, line: 0 } |  |  | 0.438 |
| walker |  | 7287 | 7 | Code::CodeKey { rung: Decl, file: beets/ui/__init__.py, decl: 18, sub: 0, line: 459 } |  |  | 0.438 |
| walker |  | 7313 | 26 | Code::CodeKey { rung: Decl, file: beets/ui/__init__.py, decl: 40, sub: 0, line: 807 } |  |  | 0.438 |
| ns | 7349 |  | 246 | BeetsPlugin base class: docstring and declared attributes | 6.1 |  | 0.432 |
| walker |  | 7377 | 64 | Code::CodeKey { rung: Decl, file: beets/ui/__init__.py, decl: 13, sub: 0, line: 207 } |  |  | 0.432 |
| walker |  | 7449 | 72 | Code::CodeKey { rung: Decl, file: beets/ui/__init__.py, decl: 19, sub: 0, line: 466 } |  |  | 0.432 |
| walker |  | 7464 | 15 | Code::CodeKey { rung: Doc, file: beets/ui/__init__.py, decl: 43, sub: 0, line: 879 } |  |  | 0.432 |
| walker |  | 7480 | 16 | Code::CodeKey { rung: Doc, file: beets/ui/__init__.py, decl: 17, sub: 0, line: 445 } |  |  | 0.432 |
| walker |  | 7496 | 16 | Code::CodeKey { rung: Doc, file: beets/ui/__init__.py, decl: 18, sub: 0, line: 459 } |  |  | 0.432 |
| walker |  | 7594 | 98 | Code::CodeKey { rung: Decl, file: beets/ui/__init__.py, decl: 33, sub: 0, line: 676 } |  |  | 0.432 |
| ns | 7595 |  | 246 | BeetsPlugin method roster (complete) | 6.2 | 6.1 | 0.428 |
| walker |  | 7611 | 17 | Code::CodeKey { rung: Doc, file: beets/ui/__init__.py, decl: 41, sub: 0, line: 830 } |  |  | 0.428 |
| walker |  | 7716 | 105 | Code::CodeKey { rung: Decl, file: beets/ui/__init__.py, decl: 27, sub: 0, line: 637 } |  |  | 0.428 |
| walker |  | 7724 | 8 | Code::CodeKey { rung: Decl, file: beets/ui/__init__.py, decl: 31, sub: 0, line: 664 } |  |  | 0.428 |
| walker |  | 7734 | 10 | Code::CodeKey { rung: Decl, file: beets/ui/__init__.py, decl: 32, sub: 0, line: 668 } |  |  | 0.428 |
| walker |  | 7847 | 113 | Code::CodeKey { rung: Decl, file: beets/ui/__init__.py, decl: 20, sub: 0, line: 498 } |  |  | 0.428 |
| walker |  | 7914 | 67 | Code::CodeKey { rung: Decl, file: beets/ui/__init__.py, decl: 23, sub: 0, line: 534 } |  |  | 0.428 |
| walker |  | 7928 | 14 | Code::CodeKey { rung: Doc, file: beets/ui/__init__.py, decl: 26, sub: 0, line: 621 } |  |  | 0.428 |
| walker |  | 7947 | 19 | Code::CodeKey { rung: Doc, file: beets/ui/__init__.py, decl: 35, sub: 0, line: 701 } |  |  | 0.428 |
| walker |  | 7967 | 20 | Code::CodeKey { rung: Doc, file: beets/ui/__init__.py, decl: 9, sub: 0, line: 150 } |  |  | 0.428 |
| walker |  | 7996 | 29 | Code::CodeKey { rung: Doc, file: beets/ui/__init__.py, decl: 33, sub: 0, line: 676 } |  |  | 0.428 |
| ns | 8017 |  | 422 | The complete `EventType` literal — every plugin event name | 6.3 | 6.1 | 0.416 |
| walker |  | 8026 | 30 | Code::CodeKey { rung: Doc, file: beets/ui/__init__.py, decl: 23, sub: 0, line: 534 } |  |  | 0.416 |
| walker |  | 8057 | 31 | Code::CodeKey { rung: Doc, file: beets/ui/__init__.py, decl: 44, sub: 0, line: 905 } |  |  | 0.416 |
| walker |  | 8089 | 32 | Code::CodeKey { rung: Doc, file: beets/ui/__init__.py, decl: 16, sub: 0, line: 433 } |  |  | 0.416 |
| walker |  | 8125 | 36 | Code::CodeKey { rung: Doc, file: beets/ui/__init__.py, decl: 27, sub: 0, line: 637 } |  |  | 0.416 |
| walker |  | 8164 | 39 | Code::CodeKey { rung: Doc, file: beets/ui/__init__.py, decl: 10, sub: 0, line: 160 } |  |  | 0.416 |
| walker |  | 8203 | 39 | Code::CodeKey { rung: Doc, file: beets/ui/__init__.py, decl: 38, sub: 0, line: 770 } |  |  | 0.416 |
| walker |  | 8242 | 39 | Code::CodeKey { rung: Doc, file: beets/ui/__init__.py, decl: 45, sub: 0, line: 996 } |  |  | 0.416 |
| walker |  | 8293 | 51 | Code::CodeKey { rung: Doc, file: beets/ui/__init__.py, decl: 40, sub: 0, line: 807 } |  |  | 0.416 |
| ns | 8321 |  | 304 | plugins.py module-level API roster (complete) | 6.4 |  | 0.410 |
| walker |  | 8347 | 54 | Code::CodeKey { rung: Doc, file: beets/ui/__init__.py, decl: 39, sub: 0, line: 783 } |  |  | 0.410 |
| walker |  | 8402 | 55 | Code::CodeKey { rung: Doc, file: beets/ui/__init__.py, decl: 14, sub: 0, line: 383 } |  |  | 0.410 |
| walker |  | 8460 | 58 | Code::CodeKey { rung: Doc, file: beets/ui/__init__.py, decl: 34, sub: 0, line: 681 } |  |  | 0.410 |
| walker |  | 8521 | 61 | Code::CodeKey { rung: Doc, file: beets/ui/__init__.py, decl: 37, sub: 0, line: 760 } |  |  | 0.410 |
| walker |  | 8590 | 69 | Code::CodeKey { rung: Doc, file: beetsplug/bpd/__init__.py, decl: 98, sub: 0, line: 770 } |  |  | 0.410 |
| ns | 8633 |  | 312 | MetadataSourcePlugin: the metadata-backend contract | 6.5 |  | 0.405 |
| walker |  | 8662 | 72 | Code::CodeKey { rung: Doc, file: beetsplug/bpd/__init__.py, decl: 116, sub: 0, line: 985 } |  |  | 0.405 |
| walker |  | 8817 | 155 | Code::CodeKey { rung: Names, file: beetsplug/tidal/__init__.py, decl: 0, sub: 0, line: 0 } |  |  | 0.405 |
| ns | 8830 |  | 197 | Query-string prefixes: how `beet ls artist:foo` is parsed | 7.1 |  | 0.401 |
| walker |  | 8896 | 79 | Code::CodeKey { rung: Decl, file: beetsplug/tidal/__init__.py, decl: 34, sub: 0, line: 494 } |  |  | 0.401 |
| walker |  | 8969 | 73 | Code::CodeKey { rung: Doc, file: beets/ui/__init__.py, decl: 12, sub: 0, line: 187 } |  |  | 0.401 |
| walker |  | 9001 | 32 | Code::CodeKey { rung: Names, file: beetsplug/_utils/__init__.py, decl: 0, sub: 0, line: 0 } |  |  | 0.401 |
| ns | 9187 |  | 357 | dbcore/query.py class roster (complete) | 7.2 |  | 0.392 |
| walker |  | 9218 | 217 | Code::CodeKey { rung: Decl, file: beetsplug/tidal/__init__.py, decl: 2, sub: 0, line: 37 } |  |  | 0.392 |
| walker |  | 9226 | 8 | Code::CodeKey { rung: Decl, file: beetsplug/tidal/__init__.py, decl: 14, sub: 0, line: 161 } |  |  | 0.392 |
| walker |  | 9235 | 9 | Code::CodeKey { rung: Decl, file: beetsplug/tidal/__init__.py, decl: 4, sub: 0, line: 53 } |  |  | 0.392 |
| walker |  | 9266 | 31 | Code::CodeKey { rung: Decl, file: beetsplug/tidal/__init__.py, decl: 13, sub: 0, line: 142 } |  |  | 0.392 |
| walker |  | 9306 | 40 | Code::CodeKey { rung: Decl, file: beetsplug/tidal/__init__.py, decl: 12, sub: 0, line: 119 } |  |  | 0.392 |
| walker |  | 9317 | 11 | Code::CodeKey { rung: Doc, file: beetsplug/tidal/__init__.py, decl: 14, sub: 0, line: 161 } |  |  | 0.392 |
| walker |  | 9336 | 19 | Code::CodeKey { rung: Doc, file: beetsplug/tidal/__init__.py, decl: 5, sub: 0, line: 60 } |  |  | 0.392 |
| walker |  | 9350 | 14 | Code::CodeKey { rung: Body, file: beetsplug/tidal/__init__.py, decl: 11, sub: 0, line: 116 } |  |  | 0.392 |
| walker |  | 9428 | 78 | Code::CodeKey { rung: Body, file: beets/ui/commands/__init__.py, decl: 1, sub: 0, line: 36 } |  |  | 0.392 |
| walker |  | 9508 | 80 | Code::CodeKey { rung: Doc, file: beets/ui/__init__.py, decl: 22, sub: 0, line: 521 } |  |  | 0.392 |
| ns | 9539 |  | 352 | beets.autotag exports, Recommendation, Proposal | 8.1 |  | 0.386 |
| ns | 9775 |  | 236 | autotag/hooks.py and distance.py symbol roster | 8.2 |  | 0.382 |
| walker |  | 9817 | 309 | Code::CodeKey { rung: Names, file: beetsplug/discogs/__init__.py, decl: 0, sub: 0, line: 0 } |  |  | 0.382 |
| walker |  | 9873 | 56 | Code::CodeKey { rung: Decl, file: beetsplug/discogs/__init__.py, decl: 4, sub: 0, line: 60 } |  |  | 0.382 |
| walker |  | 9944 | 71 | Code::CodeKey { rung: Decl, file: beetsplug/discogs/__init__.py, decl: 6, sub: 0, line: 83 } |  |  | 0.382 |
| ns | 9970 |  | 195 | beets/util/__init__.py: classes and the core path/file-operation helpers | 9.1 |  | 0.377 |
| walker |  | 9988 | 44 | Code::CodeKey { rung: Decl, file: beetsplug/discogs/__init__.py, decl: 5, sub: 0, line: 68 } |  |  | 0.377 |
