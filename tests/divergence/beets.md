Score(3000)=0.661 I=0.799 C=0.546 ns_rows≤3K=17/44 grid(1000/1442/2080/3000/4327/6240/9000)=0.594/0.757/0.585/0.661/0.588/0.525/0.450

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
| walker |  | 3144 | 206 | Code::CodeKey { rung: Names, file: beetsplug/bpd/__init__.py, decl: 0, sub: 0, line: 0 } |  |  | 0.672 |
| ns | 3298 |  | 173 | config_default.yaml: library, directory, plugins, ignore rules | 3.1 |  | 0.654 |
| walker |  | 3588 | 444 | Code::CodeKey { rung: Names, file: beetsplug/web/__init__.py, decl: 0, sub: 0, line: 0 } |  |  | 0.654 |
| walker |  | 3596 | 8 | Code::CodeKey { rung: Decl, file: beetsplug/web/__init__.py, decl: 32, sub: 0, line: 439 } |  |  | 0.654 |
| walker |  | 3605 | 9 | Code::CodeKey { rung: Decl, file: beetsplug/web/__init__.py, decl: 18, sub: 0, line: 296 } |  |  | 0.654 |
| walker |  | 3615 | 10 | Code::CodeKey { rung: Decl, file: beetsplug/web/__init__.py, decl: 30, sub: 0, line: 412 } |  |  | 0.654 |
| walker |  | 3625 | 10 | Code::CodeKey { rung: Decl, file: beetsplug/web/__init__.py, decl: 31, sub: 0, line: 423 } |  |  | 0.654 |
| walker |  | 3639 | 14 | Code::CodeKey { rung: Decl, file: beetsplug/web/__init__.py, decl: 23, sub: 0, line: 344 } |  |  | 0.654 |
| walker |  | 3654 | 15 | Code::CodeKey { rung: Decl, file: beetsplug/web/__init__.py, decl: 24, sub: 0, line: 354 } |  |  | 0.654 |
| walker |  | 3669 | 15 | Code::CodeKey { rung: Decl, file: beetsplug/web/__init__.py, decl: 29, sub: 0, line: 397 } |  |  | 0.654 |
| walker |  | 3685 | 16 | Code::CodeKey { rung: Decl, file: beetsplug/web/__init__.py, decl: 21, sub: 0, line: 317 } |  |  | 0.654 |
| ns | 3687 |  | 389 | config_default.yaml: the complete `import:` option block | 3.2 |  | 0.621 |
| walker |  | 3702 | 17 | Code::CodeKey { rung: Decl, file: beetsplug/web/__init__.py, decl: 28, sub: 0, line: 388 } |  |  | 0.621 |
| walker |  | 3724 | 22 | Code::CodeKey { rung: Decl, file: beetsplug/web/__init__.py, decl: 16, sub: 0, line: 283 } |  |  | 0.621 |
| walker |  | 3746 | 22 | Code::CodeKey { rung: Decl, file: beetsplug/web/__init__.py, decl: 33, sub: 0, line: 447 } |  |  | 0.621 |
| walker |  | 3774 | 28 | Code::CodeKey { rung: Decl, file: beetsplug/web/__init__.py, decl: 10, sub: 0, line: 252 } |  |  | 0.621 |
| walker |  | 3802 | 28 | Code::CodeKey { rung: Decl, file: beetsplug/web/__init__.py, decl: 13, sub: 0, line: 268 } |  |  | 0.621 |
| walker |  | 3833 | 31 | Code::CodeKey { rung: Decl, file: beetsplug/web/__init__.py, decl: 20, sub: 0, line: 310 } |  |  | 0.621 |
| walker |  | 3864 | 31 | Code::CodeKey { rung: Decl, file: beetsplug/web/__init__.py, decl: 25, sub: 0, line: 369 } |  |  | 0.621 |
| ns | 3866 |  | 179 | config_default.yaml: tagging defaults, `paths:` templates, threading | 3.4 |  | 0.608 |
| walker |  | 3895 | 31 | Code::CodeKey { rung: Decl, file: beetsplug/web/__init__.py, decl: 26, sub: 0, line: 375 } |  |  | 0.608 |
| walker |  | 3927 | 32 | Code::CodeKey { rung: Decl, file: beetsplug/web/__init__.py, decl: 27, sub: 0, line: 382 } |  |  | 0.608 |
| walker |  | 3960 | 33 | Code::CodeKey { rung: Decl, file: beetsplug/web/__init__.py, decl: 36, sub: 0, line: 520 } |  |  | 0.608 |
| walker |  | 3998 | 38 | Code::CodeKey { rung: Decl, file: beetsplug/web/__init__.py, decl: 19, sub: 0, line: 304 } |  |  | 0.608 |
| walker |  | 4037 | 39 | Code::CodeKey { rung: Decl, file: beetsplug/web/__init__.py, decl: 22, sub: 0, line: 338 } |  |  | 0.608 |
| walker |  | 4052 | 15 | Code::CodeKey { rung: Doc, file: beetsplug/web/__init__.py, decl: 5, sub: 0, line: 117 } |  |  | 0.608 |
| ns | 4171 |  | 305 | Path-template function roster: DefaultTemplateFunctions (complete) | 3.5 |  | 0.594 |
| walker |  | 4263 | 211 | Code::CodeKey { rung: Names, file: beets/ui/commands/__init__.py, decl: 0, sub: 0, line: 0 } |  |  | 0.595 |
| ns | 4291 |  | 120 | config_default.yaml: display formats and default sorts | 3.6 |  | 0.588 |
| walker |  | 4370 | 107 | Code::CodeKey { rung: Decl, file: beets/ui/commands/__init__.py, decl: 2, sub: 0, line: 50 } |  |  | 0.603 |
| walker |  | 4380 | 10 | Code::CodeKey { rung: Doc, file: beets/ui/commands/__init__.py, decl: 1, sub: 0, line: 36 } |  |  | 0.603 |
| walker |  | 4570 | 190 | Code::CodeKey { rung: Names, file: beets/autotag/__init__.py, decl: 0, sub: 0, line: 0 } |  |  | 0.603 |
| walker |  | 4587 | 17 | Code::CodeKey { rung: Doc, file: beetsplug/web/__init__.py, decl: 3, sub: 0, line: 103 } |  |  | 0.603 |
| walker |  | 4604 | 17 | Code::CodeKey { rung: Doc, file: beetsplug/web/__init__.py, decl: 10, sub: 0, line: 252 } |  |  | 0.603 |
| ns | 4673 |  | 382 | config_default.yaml: autotagger thresholds and distance weights | 3.7 |  | 0.581 |
| ns | 4813 |  | 140 | config_default.yaml: terminal UI, colour names, import layout | 3.8 |  | 0.570 |
| walker |  | 4866 | 262 | Code::CodeKey { rung: Names, file: beetsplug/bpd/__init__.py, decl: 0, sub: 1, line: 0 } |  |  | 0.570 |
| walker |  | 4927 | 61 | Code::CodeKey { rung: Decl, file: beetsplug/bpd/__init__.py, decl: 25, sub: 0, line: 80 } |  |  | 0.570 |
| ns | 4986 |  | 173 | Album method roster (complete, names only) | 4.1 | 2.5 | 0.561 |
| walker |  | 4989 | 62 | Code::CodeKey { rung: Decl, file: beetsplug/bpd/__init__.py, decl: 27, sub: 0, line: 110 } |  |  | 0.561 |
| walker |  | 5104 | 115 | Code::CodeKey { rung: Decl, file: beetsplug/bpd/__init__.py, decl: 26, sub: 0, line: 90 } |  |  | 0.561 |
| walker |  | 5286 | 182 | Code::CodeKey { rung: Names, file: beetsplug/bpd/__init__.py, decl: 0, sub: 2, line: 0 } |  |  | 0.561 |
| ns | 5290 |  | 304 | Item method roster (complete, names only) | 4.3 | 2.5 | 0.547 |
| walker |  | 5302 | 16 | Code::CodeKey { rung: Decl, file: beetsplug/bpd/__init__.py, decl: 36, sub: 0, line: 180 } |  |  | 0.547 |
| walker |  | 5336 | 34 | Code::CodeKey { rung: Decl, file: beetsplug/bpd/__init__.py, decl: 118, sub: 0, line: 1064 } |  |  | 0.547 |
| walker |  | 5389 | 53 | Code::CodeKey { rung: Decl, file: beetsplug/bpd/__init__.py, decl: 168, sub: 0, line: 1583 } |  |  | 0.547 |
| ns | 5458 |  | 168 | Item search fields and MediaFile-backed field sets | 4.4 | 2.5 | 0.540 |
| walker |  | 5460 | 71 | Code::CodeKey { rung: Decl, file: beetsplug/bpd/__init__.py, decl: 94, sub: 0, line: 754 } |  |  | 0.540 |
| walker |  | 5468 | 8 | Code::CodeKey { rung: Decl, file: beetsplug/bpd/__init__.py, decl: 99, sub: 0, line: 785 } |  |  | 0.540 |
| walker |  | 5551 | 83 | Code::CodeKey { rung: Decl, file: beetsplug/bpd/__init__.py, decl: 100, sub: 0, line: 794 } |  |  | 0.540 |
| ns | 5629 |  | 171 | LibModel shared-method roster (complete after `_fields`) | 4.5 | 2.5 | 0.533 |
| walker |  | 5640 | 89 | Code::CodeKey { rung: Decl, file: beetsplug/bpd/__init__.py, decl: 107, sub: 0, line: 905 } |  |  | 0.533 |
| walker |  | 5743 | 103 | Code::CodeKey { rung: Decl, file: beetsplug/bpd/__init__.py, decl: 114, sub: 0, line: 960 } |  |  | 0.533 |
| ns | 5787 |  | 158 | beets.importer public exports + package docstring | 5.1 |  | 0.536 |
| walker |  | 5950 | 207 | Code::CodeKey { rung: Decl, file: beetsplug/bpd/__init__.py, decl: 38, sub: 0, line: 193 } |  |  | 0.536 |
| ns | 6147 |  | 360 | importer/stages.py: complete stage-function roster with section banners | 5.2 |  | 0.525 |
| walker |  | 6153 | 203 | Code::CodeKey { rung: Decl, file: beetsplug/bpd/__init__.py, decl: 38, sub: 1, line: 193 } |  |  | 0.525 |
| walker |  | 6351 | 198 | Code::CodeKey { rung: Decl, file: beetsplug/bpd/__init__.py, decl: 38, sub: 2, line: 193 } |  |  | 0.525 |
| ns | 6572 |  | 425 | ImportSession.run(): how the pipeline is assembled | 5.3 | 5.2 | 0.510 |
| walker |  | 6644 | 293 | Code::CodeKey { rung: Decl, file: beetsplug/bpd/__init__.py, decl: 38, sub: 3, line: 193 } |  |  | 0.510 |
| walker |  | 6653 | 9 | Code::CodeKey { rung: Doc, file: beetsplug/bpd/__init__.py, decl: 52, sub: 0, line: 346 } |  |  | 0.510 |
| walker |  | 6662 | 9 | Code::CodeKey { rung: Doc, file: beetsplug/bpd/__init__.py, decl: 90, sub: 0, line: 725 } |  |  | 0.510 |
| walker |  | 6672 | 10 | Code::CodeKey { rung: Doc, file: beetsplug/bpd/__init__.py, decl: 56, sub: 0, line: 365 } |  |  | 0.510 |
| walker |  | 6682 | 10 | Code::CodeKey { rung: Doc, file: beetsplug/bpd/__init__.py, decl: 72, sub: 0, line: 526 } |  |  | 0.510 |
| walker |  | 6693 | 11 | Code::CodeKey { rung: Doc, file: beetsplug/bpd/__init__.py, decl: 55, sub: 0, line: 361 } |  |  | 0.510 |
| walker |  | 6704 | 11 | Code::CodeKey { rung: Doc, file: beetsplug/bpd/__init__.py, decl: 58, sub: 0, line: 386 } |  |  | 0.510 |
| walker |  | 6715 | 11 | Code::CodeKey { rung: Doc, file: beetsplug/bpd/__init__.py, decl: 112, sub: 0, line: 943 } |  |  | 0.510 |
| walker |  | 6727 | 12 | Code::CodeKey { rung: Doc, file: beetsplug/bpd/__init__.py, decl: 40, sub: 0, line: 241 } |  |  | 0.510 |
| walker |  | 6739 | 12 | Code::CodeKey { rung: Doc, file: beetsplug/bpd/__init__.py, decl: 54, sub: 0, line: 357 } |  |  | 0.510 |
| walker |  | 6751 | 12 | Code::CodeKey { rung: Doc, file: beetsplug/bpd/__init__.py, decl: 62, sub: 0, line: 458 } |  |  | 0.510 |
| walker |  | 6763 | 12 | Code::CodeKey { rung: Doc, file: beetsplug/bpd/__init__.py, decl: 63, sub: 0, line: 463 } |  |  | 0.510 |
| walker |  | 6775 | 12 | Code::CodeKey { rung: Doc, file: beetsplug/bpd/__init__.py, decl: 64, sub: 0, line: 468 } |  |  | 0.510 |
| walker |  | 6787 | 12 | Code::CodeKey { rung: Doc, file: beetsplug/bpd/__init__.py, decl: 70, sub: 0, line: 514 } |  |  | 0.510 |
| walker |  | 6799 | 12 | Code::CodeKey { rung: Doc, file: beetsplug/bpd/__init__.py, decl: 71, sub: 0, line: 522 } |  |  | 0.510 |
| walker |  | 6811 | 12 | Code::CodeKey { rung: Doc, file: beetsplug/bpd/__init__.py, decl: 87, sub: 0, line: 689 } |  |  | 0.510 |
| ns | 6829 |  | 257 | ImportSession class contract (attributes + the four decision hooks) | 5.4 | 5.3 | 0.501 |
| walker |  | 7011 | 200 | Code::CodeKey { rung: Decl, file: beetsplug/bpd/__init__.py, decl: 121, sub: 0, line: 1099 } |  |  | 0.501 |
| ns | 7103 |  | 274 | importer/tasks.py: Action enum and the complete task class roster | 5.5 |  | 0.492 |
| ns | 7349 |  | 246 | BeetsPlugin base class: docstring and declared attributes | 6.1 |  | 0.485 |
| walker |  | 7500 | 489 | Code::CodeKey { rung: Decl, file: beetsplug/bpd/__init__.py, decl: 121, sub: 1, line: 1099 } |  |  | 0.485 |
| ns | 7595 |  | 246 | BeetsPlugin method roster (complete) | 6.2 | 6.1 | 0.480 |
| walker |  | 7688 | 188 | Code::CodeKey { rung: Decl, file: beetsplug/bpd/__init__.py, decl: 121, sub: 2, line: 1099 } |  |  | 0.480 |
| walker |  | 7984 | 296 | Code::CodeKey { rung: Decl, file: beetsplug/bpd/__init__.py, decl: 121, sub: 3, line: 1099 } |  |  | 0.480 |
| walker |  | 7996 | 12 | Code::CodeKey { rung: Doc, file: beetsplug/bpd/__init__.py, decl: 160, sub: 0, line: 1518 } |  |  | 0.480 |
| walker |  | 8009 | 13 | Code::CodeKey { rung: Doc, file: beetsplug/bpd/__init__.py, decl: 45, sub: 0, line: 286 } |  |  | 0.480 |
| ns | 8017 |  | 422 | The complete `EventType` literal — every plugin event name | 6.3 | 6.1 | 0.467 |
| walker |  | 8022 | 13 | Code::CodeKey { rung: Doc, file: beetsplug/bpd/__init__.py, decl: 75, sub: 0, line: 552 } |  |  | 0.467 |
| walker |  | 8035 | 13 | Code::CodeKey { rung: Doc, file: beetsplug/bpd/__init__.py, decl: 86, sub: 0, line: 676 } |  |  | 0.467 |
| walker |  | 8048 | 13 | Code::CodeKey { rung: Doc, file: beetsplug/bpd/__init__.py, decl: 94, sub: 0, line: 754 } |  |  | 0.467 |
| walker |  | 8061 | 13 | Code::CodeKey { rung: Doc, file: beetsplug/bpd/__init__.py, decl: 170, sub: 0, line: 1601 } |  |  | 0.467 |
| walker |  | 8075 | 14 | Code::CodeKey { rung: Doc, file: beetsplug/bpd/__init__.py, decl: 57, sub: 0, line: 373 } |  |  | 0.467 |
| walker |  | 8089 | 14 | Code::CodeKey { rung: Doc, file: beetsplug/bpd/__init__.py, decl: 77, sub: 0, line: 577 } |  |  | 0.467 |
| walker |  | 8103 | 14 | Code::CodeKey { rung: Doc, file: beetsplug/bpd/__init__.py, decl: 96, sub: 0, line: 763 } |  |  | 0.467 |
| walker |  | 8117 | 14 | Code::CodeKey { rung: Doc, file: beetsplug/bpd/__init__.py, decl: 103, sub: 0, line: 816 } |  |  | 0.467 |
| walker |  | 8131 | 14 | Code::CodeKey { rung: Doc, file: beetsplug/bpd/__init__.py, decl: 105, sub: 0, line: 825 } |  |  | 0.467 |
| walker |  | 8145 | 14 | Code::CodeKey { rung: Doc, file: beetsplug/bpd/__init__.py, decl: 113, sub: 0, line: 950 } |  |  | 0.467 |
| walker |  | 8159 | 14 | Code::CodeKey { rung: Doc, file: beetsplug/bpd/__init__.py, decl: 145, sub: 0, line: 1416 } |  |  | 0.467 |
| walker |  | 8173 | 14 | Code::CodeKey { rung: Doc, file: beetsplug/bpd/__init__.py, decl: 146, sub: 0, line: 1424 } |  |  | 0.467 |
| walker |  | 8188 | 15 | Code::CodeKey { rung: Doc, file: beetsplug/bpd/__init__.py, decl: 41, sub: 0, line: 245 } |  |  | 0.467 |
| walker |  | 8203 | 15 | Code::CodeKey { rung: Doc, file: beetsplug/bpd/__init__.py, decl: 61, sub: 0, line: 453 } |  |  | 0.467 |
| walker |  | 8218 | 15 | Code::CodeKey { rung: Doc, file: beetsplug/bpd/__init__.py, decl: 69, sub: 0, line: 505 } |  |  | 0.467 |
| walker |  | 8233 | 15 | Code::CodeKey { rung: Doc, file: beetsplug/bpd/__init__.py, decl: 73, sub: 0, line: 533 } |  |  | 0.467 |
| walker |  | 8248 | 15 | Code::CodeKey { rung: Doc, file: beetsplug/bpd/__init__.py, decl: 84, sub: 0, line: 648 } |  |  | 0.467 |
| walker |  | 8263 | 15 | Code::CodeKey { rung: Doc, file: beetsplug/bpd/__init__.py, decl: 85, sub: 0, line: 654 } |  |  | 0.467 |
| walker |  | 8278 | 15 | Code::CodeKey { rung: Doc, file: beetsplug/bpd/__init__.py, decl: 130, sub: 0, line: 1204 } |  |  | 0.467 |
| walker |  | 8293 | 15 | Code::CodeKey { rung: Doc, file: beetsplug/bpd/__init__.py, decl: 140, sub: 0, line: 1314 } |  |  | 0.467 |
| walker |  | 8309 | 16 | Code::CodeKey { rung: Doc, file: beetsplug/bpd/__init__.py, decl: 67, sub: 0, line: 487 } |  |  | 0.467 |
| ns | 8321 |  | 304 | plugins.py module-level API roster (complete) | 6.4 |  | 0.461 |
| walker |  | 8325 | 16 | Code::CodeKey { rung: Doc, file: beetsplug/bpd/__init__.py, decl: 79, sub: 0, line: 604 } |  |  | 0.461 |
| walker |  | 8341 | 16 | Code::CodeKey { rung: Doc, file: beetsplug/bpd/__init__.py, decl: 88, sub: 0, line: 697 } |  |  | 0.461 |
| walker |  | 8357 | 16 | Code::CodeKey { rung: Doc, file: beetsplug/bpd/__init__.py, decl: 91, sub: 0, line: 731 } |  |  | 0.461 |
| walker |  | 8373 | 16 | Code::CodeKey { rung: Doc, file: beetsplug/bpd/__init__.py, decl: 100, sub: 0, line: 794 } |  |  | 0.461 |
| walker |  | 8389 | 16 | Code::CodeKey { rung: Doc, file: beetsplug/bpd/__init__.py, decl: 104, sub: 0, line: 821 } |  |  | 0.461 |
| walker |  | 8405 | 16 | Code::CodeKey { rung: Doc, file: beetsplug/bpd/__init__.py, decl: 111, sub: 0, line: 939 } |  |  | 0.461 |
| walker |  | 8421 | 16 | Code::CodeKey { rung: Doc, file: beetsplug/bpd/__init__.py, decl: 114, sub: 0, line: 960 } |  |  | 0.461 |
| walker |  | 8437 | 16 | Code::CodeKey { rung: Doc, file: beetsplug/bpd/__init__.py, decl: 120, sub: 0, line: 1079 } |  |  | 0.461 |
| walker |  | 8453 | 16 | Code::CodeKey { rung: Doc, file: beetsplug/bpd/__init__.py, decl: 141, sub: 0, line: 1336 } |  |  | 0.461 |
| walker |  | 8470 | 17 | Code::CodeKey { rung: Doc, file: beetsplug/bpd/__init__.py, decl: 43, sub: 0, line: 269 } |  |  | 0.461 |
| walker |  | 8487 | 17 | Code::CodeKey { rung: Doc, file: beetsplug/bpd/__init__.py, decl: 47, sub: 0, line: 297 } |  |  | 0.461 |
| walker |  | 8504 | 17 | Code::CodeKey { rung: Doc, file: beetsplug/bpd/__init__.py, decl: 65, sub: 0, line: 474 } |  |  | 0.461 |
| walker |  | 8521 | 17 | Code::CodeKey { rung: Doc, file: beetsplug/bpd/__init__.py, decl: 66, sub: 0, line: 482 } |  |  | 0.461 |
| walker |  | 8538 | 17 | Code::CodeKey { rung: Doc, file: beetsplug/bpd/__init__.py, decl: 107, sub: 0, line: 905 } |  |  | 0.461 |
| walker |  | 8555 | 17 | Code::CodeKey { rung: Doc, file: beetsplug/bpd/__init__.py, decl: 128, sub: 0, line: 1167 } |  |  | 0.461 |
| walker |  | 8572 | 17 | Code::CodeKey { rung: Doc, file: beetsplug/bpd/__init__.py, decl: 135, sub: 0, line: 1258 } |  |  | 0.461 |
| walker |  | 8590 | 18 | Code::CodeKey { rung: Doc, file: beetsplug/bpd/__init__.py, decl: 95, sub: 0, line: 757 } |  |  | 0.461 |
| walker |  | 8608 | 18 | Code::CodeKey { rung: Doc, file: beetsplug/bpd/__init__.py, decl: 101, sub: 0, line: 797 } |  |  | 0.461 |
| walker |  | 8626 | 18 | Code::CodeKey { rung: Doc, file: beetsplug/bpd/__init__.py, decl: 108, sub: 0, line: 908 } |  |  | 0.461 |
| ns | 8633 |  | 312 | MetadataSourcePlugin: the metadata-backend contract | 6.5 |  | 0.455 |
| walker |  | 8644 | 18 | Code::CodeKey { rung: Doc, file: beetsplug/bpd/__init__.py, decl: 124, sub: 0, line: 1117 } |  |  | 0.455 |
| walker |  | 8662 | 18 | Code::CodeKey { rung: Doc, file: beetsplug/bpd/__init__.py, decl: 131, sub: 0, line: 1209 } |  |  | 0.455 |
| walker |  | 8680 | 18 | Code::CodeKey { rung: Doc, file: beetsplug/bpd/__init__.py, decl: 133, sub: 0, line: 1248 } |  |  | 0.455 |
| walker |  | 8698 | 18 | Code::CodeKey { rung: Doc, file: beetsplug/bpd/__init__.py, decl: 166, sub: 0, line: 1565 } |  |  | 0.455 |
| walker |  | 8716 | 18 | Code::CodeKey { rung: Doc, file: beetsplug/web/__init__.py, decl: 9, sub: 0, line: 241 } |  |  | 0.455 |
| walker |  | 8734 | 18 | Code::CodeKey { rung: Doc, file: beetsplug/web/__init__.py, decl: 13, sub: 0, line: 268 } |  |  | 0.455 |
| ns | 8830 |  | 197 | Query-string prefixes: how `beet ls artist:foo` is parsed | 7.1 |  | 0.450 |
| walker |  | 8920 | 186 | Code::CodeKey { rung: Names, file: beetsplug/metasync/__init__.py, decl: 0, sub: 0, line: 0 } |  |  | 0.450 |
| walker |  | 8949 | 29 | Code::CodeKey { rung: Decl, file: beetsplug/metasync/__init__.py, decl: 2, sub: 0, line: 34 } |  |  | 0.450 |
| walker |  | 9001 | 52 | Code::CodeKey { rung: Decl, file: beetsplug/metasync/__init__.py, decl: 3, sub: 0, line: 40 } |  |  | 0.450 |
| walker |  | 9010 | 9 | Code::CodeKey { rung: Decl, file: beetsplug/metasync/__init__.py, decl: 5, sub: 0, line: 47 } |  |  | 0.450 |
| walker |  | 9066 | 56 | Code::CodeKey { rung: Decl, file: beetsplug/metasync/__init__.py, decl: 9, sub: 0, line: 76 } |  |  | 0.450 |
| walker |  | 9071 | 5 | Code::CodeKey { rung: Body, file: beetsplug/metasync/__init__.py, decl: 5, sub: 0, line: 47 } |  |  | 0.450 |
| walker |  | 9085 | 14 | Code::CodeKey { rung: Doc, file: beetsplug/metasync/__init__.py, decl: 12, sub: 0, line: 104 } |  |  | 0.450 |
| walker |  | 9092 | 7 | Code::CodeKey { rung: Body, file: beetsplug/metasync/__init__.py, decl: 10, sub: 0, line: 79 } |  |  | 0.450 |
| ns | 9187 |  | 357 | dbcore/query.py class roster (complete) | 7.2 |  | 0.441 |
| walker |  | 9296 | 204 | Code::CodeKey { rung: Names, file: beets/ui/__init__.py, decl: 0, sub: 0, line: 0 } |  |  | 0.441 |
| walker |  | 9323 | 27 | Code::CodeKey { rung: Decl, file: beets/ui/__init__.py, decl: 2, sub: 0, line: 65 } |  |  | 0.441 |
| walker |  | 9395 | 72 | Code::CodeKey { rung: Names, file: beets/__init__.py, decl: 0, sub: 0, line: 0 } |  |  | 0.441 |
| walker |  | 9421 | 26 | Code::CodeKey { rung: Decl, file: beets/__init__.py, decl: 2, sub: 0, line: 35 } |  |  | 0.441 |
| walker |  | 9431 | 10 | Code::CodeKey { rung: Doc, file: beets/__init__.py, decl: 1, sub: 0, line: 26 } |  |  | 0.441 |
| ns | 9539 |  | 352 | beets.autotag exports, Recommendation, Proposal | 8.1 |  | 0.434 |
| walker |  | 9647 | 216 | Code::CodeKey { rung: Names, file: beets/ui/__init__.py, decl: 0, sub: 1, line: 0 } |  |  | 0.434 |
| walker |  | 9654 | 7 | Code::CodeKey { rung: Decl, file: beets/ui/__init__.py, decl: 18, sub: 0, line: 459 } |  |  | 0.434 |
| walker |  | 9718 | 64 | Code::CodeKey { rung: Decl, file: beets/ui/__init__.py, decl: 13, sub: 0, line: 207 } |  |  | 0.434 |
| ns | 9775 |  | 236 | autotag/hooks.py and distance.py symbol roster | 8.2 |  | 0.429 |
| walker |  | 9790 | 72 | Code::CodeKey { rung: Decl, file: beets/ui/__init__.py, decl: 19, sub: 0, line: 466 } |  |  | 0.429 |
| walker |  | 9903 | 113 | Code::CodeKey { rung: Decl, file: beets/ui/__init__.py, decl: 20, sub: 0, line: 498 } |  |  | 0.429 |
| walker |  | 9970 | 67 | Code::CodeKey { rung: Decl, file: beets/ui/__init__.py, decl: 23, sub: 0, line: 534 } |  |  | 0.424 |
| ns | 9970 |  | 195 | beets/util/__init__.py: classes and the core path/file-operation helpers | 9.1 |  | 0.424 |
| walker |  | 9984 | 14 | Code::CodeKey { rung: Doc, file: beets/ui/__init__.py, decl: 26, sub: 0, line: 621 } |  |  | 0.424 |
| walker |  | 10000 | 16 | Code::CodeKey { rung: Doc, file: beets/ui/__init__.py, decl: 17, sub: 0, line: 445 } |  |  | 0.424 |
