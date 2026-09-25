Score(3000)=0.634 I=0.788 C=0.511 ns_rows≤3K=17/44 grid(1000/1442/2080/3000/4327/6240/9000)=0.594/0.599/0.579/0.634/0.556/0.475/0.407

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
| walker |  | 2297 | 81 | Toml::PackageMetadata { file: pyproject.toml } |  |  | 0.569 |
| ns | 2373 |  | 190 | Library class: models and schema migrations | 2.3 |  | 0.556 |
| walker |  | 2502 | 205 | Markdown::Section { file: README.rst, section_index: 0, keeps_default_concavity: false } |  |  | 0.691 |
| ns | 2528 |  | 155 | Library method roster (complete) | 2.4 | 2.3 | 0.678 |
| walker |  | 2946 | 444 | Code::CodeKey { rung: Names, file: beetsplug/web/__init__.py, decl: 0, sub: 0, line: 0 } |  |  | 0.678 |
| ns | 2952 |  | 424 | models.py class headers: LibModel, FormattedItemMapping, Album, Item | 2.5 |  | 0.634 |
| walker |  | 2954 | 8 | Code::CodeKey { rung: Decl, file: beetsplug/web/__init__.py, decl: 32, sub: 0, line: 439 } |  |  | 0.634 |
| walker |  | 2963 | 9 | Code::CodeKey { rung: Decl, file: beetsplug/web/__init__.py, decl: 18, sub: 0, line: 296 } |  |  | 0.634 |
| walker |  | 2973 | 10 | Code::CodeKey { rung: Decl, file: beetsplug/web/__init__.py, decl: 30, sub: 0, line: 412 } |  |  | 0.634 |
| walker |  | 2983 | 10 | Code::CodeKey { rung: Decl, file: beetsplug/web/__init__.py, decl: 31, sub: 0, line: 423 } |  |  | 0.634 |
| walker |  | 2997 | 14 | Code::CodeKey { rung: Decl, file: beetsplug/web/__init__.py, decl: 23, sub: 0, line: 344 } |  |  | 0.634 |
| walker |  | 3012 | 15 | Code::CodeKey { rung: Decl, file: beetsplug/web/__init__.py, decl: 24, sub: 0, line: 354 } |  |  | 0.634 |
| walker |  | 3027 | 15 | Code::CodeKey { rung: Decl, file: beetsplug/web/__init__.py, decl: 29, sub: 0, line: 397 } |  |  | 0.634 |
| walker |  | 3043 | 16 | Code::CodeKey { rung: Decl, file: beetsplug/web/__init__.py, decl: 21, sub: 0, line: 317 } |  |  | 0.634 |
| walker |  | 3060 | 17 | Code::CodeKey { rung: Decl, file: beetsplug/web/__init__.py, decl: 28, sub: 0, line: 388 } |  |  | 0.634 |
| walker |  | 3082 | 22 | Code::CodeKey { rung: Decl, file: beetsplug/web/__init__.py, decl: 16, sub: 0, line: 283 } |  |  | 0.634 |
| walker |  | 3104 | 22 | Code::CodeKey { rung: Decl, file: beetsplug/web/__init__.py, decl: 33, sub: 0, line: 447 } |  |  | 0.634 |
| ns | 3125 |  | 173 | beets.dbcore public export block + package docstring | 2.6 |  | 0.617 |
| walker |  | 3132 | 28 | Code::CodeKey { rung: Decl, file: beetsplug/web/__init__.py, decl: 10, sub: 0, line: 252 } |  |  | 0.617 |
| walker |  | 3160 | 28 | Code::CodeKey { rung: Decl, file: beetsplug/web/__init__.py, decl: 13, sub: 0, line: 268 } |  |  | 0.617 |
| walker |  | 3191 | 31 | Code::CodeKey { rung: Decl, file: beetsplug/web/__init__.py, decl: 20, sub: 0, line: 310 } |  |  | 0.617 |
| walker |  | 3222 | 31 | Code::CodeKey { rung: Decl, file: beetsplug/web/__init__.py, decl: 25, sub: 0, line: 369 } |  |  | 0.617 |
| walker |  | 3253 | 31 | Code::CodeKey { rung: Decl, file: beetsplug/web/__init__.py, decl: 26, sub: 0, line: 375 } |  |  | 0.617 |
| walker |  | 3285 | 32 | Code::CodeKey { rung: Decl, file: beetsplug/web/__init__.py, decl: 27, sub: 0, line: 382 } |  |  | 0.617 |
| ns | 3298 |  | 173 | config_default.yaml: library, directory, plugins, ignore rules | 3.1 |  | 0.601 |
| walker |  | 3318 | 33 | Code::CodeKey { rung: Decl, file: beetsplug/web/__init__.py, decl: 36, sub: 0, line: 520 } |  |  | 0.601 |
| walker |  | 3356 | 38 | Code::CodeKey { rung: Decl, file: beetsplug/web/__init__.py, decl: 19, sub: 0, line: 304 } |  |  | 0.601 |
| walker |  | 3395 | 39 | Code::CodeKey { rung: Decl, file: beetsplug/web/__init__.py, decl: 22, sub: 0, line: 338 } |  |  | 0.601 |
| walker |  | 3410 | 15 | Code::CodeKey { rung: Doc, file: beetsplug/web/__init__.py, decl: 5, sub: 0, line: 117 } |  |  | 0.601 |
| walker |  | 3427 | 17 | Code::CodeKey { rung: Doc, file: beetsplug/web/__init__.py, decl: 3, sub: 0, line: 103 } |  |  | 0.601 |
| walker |  | 3444 | 17 | Code::CodeKey { rung: Doc, file: beetsplug/web/__init__.py, decl: 10, sub: 0, line: 252 } |  |  | 0.601 |
| walker |  | 3462 | 18 | Code::CodeKey { rung: Doc, file: beetsplug/web/__init__.py, decl: 9, sub: 0, line: 241 } |  |  | 0.601 |
| walker |  | 3480 | 18 | Code::CodeKey { rung: Doc, file: beetsplug/web/__init__.py, decl: 13, sub: 0, line: 268 } |  |  | 0.601 |
| walker |  | 3499 | 19 | Code::CodeKey { rung: Doc, file: beetsplug/web/__init__.py, decl: 7, sub: 0, line: 176 } |  |  | 0.601 |
| walker |  | 3519 | 20 | Code::CodeKey { rung: Doc, file: beetsplug/web/__init__.py, decl: 6, sub: 0, line: 122 } |  |  | 0.601 |
| walker |  | 3549 | 30 | Code::CodeKey { rung: Doc, file: beetsplug/web/__init__.py, decl: 4, sub: 0, line: 109 } |  |  | 0.601 |
| walker |  | 3583 | 34 | Code::CodeKey { rung: Doc, file: beetsplug/web/__init__.py, decl: 8, sub: 0, line: 223 } |  |  | 0.601 |
| walker |  | 3639 | 56 | Code::CodeKey { rung: Doc, file: beetsplug/web/__init__.py, decl: 1, sub: 0, line: 46 } |  |  | 0.601 |
| ns | 3687 |  | 389 | config_default.yaml: the complete `import:` option block | 3.2 |  | 0.571 |
| walker |  | 3850 | 211 | Code::CodeKey { rung: Names, file: beets/ui/commands/__init__.py, decl: 0, sub: 0, line: 0 } |  |  | 0.571 |
| walker |  | 3860 | 10 | Code::CodeKey { rung: Doc, file: beets/ui/commands/__init__.py, decl: 1, sub: 0, line: 36 } |  |  | 0.571 |
| ns | 3866 |  | 179 | config_default.yaml: tagging defaults, `paths:` templates, threading | 3.4 |  | 0.559 |
| walker |  | 3967 | 107 | Code::CodeKey { rung: Decl, file: beets/ui/commands/__init__.py, decl: 2, sub: 0, line: 50 } |  |  | 0.575 |
| ns | 4171 |  | 305 | Path-template function roster: DefaultTemplateFunctions (complete) | 3.5 |  | 0.563 |
| walker |  | 4224 | 257 | Code::CodeKey { rung: Names, file: beetsplug/bpd/__init__.py, decl: 0, sub: 0, line: 0 } |  |  | 0.563 |
| ns | 4291 |  | 120 | config_default.yaml: display formats and default sorts | 3.6 |  | 0.556 |
| walker |  | 4617 | 393 | Code::CodeKey { rung: Names, file: beetsplug/bpd/__init__.py, decl: 0, sub: 1, line: 0 } |  |  | 0.556 |
| walker |  | 4633 | 16 | Code::CodeKey { rung: Decl, file: beetsplug/bpd/__init__.py, decl: 36, sub: 0, line: 180 } |  |  | 0.556 |
| walker |  | 4667 | 34 | Code::CodeKey { rung: Decl, file: beetsplug/bpd/__init__.py, decl: 118, sub: 0, line: 1064 } |  |  | 0.556 |
| ns | 4673 |  | 382 | config_default.yaml: autotagger thresholds and distance weights | 3.7 |  | 0.535 |
| walker |  | 4720 | 53 | Code::CodeKey { rung: Decl, file: beetsplug/bpd/__init__.py, decl: 168, sub: 0, line: 1583 } |  |  | 0.535 |
| walker |  | 4781 | 61 | Code::CodeKey { rung: Decl, file: beetsplug/bpd/__init__.py, decl: 25, sub: 0, line: 80 } |  |  | 0.535 |
| ns | 4813 |  | 140 | config_default.yaml: terminal UI, colour names, import layout | 3.8 |  | 0.526 |
| walker |  | 4843 | 62 | Code::CodeKey { rung: Decl, file: beetsplug/bpd/__init__.py, decl: 27, sub: 0, line: 110 } |  |  | 0.526 |
| walker |  | 4914 | 71 | Code::CodeKey { rung: Decl, file: beetsplug/bpd/__init__.py, decl: 94, sub: 0, line: 754 } |  |  | 0.526 |
| walker |  | 4922 | 8 | Code::CodeKey { rung: Decl, file: beetsplug/bpd/__init__.py, decl: 99, sub: 0, line: 785 } |  |  | 0.526 |
| walker |  | 4935 | 13 | Code::CodeKey { rung: Doc, file: beetsplug/bpd/__init__.py, decl: 94, sub: 0, line: 754 } |  |  | 0.526 |
| walker |  | 4948 | 13 | Code::CodeKey { rung: Doc, file: beetsplug/bpd/__init__.py, decl: 170, sub: 0, line: 1601 } |  |  | 0.526 |
| ns | 4986 |  | 173 | Album method roster (complete, names only) | 4.1 | 2.5 | 0.518 |
| walker |  | 5031 | 83 | Code::CodeKey { rung: Decl, file: beetsplug/bpd/__init__.py, decl: 100, sub: 0, line: 794 } |  |  | 0.518 |
| walker |  | 5045 | 14 | Code::CodeKey { rung: Doc, file: beetsplug/bpd/__init__.py, decl: 96, sub: 0, line: 763 } |  |  | 0.518 |
| walker |  | 5059 | 14 | Code::CodeKey { rung: Doc, file: beetsplug/bpd/__init__.py, decl: 103, sub: 0, line: 816 } |  |  | 0.518 |
| walker |  | 5073 | 14 | Code::CodeKey { rung: Doc, file: beetsplug/bpd/__init__.py, decl: 105, sub: 0, line: 825 } |  |  | 0.518 |
| walker |  | 5162 | 89 | Code::CodeKey { rung: Decl, file: beetsplug/bpd/__init__.py, decl: 107, sub: 0, line: 905 } |  |  | 0.518 |
| walker |  | 5173 | 11 | Code::CodeKey { rung: Doc, file: beetsplug/bpd/__init__.py, decl: 112, sub: 0, line: 943 } |  |  | 0.518 |
| walker |  | 5187 | 14 | Code::CodeKey { rung: Doc, file: beetsplug/bpd/__init__.py, decl: 113, sub: 0, line: 950 } |  |  | 0.518 |
| walker |  | 5203 | 16 | Code::CodeKey { rung: Doc, file: beetsplug/bpd/__init__.py, decl: 100, sub: 0, line: 794 } |  |  | 0.518 |
| walker |  | 5219 | 16 | Code::CodeKey { rung: Doc, file: beetsplug/bpd/__init__.py, decl: 104, sub: 0, line: 821 } |  |  | 0.518 |
| walker |  | 5235 | 16 | Code::CodeKey { rung: Doc, file: beetsplug/bpd/__init__.py, decl: 111, sub: 0, line: 939 } |  |  | 0.518 |
| walker |  | 5251 | 16 | Code::CodeKey { rung: Doc, file: beetsplug/bpd/__init__.py, decl: 120, sub: 0, line: 1079 } |  |  | 0.518 |
| ns | 5290 |  | 304 | Item method roster (complete, names only) | 4.3 | 2.5 | 0.504 |
| walker |  | 5354 | 103 | Code::CodeKey { rung: Decl, file: beetsplug/bpd/__init__.py, decl: 114, sub: 0, line: 960 } |  |  | 0.504 |
| walker |  | 5370 | 16 | Code::CodeKey { rung: Doc, file: beetsplug/bpd/__init__.py, decl: 114, sub: 0, line: 960 } |  |  | 0.504 |
| walker |  | 5387 | 17 | Code::CodeKey { rung: Doc, file: beetsplug/bpd/__init__.py, decl: 107, sub: 0, line: 905 } |  |  | 0.504 |
| walker |  | 5405 | 18 | Code::CodeKey { rung: Doc, file: beetsplug/bpd/__init__.py, decl: 95, sub: 0, line: 757 } |  |  | 0.504 |
| walker |  | 5423 | 18 | Code::CodeKey { rung: Doc, file: beetsplug/bpd/__init__.py, decl: 101, sub: 0, line: 797 } |  |  | 0.504 |
| walker |  | 5441 | 18 | Code::CodeKey { rung: Doc, file: beetsplug/bpd/__init__.py, decl: 108, sub: 0, line: 908 } |  |  | 0.504 |
| ns | 5458 |  | 168 | Item search fields and MediaFile-backed field sets | 4.4 | 2.5 | 0.498 |
| walker |  | 5556 | 115 | Code::CodeKey { rung: Decl, file: beetsplug/bpd/__init__.py, decl: 26, sub: 0, line: 90 } |  |  | 0.498 |
| walker |  | 5575 | 19 | Code::CodeKey { rung: Doc, file: beetsplug/bpd/__init__.py, decl: 30, sub: 0, line: 138 } |  |  | 0.498 |
| walker |  | 5594 | 19 | Code::CodeKey { rung: Doc, file: beetsplug/bpd/__init__.py, decl: 110, sub: 0, line: 915 } |  |  | 0.498 |
| walker |  | 5619 | 25 | Code::CodeKey { rung: Doc, file: beetsplug/bpd/__init__.py, decl: 102, sub: 0, line: 804 } |  |  | 0.498 |
| ns | 5629 |  | 171 | LibModel shared-method roster (complete after `_fields`) | 4.5 | 2.5 | 0.491 |
| walker |  | 5649 | 30 | Code::CodeKey { rung: Doc, file: beetsplug/bpd/__init__.py, decl: 35, sub: 0, line: 174 } |  |  | 0.491 |
| walker |  | 5680 | 31 | Code::CodeKey { rung: Doc, file: beetsplug/bpd/__init__.py, decl: 27, sub: 0, line: 110 } |  |  | 0.491 |
| walker |  | 5712 | 32 | Code::CodeKey { rung: Doc, file: beetsplug/bpd/__init__.py, decl: 106, sub: 0, line: 838 } |  |  | 0.491 |
| walker |  | 5744 | 32 | Code::CodeKey { rung: Doc, file: beetsplug/bpd/__init__.py, decl: 117, sub: 0, line: 1017 } |  |  | 0.491 |
| walker |  | 5778 | 34 | Code::CodeKey { rung: Doc, file: beetsplug/bpd/__init__.py, decl: 29, sub: 0, line: 123 } |  |  | 0.491 |
| ns | 5787 |  | 158 | beets.importer public exports + package docstring | 5.1 |  | 0.485 |
| walker |  | 5813 | 35 | Code::CodeKey { rung: Doc, file: beetsplug/bpd/__init__.py, decl: 168, sub: 0, line: 1583 } |  |  | 0.485 |
| walker |  | 5853 | 40 | Code::CodeKey { rung: Doc, file: beetsplug/bpd/__init__.py, decl: 36, sub: 0, line: 180 } |  |  | 0.485 |
| walker |  | 5894 | 41 | Code::CodeKey { rung: Doc, file: beetsplug/bpd/__init__.py, decl: 115, sub: 0, line: 966 } |  |  | 0.485 |
| walker |  | 5942 | 48 | Code::CodeKey { rung: Doc, file: beetsplug/bpd/__init__.py, decl: 119, sub: 0, line: 1070 } |  |  | 0.485 |
| ns | 6147 |  | 360 | importer/stages.py: complete stage-function roster with section banners | 5.2 |  | 0.475 |
| walker |  | 6200 | 258 | Code::CodeKey { rung: Names, file: beets/ui/__init__.py, decl: 0, sub: 0, line: 0 } |  |  | 0.475 |
| walker |  | 6227 | 27 | Code::CodeKey { rung: Decl, file: beets/ui/__init__.py, decl: 2, sub: 0, line: 65 } |  |  | 0.475 |
| walker |  | 6239 | 12 | Code::CodeKey { rung: Body, file: beets/ui/__init__.py, decl: 4, sub: 0, line: 80 } |  |  | 0.475 |
| walker |  | 6251 | 12 | Code::CodeKey { rung: Body, file: beets/ui/__init__.py, decl: 5, sub: 0, line: 85 } |  |  | 0.475 |
| walker |  | 6270 | 19 | Code::CodeKey { rung: Doc, file: beets/ui/__init__.py, decl: 4, sub: 0, line: 80 } |  |  | 0.475 |
| walker |  | 6289 | 19 | Code::CodeKey { rung: Doc, file: beets/ui/__init__.py, decl: 5, sub: 0, line: 85 } |  |  | 0.475 |
| walker |  | 6326 | 37 | Code::CodeKey { rung: Doc, file: beets/ui/__init__.py, decl: 3, sub: 0, line: 71 } |  |  | 0.475 |
| walker |  | 6383 | 57 | Code::CodeKey { rung: Doc, file: beets/ui/__init__.py, decl: 6, sub: 0, line: 90 } |  |  | 0.475 |
| walker |  | 6449 | 66 | Code::CodeKey { rung: Doc, file: beetsplug/bpd/__init__.py, decl: 118, sub: 0, line: 1064 } |  |  | 0.475 |
| ns | 6572 |  | 425 | ImportSession.run(): how the pipeline is assembled | 5.3 | 5.2 | 0.461 |
| walker |  | 6635 | 186 | Code::CodeKey { rung: Names, file: beetsplug/metasync/__init__.py, decl: 0, sub: 0, line: 0 } |  |  | 0.461 |
| walker |  | 6664 | 29 | Code::CodeKey { rung: Decl, file: beetsplug/metasync/__init__.py, decl: 2, sub: 0, line: 34 } |  |  | 0.461 |
| walker |  | 6716 | 52 | Code::CodeKey { rung: Decl, file: beetsplug/metasync/__init__.py, decl: 3, sub: 0, line: 40 } |  |  | 0.461 |
| walker |  | 6725 | 9 | Code::CodeKey { rung: Decl, file: beetsplug/metasync/__init__.py, decl: 5, sub: 0, line: 47 } |  |  | 0.461 |
| walker |  | 6730 | 5 | Code::CodeKey { rung: Body, file: beetsplug/metasync/__init__.py, decl: 5, sub: 0, line: 47 } |  |  | 0.461 |
| walker |  | 6786 | 56 | Code::CodeKey { rung: Decl, file: beetsplug/metasync/__init__.py, decl: 9, sub: 0, line: 76 } |  |  | 0.461 |
| walker |  | 6800 | 14 | Code::CodeKey { rung: Doc, file: beetsplug/metasync/__init__.py, decl: 12, sub: 0, line: 104 } |  |  | 0.461 |
| walker |  | 6819 | 19 | Code::CodeKey { rung: Doc, file: beetsplug/metasync/__init__.py, decl: 8, sub: 0, line: 68 } |  |  | 0.461 |
| walker |  | 6826 | 7 | Code::CodeKey { rung: Body, file: beetsplug/metasync/__init__.py, decl: 10, sub: 0, line: 79 } |  |  | 0.461 |
| ns | 6829 |  | 257 | ImportSession class contract (attributes + the four decision hooks) | 5.4 | 5.3 | 0.452 |
| walker |  | 6871 | 45 | Code::CodeKey { rung: Doc, file: beetsplug/metasync/__init__.py, decl: 6, sub: 0, line: 52 } |  |  | 0.452 |
| walker |  | 6943 | 72 | Code::CodeKey { rung: Names, file: beets/__init__.py, decl: 0, sub: 0, line: 0 } |  |  | 0.452 |
| walker |  | 6969 | 26 | Code::CodeKey { rung: Decl, file: beets/__init__.py, decl: 2, sub: 0, line: 35 } |  |  | 0.452 |
| walker |  | 6979 | 10 | Code::CodeKey { rung: Doc, file: beets/__init__.py, decl: 1, sub: 0, line: 26 } |  |  | 0.452 |
| walker |  | 7019 | 40 | Code::CodeKey { rung: Doc, file: beets/__init__.py, decl: 2, sub: 0, line: 35 } |  |  | 0.452 |
| walker |  | 7076 | 57 | Code::CodeKey { rung: Body, file: beets/__init__.py, decl: 1, sub: 0, line: 26 } |  |  | 0.452 |
| ns | 7103 |  | 274 | importer/tasks.py: Action enum and the complete task class roster | 5.5 |  | 0.445 |
| ns | 7349 |  | 246 | BeetsPlugin base class: docstring and declared attributes | 6.1 |  | 0.438 |
| walker |  | 7361 | 285 | Code::CodeKey { rung: Names, file: beets/ui/__init__.py, decl: 0, sub: 1, line: 0 } |  |  | 0.438 |
| walker |  | 7368 | 7 | Code::CodeKey { rung: Decl, file: beets/ui/__init__.py, decl: 18, sub: 0, line: 459 } |  |  | 0.438 |
| walker |  | 7394 | 26 | Code::CodeKey { rung: Decl, file: beets/ui/__init__.py, decl: 40, sub: 0, line: 807 } |  |  | 0.438 |
| walker |  | 7458 | 64 | Code::CodeKey { rung: Decl, file: beets/ui/__init__.py, decl: 13, sub: 0, line: 207 } |  |  | 0.438 |
| walker |  | 7530 | 72 | Code::CodeKey { rung: Decl, file: beets/ui/__init__.py, decl: 19, sub: 0, line: 466 } |  |  | 0.438 |
| walker |  | 7545 | 15 | Code::CodeKey { rung: Doc, file: beets/ui/__init__.py, decl: 43, sub: 0, line: 879 } |  |  | 0.438 |
| walker |  | 7561 | 16 | Code::CodeKey { rung: Doc, file: beets/ui/__init__.py, decl: 17, sub: 0, line: 445 } |  |  | 0.438 |
| walker |  | 7577 | 16 | Code::CodeKey { rung: Doc, file: beets/ui/__init__.py, decl: 18, sub: 0, line: 459 } |  |  | 0.438 |
| ns | 7595 |  | 246 | BeetsPlugin method roster (complete) | 6.2 | 6.1 | 0.434 |
| walker |  | 7675 | 98 | Code::CodeKey { rung: Decl, file: beets/ui/__init__.py, decl: 33, sub: 0, line: 676 } |  |  | 0.434 |
| walker |  | 7692 | 17 | Code::CodeKey { rung: Doc, file: beets/ui/__init__.py, decl: 41, sub: 0, line: 830 } |  |  | 0.434 |
| walker |  | 7797 | 105 | Code::CodeKey { rung: Decl, file: beets/ui/__init__.py, decl: 27, sub: 0, line: 637 } |  |  | 0.434 |
| walker |  | 7805 | 8 | Code::CodeKey { rung: Decl, file: beets/ui/__init__.py, decl: 31, sub: 0, line: 664 } |  |  | 0.434 |
| walker |  | 7815 | 10 | Code::CodeKey { rung: Decl, file: beets/ui/__init__.py, decl: 32, sub: 0, line: 668 } |  |  | 0.434 |
| walker |  | 7928 | 113 | Code::CodeKey { rung: Decl, file: beets/ui/__init__.py, decl: 20, sub: 0, line: 498 } |  |  | 0.434 |
| walker |  | 7995 | 67 | Code::CodeKey { rung: Decl, file: beets/ui/__init__.py, decl: 23, sub: 0, line: 534 } |  |  | 0.434 |
| walker |  | 8009 | 14 | Code::CodeKey { rung: Doc, file: beets/ui/__init__.py, decl: 26, sub: 0, line: 621 } |  |  | 0.434 |
| ns | 8017 |  | 422 | The complete `EventType` literal — every plugin event name | 6.3 | 6.1 | 0.422 |
| walker |  | 8028 | 19 | Code::CodeKey { rung: Doc, file: beets/ui/__init__.py, decl: 35, sub: 0, line: 701 } |  |  | 0.422 |
| walker |  | 8048 | 20 | Code::CodeKey { rung: Doc, file: beets/ui/__init__.py, decl: 9, sub: 0, line: 150 } |  |  | 0.422 |
| walker |  | 8077 | 29 | Code::CodeKey { rung: Doc, file: beets/ui/__init__.py, decl: 33, sub: 0, line: 676 } |  |  | 0.422 |
| walker |  | 8107 | 30 | Code::CodeKey { rung: Doc, file: beets/ui/__init__.py, decl: 23, sub: 0, line: 534 } |  |  | 0.422 |
| walker |  | 8138 | 31 | Code::CodeKey { rung: Doc, file: beets/ui/__init__.py, decl: 44, sub: 0, line: 905 } |  |  | 0.422 |
| walker |  | 8170 | 32 | Code::CodeKey { rung: Doc, file: beets/ui/__init__.py, decl: 16, sub: 0, line: 433 } |  |  | 0.422 |
| walker |  | 8206 | 36 | Code::CodeKey { rung: Doc, file: beets/ui/__init__.py, decl: 27, sub: 0, line: 637 } |  |  | 0.422 |
| walker |  | 8245 | 39 | Code::CodeKey { rung: Doc, file: beets/ui/__init__.py, decl: 10, sub: 0, line: 160 } |  |  | 0.422 |
| walker |  | 8284 | 39 | Code::CodeKey { rung: Doc, file: beets/ui/__init__.py, decl: 38, sub: 0, line: 770 } |  |  | 0.422 |
| ns | 8321 |  | 304 | plugins.py module-level API roster (complete) | 6.4 |  | 0.417 |
| walker |  | 8323 | 39 | Code::CodeKey { rung: Doc, file: beets/ui/__init__.py, decl: 45, sub: 0, line: 996 } |  |  | 0.417 |
| walker |  | 8374 | 51 | Code::CodeKey { rung: Doc, file: beets/ui/__init__.py, decl: 40, sub: 0, line: 807 } |  |  | 0.417 |
| walker |  | 8428 | 54 | Code::CodeKey { rung: Doc, file: beets/ui/__init__.py, decl: 39, sub: 0, line: 783 } |  |  | 0.417 |
| walker |  | 8483 | 55 | Code::CodeKey { rung: Doc, file: beets/ui/__init__.py, decl: 14, sub: 0, line: 383 } |  |  | 0.417 |
| walker |  | 8541 | 58 | Code::CodeKey { rung: Doc, file: beets/ui/__init__.py, decl: 34, sub: 0, line: 681 } |  |  | 0.417 |
| walker |  | 8602 | 61 | Code::CodeKey { rung: Doc, file: beets/ui/__init__.py, decl: 37, sub: 0, line: 760 } |  |  | 0.417 |
| ns | 8633 |  | 312 | MetadataSourcePlugin: the metadata-backend contract | 6.5 |  | 0.411 |
| walker |  | 8671 | 69 | Code::CodeKey { rung: Doc, file: beetsplug/bpd/__init__.py, decl: 98, sub: 0, line: 770 } |  |  | 0.411 |
| walker |  | 8743 | 72 | Code::CodeKey { rung: Doc, file: beetsplug/bpd/__init__.py, decl: 116, sub: 0, line: 985 } |  |  | 0.411 |
| ns | 8830 |  | 197 | Query-string prefixes: how `beet ls artist:foo` is parsed | 7.1 |  | 0.407 |
| walker |  | 8898 | 155 | Code::CodeKey { rung: Names, file: beetsplug/tidal/__init__.py, decl: 0, sub: 0, line: 0 } |  |  | 0.407 |
| walker |  | 8977 | 79 | Code::CodeKey { rung: Decl, file: beetsplug/tidal/__init__.py, decl: 34, sub: 0, line: 494 } |  |  | 0.407 |
| walker |  | 9050 | 73 | Code::CodeKey { rung: Doc, file: beets/ui/__init__.py, decl: 12, sub: 0, line: 187 } |  |  | 0.407 |
| walker |  | 9082 | 32 | Code::CodeKey { rung: Names, file: beetsplug/_utils/__init__.py, decl: 0, sub: 0, line: 0 } |  |  | 0.407 |
| ns | 9187 |  | 357 | dbcore/query.py class roster (complete) | 7.2 |  | 0.399 |
| walker |  | 9299 | 217 | Code::CodeKey { rung: Decl, file: beetsplug/tidal/__init__.py, decl: 2, sub: 0, line: 37 } |  |  | 0.399 |
| walker |  | 9307 | 8 | Code::CodeKey { rung: Decl, file: beetsplug/tidal/__init__.py, decl: 14, sub: 0, line: 161 } |  |  | 0.399 |
| walker |  | 9316 | 9 | Code::CodeKey { rung: Decl, file: beetsplug/tidal/__init__.py, decl: 4, sub: 0, line: 53 } |  |  | 0.399 |
| walker |  | 9347 | 31 | Code::CodeKey { rung: Decl, file: beetsplug/tidal/__init__.py, decl: 13, sub: 0, line: 142 } |  |  | 0.399 |
| walker |  | 9387 | 40 | Code::CodeKey { rung: Decl, file: beetsplug/tidal/__init__.py, decl: 12, sub: 0, line: 119 } |  |  | 0.399 |
| walker |  | 9398 | 11 | Code::CodeKey { rung: Doc, file: beetsplug/tidal/__init__.py, decl: 14, sub: 0, line: 161 } |  |  | 0.399 |
| walker |  | 9417 | 19 | Code::CodeKey { rung: Doc, file: beetsplug/tidal/__init__.py, decl: 5, sub: 0, line: 60 } |  |  | 0.399 |
| walker |  | 9431 | 14 | Code::CodeKey { rung: Body, file: beetsplug/tidal/__init__.py, decl: 11, sub: 0, line: 116 } |  |  | 0.399 |
| walker |  | 9509 | 78 | Code::CodeKey { rung: Body, file: beets/ui/commands/__init__.py, decl: 1, sub: 0, line: 36 } |  |  | 0.399 |
| ns | 9539 |  | 352 | beets.autotag exports, Recommendation, Proposal | 8.1 |  | 0.392 |
| walker |  | 9589 | 80 | Code::CodeKey { rung: Doc, file: beets/ui/__init__.py, decl: 22, sub: 0, line: 521 } |  |  | 0.392 |
| ns | 9775 |  | 236 | autotag/hooks.py and distance.py symbol roster | 8.2 |  | 0.388 |
| walker |  | 9898 | 309 | Code::CodeKey { rung: Names, file: beetsplug/discogs/__init__.py, decl: 0, sub: 0, line: 0 } |  |  | 0.388 |
| walker |  | 9954 | 56 | Code::CodeKey { rung: Decl, file: beetsplug/discogs/__init__.py, decl: 4, sub: 0, line: 60 } |  |  | 0.388 |
| ns | 9970 |  | 195 | beets/util/__init__.py: classes and the core path/file-operation helpers | 9.1 |  | 0.383 |
| walker |  | 10000 | 46 | Code::CodeKey { rung: Decl, file: beetsplug/discogs/__init__.py, decl: 6, sub: 0, line: 83 } |  |  | 0.383 |
