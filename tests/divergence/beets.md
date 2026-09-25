Score(3000)=0.643 I=0.794 C=0.521 ns_rows≤3K=17/44 grid(1000/1442/2080/3000/4327/6240/9000)=0.594/0.757/0.593/0.643/0.588/0.525/0.450

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
| ns | 1227 |  | 176 | Canonical test / lint / typecheck commands | 1.10 |  | 0.590 |
| walker |  | 1339 | 205 | Markdown::Section { file: README.rst, section_index: 0, keeps_default_concavity: false } |  |  | 0.757 |
| walker |  | 1378 | 39 | Fs::DirListing { dir: .github } |  |  | 0.757 |
| walker |  | 1409 | 31 | Fs::DirListing { dir: .github/workflows } |  |  | 0.757 |
| ns | 1445 |  | 218 | test/ and docs/ top-level listings (complete) | 1.11 |  | 0.671 |
| walker |  | 1450 | 41 | Fs::DirListing { dir: docs/dev/plugins/other } |  |  | 0.671 |
| walker |  | 1721 | 271 | Code::CodeKey { rung: Names, file: beets/dbcore/__init__.py, decl: 0, sub: 0, line: 0 } |  |  | 0.674 |
| ns | 1828 |  | 383 | beetsplug/ bundled plugin listing (complete, 81 entries) | 1.12 |  | 0.553 |
| walker |  | 1873 | 152 | Code::CodeKey { rung: Names, file: beets/importer/__init__.py, decl: 0, sub: 0, line: 0 } |  |  | 0.554 |
| ns | 2006 |  | 178 | beets.library public export block (complete) | 2.1 |  | 0.535 |
| ns | 2183 |  | 177 | The `beet` subcommand roster (default_commands) | 2.2 |  | 0.515 |
| walker |  | 2256 | 383 | Fs::DirListing { dir: beetsplug } |  |  | 0.700 |
| walker |  | 2267 | 11 | Fs::DirListing { dir: beetsplug/bpd } |  |  | 0.700 |
| walker |  | 2279 | 12 | Fs::DirListing { dir: beetsplug/web } |  |  | 0.700 |
| walker |  | 2293 | 14 | Fs::DirListing { dir: beetsplug/discogs } |  |  | 0.700 |
| walker |  | 2309 | 16 | Fs::DirListing { dir: beetsplug/metasync } |  |  | 0.700 |
| walker |  | 2313 | 4 | Fs::DirListing { dir: beetsplug/web/templates } |  |  | 0.700 |
| walker |  | 2332 | 19 | Fs::DirListing { dir: beetsplug/tidal } |  |  | 0.700 |
| walker |  | 2355 | 23 | Fs::DirListing { dir: beetsplug/lastgenre } |  |  | 0.700 |
| walker |  | 2371 | 16 | Code::CodeKey { rung: ModuleDoc, file: beetsplug/web/__init__.py, decl: 0, sub: 0, line: 0 } |  |  | 0.700 |
| ns | 2373 |  | 190 | Library class: models and schema migrations | 2.3 |  | 0.684 |
| walker |  | 2388 | 17 | Code::CodeKey { rung: ModuleDoc, file: beetsplug/metasync/__init__.py, decl: 0, sub: 0, line: 0 } |  |  | 0.684 |
| walker |  | 2418 | 30 | Fs::DirListing { dir: beetsplug/_utils } |  |  | 0.684 |
| walker |  | 2457 | 39 | Code::CodeKey { rung: ModuleDoc, file: beetsplug/discogs/__init__.py, decl: 0, sub: 0, line: 0 } |  |  | 0.684 |
| walker |  | 2479 | 22 | Code::CodeKey { rung: ModuleDoc, file: beetsplug/zero.py, decl: 0, sub: 0, line: 0 } |  |  | 0.684 |
| walker |  | 2502 | 23 | Code::CodeKey { rung: ModuleDoc, file: beetsplug/subsonicupdate.py, decl: 0, sub: 0, line: 0 } |  |  | 0.684 |
| walker |  | 2526 | 24 | Code::CodeKey { rung: ModuleDoc, file: beetsplug/subsonicplaylist.py, decl: 0, sub: 0, line: 0 } |  |  | 0.684 |
| ns | 2528 |  | 155 | Library method roster (complete) | 2.4 | 2.3 | 0.671 |
| walker |  | 2550 | 24 | Code::CodeKey { rung: ModuleDoc, file: beetsplug/unimported.py, decl: 0, sub: 0, line: 0 } |  |  | 0.671 |
| walker |  | 2613 | 63 | Code::CodeKey { rung: ModuleDoc, file: beetsplug/bpd/__init__.py, decl: 0, sub: 0, line: 0 } |  |  | 0.671 |
| walker |  | 2635 | 22 | Fs::DirListing { dir: beetsplug/web/static } |  |  | 0.671 |
| walker |  | 2669 | 34 | Code::CodeKey { rung: ModuleDoc, file: beetsplug/titlecase.py, decl: 0, sub: 0, line: 0 } |  |  | 0.671 |
| walker |  | 2704 | 35 | Code::CodeKey { rung: ModuleDoc, file: beetsplug/ihate.py, decl: 0, sub: 0, line: 0 } |  |  | 0.671 |
| walker |  | 2739 | 35 | Code::CodeKey { rung: ModuleDoc, file: beetsplug/the.py, decl: 0, sub: 0, line: 0 } |  |  | 0.671 |
| walker |  | 2844 | 105 | Code::CodeKey { rung: ModuleDoc, file: beetsplug/lastgenre/__init__.py, decl: 0, sub: 0, line: 0 } |  |  | 0.671 |
| ns | 2952 |  | 424 | models.py class headers: LibModel, FormattedItemMapping, Album, Item | 2.5 |  | 0.628 |
| walker |  | 3055 | 211 | Code::CodeKey { rung: Names, file: beets/library/__init__.py, decl: 0, sub: 0, line: 0 } |  |  | 0.661 |
| walker |  | 3122 | 67 | Code::CodeKey { rung: Decl, file: beets/library/__init__.py, decl: 1, sub: 0, line: 8 } |  |  | 0.661 |
| ns | 3125 |  | 173 | beets.dbcore public export block + package docstring | 2.6 |  | 0.672 |
| walker |  | 3141 | 19 | Code::CodeKey { rung: Body, file: beets/library/__init__.py, decl: 2, sub: 0, line: 15 } |  |  | 0.672 |
| ns | 3298 |  | 173 | config_default.yaml: library, directory, plugins, ignore rules | 3.1 |  | 0.654 |
| walker |  | 3585 | 444 | Code::CodeKey { rung: Names, file: beetsplug/web/__init__.py, decl: 0, sub: 0, line: 0 } |  |  | 0.654 |
| walker |  | 3593 | 8 | Code::CodeKey { rung: Decl, file: beetsplug/web/__init__.py, decl: 32, sub: 0, line: 439 } |  |  | 0.654 |
| walker |  | 3602 | 9 | Code::CodeKey { rung: Decl, file: beetsplug/web/__init__.py, decl: 18, sub: 0, line: 296 } |  |  | 0.654 |
| walker |  | 3612 | 10 | Code::CodeKey { rung: Decl, file: beetsplug/web/__init__.py, decl: 30, sub: 0, line: 412 } |  |  | 0.654 |
| walker |  | 3622 | 10 | Code::CodeKey { rung: Decl, file: beetsplug/web/__init__.py, decl: 31, sub: 0, line: 423 } |  |  | 0.654 |
| walker |  | 3636 | 14 | Code::CodeKey { rung: Decl, file: beetsplug/web/__init__.py, decl: 23, sub: 0, line: 344 } |  |  | 0.654 |
| walker |  | 3651 | 15 | Code::CodeKey { rung: Decl, file: beetsplug/web/__init__.py, decl: 24, sub: 0, line: 354 } |  |  | 0.654 |
| walker |  | 3666 | 15 | Code::CodeKey { rung: Decl, file: beetsplug/web/__init__.py, decl: 29, sub: 0, line: 397 } |  |  | 0.654 |
| walker |  | 3682 | 16 | Code::CodeKey { rung: Decl, file: beetsplug/web/__init__.py, decl: 21, sub: 0, line: 317 } |  |  | 0.654 |
| ns | 3687 |  | 389 | config_default.yaml: the complete `import:` option block | 3.2 |  | 0.621 |
| walker |  | 3699 | 17 | Code::CodeKey { rung: Decl, file: beetsplug/web/__init__.py, decl: 28, sub: 0, line: 388 } |  |  | 0.621 |
| walker |  | 3721 | 22 | Code::CodeKey { rung: Decl, file: beetsplug/web/__init__.py, decl: 16, sub: 0, line: 283 } |  |  | 0.621 |
| walker |  | 3743 | 22 | Code::CodeKey { rung: Decl, file: beetsplug/web/__init__.py, decl: 33, sub: 0, line: 447 } |  |  | 0.621 |
| walker |  | 3771 | 28 | Code::CodeKey { rung: Decl, file: beetsplug/web/__init__.py, decl: 10, sub: 0, line: 252 } |  |  | 0.621 |
| walker |  | 3799 | 28 | Code::CodeKey { rung: Decl, file: beetsplug/web/__init__.py, decl: 13, sub: 0, line: 268 } |  |  | 0.621 |
| walker |  | 3830 | 31 | Code::CodeKey { rung: Decl, file: beetsplug/web/__init__.py, decl: 20, sub: 0, line: 310 } |  |  | 0.621 |
| walker |  | 3861 | 31 | Code::CodeKey { rung: Decl, file: beetsplug/web/__init__.py, decl: 25, sub: 0, line: 369 } |  |  | 0.621 |
| ns | 3866 |  | 179 | config_default.yaml: tagging defaults, `paths:` templates, threading | 3.4 |  | 0.608 |
| walker |  | 3892 | 31 | Code::CodeKey { rung: Decl, file: beetsplug/web/__init__.py, decl: 26, sub: 0, line: 375 } |  |  | 0.608 |
| walker |  | 3924 | 32 | Code::CodeKey { rung: Decl, file: beetsplug/web/__init__.py, decl: 27, sub: 0, line: 382 } |  |  | 0.608 |
| walker |  | 3957 | 33 | Code::CodeKey { rung: Decl, file: beetsplug/web/__init__.py, decl: 36, sub: 0, line: 520 } |  |  | 0.608 |
| walker |  | 3995 | 38 | Code::CodeKey { rung: Decl, file: beetsplug/web/__init__.py, decl: 19, sub: 0, line: 304 } |  |  | 0.608 |
| walker |  | 4034 | 39 | Code::CodeKey { rung: Decl, file: beetsplug/web/__init__.py, decl: 22, sub: 0, line: 338 } |  |  | 0.608 |
| walker |  | 4049 | 15 | Code::CodeKey { rung: Doc, file: beetsplug/web/__init__.py, decl: 5, sub: 0, line: 117 } |  |  | 0.608 |
| walker |  | 4066 | 17 | Code::CodeKey { rung: Doc, file: beetsplug/web/__init__.py, decl: 3, sub: 0, line: 103 } |  |  | 0.608 |
| walker |  | 4083 | 17 | Code::CodeKey { rung: Doc, file: beetsplug/web/__init__.py, decl: 10, sub: 0, line: 252 } |  |  | 0.608 |
| walker |  | 4101 | 18 | Code::CodeKey { rung: Doc, file: beetsplug/web/__init__.py, decl: 9, sub: 0, line: 241 } |  |  | 0.608 |
| walker |  | 4119 | 18 | Code::CodeKey { rung: Doc, file: beetsplug/web/__init__.py, decl: 13, sub: 0, line: 268 } |  |  | 0.608 |
| walker |  | 4138 | 19 | Code::CodeKey { rung: Doc, file: beetsplug/web/__init__.py, decl: 7, sub: 0, line: 176 } |  |  | 0.608 |
| walker |  | 4158 | 20 | Code::CodeKey { rung: Doc, file: beetsplug/web/__init__.py, decl: 6, sub: 0, line: 122 } |  |  | 0.608 |
| ns | 4171 |  | 305 | Path-template function roster: DefaultTemplateFunctions (complete) | 3.5 |  | 0.594 |
| walker |  | 4188 | 30 | Code::CodeKey { rung: Doc, file: beetsplug/web/__init__.py, decl: 4, sub: 0, line: 109 } |  |  | 0.594 |
| walker |  | 4222 | 34 | Code::CodeKey { rung: Doc, file: beetsplug/web/__init__.py, decl: 8, sub: 0, line: 223 } |  |  | 0.594 |
| walker |  | 4278 | 56 | Code::CodeKey { rung: Doc, file: beetsplug/web/__init__.py, decl: 1, sub: 0, line: 46 } |  |  | 0.594 |
| ns | 4291 |  | 120 | config_default.yaml: display formats and default sorts | 3.6 |  | 0.588 |
| walker |  | 4489 | 211 | Code::CodeKey { rung: Names, file: beets/ui/commands/__init__.py, decl: 0, sub: 0, line: 0 } |  |  | 0.588 |
| walker |  | 4499 | 10 | Code::CodeKey { rung: Doc, file: beets/ui/commands/__init__.py, decl: 1, sub: 0, line: 36 } |  |  | 0.588 |
| walker |  | 4606 | 107 | Code::CodeKey { rung: Decl, file: beets/ui/commands/__init__.py, decl: 2, sub: 0, line: 50 } |  |  | 0.603 |
| ns | 4673 |  | 382 | config_default.yaml: autotagger thresholds and distance weights | 3.7 |  | 0.580 |
| ns | 4813 |  | 140 | config_default.yaml: terminal UI, colour names, import layout | 3.8 |  | 0.570 |
| walker |  | 4863 | 257 | Code::CodeKey { rung: Names, file: beetsplug/bpd/__init__.py, decl: 0, sub: 0, line: 0 } |  |  | 0.570 |
| ns | 4986 |  | 173 | Album method roster (complete, names only) | 4.1 | 2.5 | 0.561 |
| walker |  | 5053 | 190 | Code::CodeKey { rung: Names, file: beets/autotag/__init__.py, decl: 0, sub: 0, line: 0 } |  |  | 0.561 |
| ns | 5290 |  | 304 | Item method roster (complete, names only) | 4.3 | 2.5 | 0.547 |
| walker |  | 5446 | 393 | Code::CodeKey { rung: Names, file: beetsplug/bpd/__init__.py, decl: 0, sub: 1, line: 0 } |  |  | 0.547 |
| ns | 5458 |  | 168 | Item search fields and MediaFile-backed field sets | 4.4 | 2.5 | 0.540 |
| walker |  | 5462 | 16 | Code::CodeKey { rung: Decl, file: beetsplug/bpd/__init__.py, decl: 36, sub: 0, line: 180 } |  |  | 0.540 |
| walker |  | 5496 | 34 | Code::CodeKey { rung: Decl, file: beetsplug/bpd/__init__.py, decl: 118, sub: 0, line: 1064 } |  |  | 0.540 |
| walker |  | 5549 | 53 | Code::CodeKey { rung: Decl, file: beetsplug/bpd/__init__.py, decl: 168, sub: 0, line: 1583 } |  |  | 0.540 |
| walker |  | 5610 | 61 | Code::CodeKey { rung: Decl, file: beetsplug/bpd/__init__.py, decl: 25, sub: 0, line: 80 } |  |  | 0.540 |
| ns | 5629 |  | 171 | LibModel shared-method roster (complete after `_fields`) | 4.5 | 2.5 | 0.533 |
| walker |  | 5672 | 62 | Code::CodeKey { rung: Decl, file: beetsplug/bpd/__init__.py, decl: 27, sub: 0, line: 110 } |  |  | 0.533 |
| walker |  | 5743 | 71 | Code::CodeKey { rung: Decl, file: beetsplug/bpd/__init__.py, decl: 94, sub: 0, line: 754 } |  |  | 0.533 |
| walker |  | 5751 | 8 | Code::CodeKey { rung: Decl, file: beetsplug/bpd/__init__.py, decl: 99, sub: 0, line: 785 } |  |  | 0.533 |
| walker |  | 5764 | 13 | Code::CodeKey { rung: Doc, file: beetsplug/bpd/__init__.py, decl: 94, sub: 0, line: 754 } |  |  | 0.533 |
| walker |  | 5777 | 13 | Code::CodeKey { rung: Doc, file: beetsplug/bpd/__init__.py, decl: 170, sub: 0, line: 1601 } |  |  | 0.533 |
| ns | 5787 |  | 158 | beets.importer public exports + package docstring | 5.1 |  | 0.536 |
| walker |  | 5860 | 83 | Code::CodeKey { rung: Decl, file: beetsplug/bpd/__init__.py, decl: 100, sub: 0, line: 794 } |  |  | 0.536 |
| walker |  | 5874 | 14 | Code::CodeKey { rung: Doc, file: beetsplug/bpd/__init__.py, decl: 96, sub: 0, line: 763 } |  |  | 0.536 |
| walker |  | 5888 | 14 | Code::CodeKey { rung: Doc, file: beetsplug/bpd/__init__.py, decl: 103, sub: 0, line: 816 } |  |  | 0.536 |
| walker |  | 5902 | 14 | Code::CodeKey { rung: Doc, file: beetsplug/bpd/__init__.py, decl: 105, sub: 0, line: 825 } |  |  | 0.536 |
| walker |  | 5991 | 89 | Code::CodeKey { rung: Decl, file: beetsplug/bpd/__init__.py, decl: 107, sub: 0, line: 905 } |  |  | 0.536 |
| walker |  | 6002 | 11 | Code::CodeKey { rung: Doc, file: beetsplug/bpd/__init__.py, decl: 112, sub: 0, line: 943 } |  |  | 0.536 |
| walker |  | 6016 | 14 | Code::CodeKey { rung: Doc, file: beetsplug/bpd/__init__.py, decl: 113, sub: 0, line: 950 } |  |  | 0.536 |
| walker |  | 6032 | 16 | Code::CodeKey { rung: Doc, file: beetsplug/bpd/__init__.py, decl: 100, sub: 0, line: 794 } |  |  | 0.536 |
| walker |  | 6048 | 16 | Code::CodeKey { rung: Doc, file: beetsplug/bpd/__init__.py, decl: 104, sub: 0, line: 821 } |  |  | 0.536 |
| walker |  | 6064 | 16 | Code::CodeKey { rung: Doc, file: beetsplug/bpd/__init__.py, decl: 111, sub: 0, line: 939 } |  |  | 0.536 |
| walker |  | 6080 | 16 | Code::CodeKey { rung: Doc, file: beetsplug/bpd/__init__.py, decl: 120, sub: 0, line: 1079 } |  |  | 0.536 |
| ns | 6147 |  | 360 | importer/stages.py: complete stage-function roster with section banners | 5.2 |  | 0.525 |
| walker |  | 6183 | 103 | Code::CodeKey { rung: Decl, file: beetsplug/bpd/__init__.py, decl: 114, sub: 0, line: 960 } |  |  | 0.525 |
| walker |  | 6199 | 16 | Code::CodeKey { rung: Doc, file: beetsplug/bpd/__init__.py, decl: 114, sub: 0, line: 960 } |  |  | 0.525 |
| walker |  | 6216 | 17 | Code::CodeKey { rung: Doc, file: beetsplug/bpd/__init__.py, decl: 107, sub: 0, line: 905 } |  |  | 0.525 |
| walker |  | 6234 | 18 | Code::CodeKey { rung: Doc, file: beetsplug/bpd/__init__.py, decl: 95, sub: 0, line: 757 } |  |  | 0.525 |
| walker |  | 6252 | 18 | Code::CodeKey { rung: Doc, file: beetsplug/bpd/__init__.py, decl: 101, sub: 0, line: 797 } |  |  | 0.525 |
| walker |  | 6270 | 18 | Code::CodeKey { rung: Doc, file: beetsplug/bpd/__init__.py, decl: 108, sub: 0, line: 908 } |  |  | 0.525 |
| walker |  | 6385 | 115 | Code::CodeKey { rung: Decl, file: beetsplug/bpd/__init__.py, decl: 26, sub: 0, line: 90 } |  |  | 0.525 |
| walker |  | 6404 | 19 | Code::CodeKey { rung: Doc, file: beetsplug/bpd/__init__.py, decl: 30, sub: 0, line: 138 } |  |  | 0.525 |
| walker |  | 6423 | 19 | Code::CodeKey { rung: Doc, file: beetsplug/bpd/__init__.py, decl: 110, sub: 0, line: 915 } |  |  | 0.525 |
| walker |  | 6448 | 25 | Code::CodeKey { rung: Doc, file: beetsplug/bpd/__init__.py, decl: 102, sub: 0, line: 804 } |  |  | 0.525 |
| walker |  | 6478 | 30 | Code::CodeKey { rung: Doc, file: beetsplug/bpd/__init__.py, decl: 35, sub: 0, line: 174 } |  |  | 0.525 |
| walker |  | 6509 | 31 | Code::CodeKey { rung: Doc, file: beetsplug/bpd/__init__.py, decl: 27, sub: 0, line: 110 } |  |  | 0.525 |
| walker |  | 6541 | 32 | Code::CodeKey { rung: Doc, file: beetsplug/bpd/__init__.py, decl: 106, sub: 0, line: 838 } |  |  | 0.525 |
| ns | 6572 |  | 425 | ImportSession.run(): how the pipeline is assembled | 5.3 | 5.2 | 0.510 |
| walker |  | 6573 | 32 | Code::CodeKey { rung: Doc, file: beetsplug/bpd/__init__.py, decl: 117, sub: 0, line: 1017 } |  |  | 0.510 |
| walker |  | 6607 | 34 | Code::CodeKey { rung: Doc, file: beetsplug/bpd/__init__.py, decl: 29, sub: 0, line: 123 } |  |  | 0.510 |
| walker |  | 6642 | 35 | Code::CodeKey { rung: Doc, file: beetsplug/bpd/__init__.py, decl: 168, sub: 0, line: 1583 } |  |  | 0.510 |
| walker |  | 6682 | 40 | Code::CodeKey { rung: Doc, file: beetsplug/bpd/__init__.py, decl: 36, sub: 0, line: 180 } |  |  | 0.510 |
| walker |  | 6723 | 41 | Code::CodeKey { rung: Doc, file: beetsplug/bpd/__init__.py, decl: 115, sub: 0, line: 966 } |  |  | 0.510 |
| walker |  | 6771 | 48 | Code::CodeKey { rung: Doc, file: beetsplug/bpd/__init__.py, decl: 119, sub: 0, line: 1070 } |  |  | 0.510 |
| ns | 6829 |  | 257 | ImportSession class contract (attributes + the four decision hooks) | 5.4 | 5.3 | 0.501 |
| walker |  | 7029 | 258 | Code::CodeKey { rung: Names, file: beets/ui/__init__.py, decl: 0, sub: 0, line: 0 } |  |  | 0.501 |
| walker |  | 7056 | 27 | Code::CodeKey { rung: Decl, file: beets/ui/__init__.py, decl: 2, sub: 0, line: 65 } |  |  | 0.501 |
| walker |  | 7068 | 12 | Code::CodeKey { rung: Body, file: beets/ui/__init__.py, decl: 4, sub: 0, line: 80 } |  |  | 0.501 |
| walker |  | 7080 | 12 | Code::CodeKey { rung: Body, file: beets/ui/__init__.py, decl: 5, sub: 0, line: 85 } |  |  | 0.501 |
| walker |  | 7099 | 19 | Code::CodeKey { rung: Doc, file: beets/ui/__init__.py, decl: 4, sub: 0, line: 80 } |  |  | 0.501 |
| ns | 7103 |  | 274 | importer/tasks.py: Action enum and the complete task class roster | 5.5 |  | 0.492 |
| walker |  | 7118 | 19 | Code::CodeKey { rung: Doc, file: beets/ui/__init__.py, decl: 5, sub: 0, line: 85 } |  |  | 0.492 |
| walker |  | 7155 | 37 | Code::CodeKey { rung: Doc, file: beets/ui/__init__.py, decl: 3, sub: 0, line: 71 } |  |  | 0.492 |
| walker |  | 7212 | 57 | Code::CodeKey { rung: Doc, file: beets/ui/__init__.py, decl: 6, sub: 0, line: 90 } |  |  | 0.492 |
| walker |  | 7278 | 66 | Code::CodeKey { rung: Doc, file: beetsplug/bpd/__init__.py, decl: 118, sub: 0, line: 1064 } |  |  | 0.492 |
| ns | 7349 |  | 246 | BeetsPlugin base class: docstring and declared attributes | 6.1 |  | 0.485 |
| walker |  | 7464 | 186 | Code::CodeKey { rung: Names, file: beetsplug/metasync/__init__.py, decl: 0, sub: 0, line: 0 } |  |  | 0.485 |
| walker |  | 7493 | 29 | Code::CodeKey { rung: Decl, file: beetsplug/metasync/__init__.py, decl: 2, sub: 0, line: 34 } |  |  | 0.485 |
| walker |  | 7545 | 52 | Code::CodeKey { rung: Decl, file: beetsplug/metasync/__init__.py, decl: 3, sub: 0, line: 40 } |  |  | 0.485 |
| walker |  | 7554 | 9 | Code::CodeKey { rung: Decl, file: beetsplug/metasync/__init__.py, decl: 5, sub: 0, line: 47 } |  |  | 0.485 |
| walker |  | 7559 | 5 | Code::CodeKey { rung: Body, file: beetsplug/metasync/__init__.py, decl: 5, sub: 0, line: 47 } |  |  | 0.485 |
| ns | 7595 |  | 246 | BeetsPlugin method roster (complete) | 6.2 | 6.1 | 0.480 |
| walker |  | 7615 | 56 | Code::CodeKey { rung: Decl, file: beetsplug/metasync/__init__.py, decl: 9, sub: 0, line: 76 } |  |  | 0.480 |
| walker |  | 7629 | 14 | Code::CodeKey { rung: Doc, file: beetsplug/metasync/__init__.py, decl: 12, sub: 0, line: 104 } |  |  | 0.480 |
| walker |  | 7648 | 19 | Code::CodeKey { rung: Doc, file: beetsplug/metasync/__init__.py, decl: 8, sub: 0, line: 68 } |  |  | 0.480 |
| walker |  | 7655 | 7 | Code::CodeKey { rung: Body, file: beetsplug/metasync/__init__.py, decl: 10, sub: 0, line: 79 } |  |  | 0.480 |
| walker |  | 7700 | 45 | Code::CodeKey { rung: Doc, file: beetsplug/metasync/__init__.py, decl: 6, sub: 0, line: 52 } |  |  | 0.480 |
| walker |  | 7772 | 72 | Code::CodeKey { rung: Names, file: beets/__init__.py, decl: 0, sub: 0, line: 0 } |  |  | 0.480 |
| walker |  | 7798 | 26 | Code::CodeKey { rung: Decl, file: beets/__init__.py, decl: 2, sub: 0, line: 35 } |  |  | 0.480 |
| walker |  | 7808 | 10 | Code::CodeKey { rung: Doc, file: beets/__init__.py, decl: 1, sub: 0, line: 26 } |  |  | 0.480 |
| walker |  | 7848 | 40 | Code::CodeKey { rung: Doc, file: beets/__init__.py, decl: 2, sub: 0, line: 35 } |  |  | 0.480 |
| walker |  | 7905 | 57 | Code::CodeKey { rung: Body, file: beets/__init__.py, decl: 1, sub: 0, line: 26 } |  |  | 0.480 |
| ns | 8017 |  | 422 | The complete `EventType` literal — every plugin event name | 6.3 | 6.1 | 0.467 |
| walker |  | 8190 | 285 | Code::CodeKey { rung: Names, file: beets/ui/__init__.py, decl: 0, sub: 1, line: 0 } |  |  | 0.467 |
| walker |  | 8197 | 7 | Code::CodeKey { rung: Decl, file: beets/ui/__init__.py, decl: 18, sub: 0, line: 459 } |  |  | 0.467 |
| walker |  | 8223 | 26 | Code::CodeKey { rung: Decl, file: beets/ui/__init__.py, decl: 40, sub: 0, line: 807 } |  |  | 0.467 |
| walker |  | 8287 | 64 | Code::CodeKey { rung: Decl, file: beets/ui/__init__.py, decl: 13, sub: 0, line: 207 } |  |  | 0.467 |
| ns | 8321 |  | 304 | plugins.py module-level API roster (complete) | 6.4 |  | 0.461 |
| walker |  | 8359 | 72 | Code::CodeKey { rung: Decl, file: beets/ui/__init__.py, decl: 19, sub: 0, line: 466 } |  |  | 0.461 |
| walker |  | 8374 | 15 | Code::CodeKey { rung: Doc, file: beets/ui/__init__.py, decl: 43, sub: 0, line: 879 } |  |  | 0.461 |
| walker |  | 8390 | 16 | Code::CodeKey { rung: Doc, file: beets/ui/__init__.py, decl: 17, sub: 0, line: 445 } |  |  | 0.461 |
| walker |  | 8406 | 16 | Code::CodeKey { rung: Doc, file: beets/ui/__init__.py, decl: 18, sub: 0, line: 459 } |  |  | 0.461 |
| walker |  | 8504 | 98 | Code::CodeKey { rung: Decl, file: beets/ui/__init__.py, decl: 33, sub: 0, line: 676 } |  |  | 0.461 |
| walker |  | 8521 | 17 | Code::CodeKey { rung: Doc, file: beets/ui/__init__.py, decl: 41, sub: 0, line: 830 } |  |  | 0.461 |
| walker |  | 8626 | 105 | Code::CodeKey { rung: Decl, file: beets/ui/__init__.py, decl: 27, sub: 0, line: 637 } |  |  | 0.461 |
| ns | 8633 |  | 312 | MetadataSourcePlugin: the metadata-backend contract | 6.5 |  | 0.455 |
| walker |  | 8634 | 8 | Code::CodeKey { rung: Decl, file: beets/ui/__init__.py, decl: 31, sub: 0, line: 664 } |  |  | 0.455 |
| walker |  | 8644 | 10 | Code::CodeKey { rung: Decl, file: beets/ui/__init__.py, decl: 32, sub: 0, line: 668 } |  |  | 0.455 |
| walker |  | 8757 | 113 | Code::CodeKey { rung: Decl, file: beets/ui/__init__.py, decl: 20, sub: 0, line: 498 } |  |  | 0.455 |
| walker |  | 8824 | 67 | Code::CodeKey { rung: Decl, file: beets/ui/__init__.py, decl: 23, sub: 0, line: 534 } |  |  | 0.455 |
| ns | 8830 |  | 197 | Query-string prefixes: how `beet ls artist:foo` is parsed | 7.1 |  | 0.450 |
| walker |  | 8838 | 14 | Code::CodeKey { rung: Doc, file: beets/ui/__init__.py, decl: 26, sub: 0, line: 621 } |  |  | 0.450 |
| walker |  | 8857 | 19 | Code::CodeKey { rung: Doc, file: beets/ui/__init__.py, decl: 35, sub: 0, line: 701 } |  |  | 0.450 |
| walker |  | 8877 | 20 | Code::CodeKey { rung: Doc, file: beets/ui/__init__.py, decl: 9, sub: 0, line: 150 } |  |  | 0.450 |
| walker |  | 8906 | 29 | Code::CodeKey { rung: Doc, file: beets/ui/__init__.py, decl: 33, sub: 0, line: 676 } |  |  | 0.450 |
| walker |  | 8936 | 30 | Code::CodeKey { rung: Doc, file: beets/ui/__init__.py, decl: 23, sub: 0, line: 534 } |  |  | 0.450 |
| walker |  | 8967 | 31 | Code::CodeKey { rung: Doc, file: beets/ui/__init__.py, decl: 44, sub: 0, line: 905 } |  |  | 0.450 |
| walker |  | 8999 | 32 | Code::CodeKey { rung: Doc, file: beets/ui/__init__.py, decl: 16, sub: 0, line: 433 } |  |  | 0.450 |
| walker |  | 9035 | 36 | Code::CodeKey { rung: Doc, file: beets/ui/__init__.py, decl: 27, sub: 0, line: 637 } |  |  | 0.450 |
| walker |  | 9074 | 39 | Code::CodeKey { rung: Doc, file: beets/ui/__init__.py, decl: 10, sub: 0, line: 160 } |  |  | 0.450 |
| walker |  | 9113 | 39 | Code::CodeKey { rung: Doc, file: beets/ui/__init__.py, decl: 38, sub: 0, line: 770 } |  |  | 0.450 |
| walker |  | 9152 | 39 | Code::CodeKey { rung: Doc, file: beets/ui/__init__.py, decl: 45, sub: 0, line: 996 } |  |  | 0.450 |
| ns | 9187 |  | 357 | dbcore/query.py class roster (complete) | 7.2 |  | 0.441 |
| walker |  | 9203 | 51 | Code::CodeKey { rung: Doc, file: beets/ui/__init__.py, decl: 40, sub: 0, line: 807 } |  |  | 0.441 |
| walker |  | 9257 | 54 | Code::CodeKey { rung: Doc, file: beets/ui/__init__.py, decl: 39, sub: 0, line: 783 } |  |  | 0.441 |
| walker |  | 9312 | 55 | Code::CodeKey { rung: Doc, file: beets/ui/__init__.py, decl: 14, sub: 0, line: 383 } |  |  | 0.441 |
| walker |  | 9370 | 58 | Code::CodeKey { rung: Doc, file: beets/ui/__init__.py, decl: 34, sub: 0, line: 681 } |  |  | 0.441 |
| walker |  | 9431 | 61 | Code::CodeKey { rung: Doc, file: beets/ui/__init__.py, decl: 37, sub: 0, line: 760 } |  |  | 0.441 |
| walker |  | 9500 | 69 | Code::CodeKey { rung: Doc, file: beetsplug/bpd/__init__.py, decl: 98, sub: 0, line: 770 } |  |  | 0.441 |
| ns | 9539 |  | 352 | beets.autotag exports, Recommendation, Proposal | 8.1 |  | 0.434 |
| walker |  | 9572 | 72 | Code::CodeKey { rung: Doc, file: beetsplug/bpd/__init__.py, decl: 116, sub: 0, line: 985 } |  |  | 0.434 |
| walker |  | 9727 | 155 | Code::CodeKey { rung: Names, file: beetsplug/tidal/__init__.py, decl: 0, sub: 0, line: 0 } |  |  | 0.434 |
| ns | 9775 |  | 236 | autotag/hooks.py and distance.py symbol roster | 8.2 |  | 0.429 |
| walker |  | 9806 | 79 | Code::CodeKey { rung: Decl, file: beetsplug/tidal/__init__.py, decl: 34, sub: 0, line: 494 } |  |  | 0.429 |
| walker |  | 9879 | 73 | Code::CodeKey { rung: Doc, file: beets/ui/__init__.py, decl: 12, sub: 0, line: 187 } |  |  | 0.429 |
| walker |  | 9911 | 32 | Code::CodeKey { rung: Names, file: beetsplug/_utils/__init__.py, decl: 0, sub: 0, line: 0 } |  |  | 0.429 |
| ns | 9970 |  | 195 | beets/util/__init__.py: classes and the core path/file-operation helpers | 9.1 |  | 0.424 |
| walker |  | 9988 | 77 | Code::CodeKey { rung: Decl, file: beetsplug/tidal/__init__.py, decl: 2, sub: 0, line: 37 } |  |  | 0.424 |
