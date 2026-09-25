Score(3000)=0.661 I=0.799 C=0.546 ns_rows≤3K=17/44 grid(1000/1442/2080/3000/4327/6240/9000)=0.594/0.757/0.585/0.661/0.593/0.525/0.450

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
| walker |  | 1423 | 14 | Fs::DirListing { dir: .github/ISSUE_TEMPLATE } |  |  | 0.757 |
| ns | 1445 |  | 218 | test/ and docs/ top-level listings (complete) | 1.11 |  | 0.671 |
| walker |  | 1464 | 41 | Fs::DirListing { dir: docs/dev/plugins/other } |  |  | 0.671 |
| walker |  | 1735 | 271 | Code::CodeKey { rung: Names, file: beets/dbcore/__init__.py, decl: 0, sub: 0, line: 0 } |  |  | 0.674 |
| ns | 1828 |  | 383 | beetsplug/ bundled plugin listing (complete, 81 entries) | 1.12 |  | 0.553 |
| walker |  | 1887 | 152 | Code::CodeKey { rung: Names, file: beets/importer/__init__.py, decl: 0, sub: 0, line: 0 } |  |  | 0.554 |
| ns | 2006 |  | 178 | beets.library public export block (complete) | 2.1 |  | 0.535 |
| ns | 2183 |  | 177 | The `beet` subcommand roster (default_commands) | 2.2 |  | 0.515 |
| walker |  | 2270 | 383 | Fs::DirListing { dir: beetsplug } |  |  | 0.700 |
| walker |  | 2281 | 11 | Fs::DirListing { dir: beetsplug/bpd } |  |  | 0.700 |
| walker |  | 2293 | 12 | Fs::DirListing { dir: beetsplug/web } |  |  | 0.700 |
| walker |  | 2307 | 14 | Fs::DirListing { dir: beetsplug/discogs } |  |  | 0.700 |
| walker |  | 2323 | 16 | Fs::DirListing { dir: beetsplug/metasync } |  |  | 0.700 |
| walker |  | 2327 | 4 | Fs::DirListing { dir: beetsplug/web/templates } |  |  | 0.700 |
| walker |  | 2346 | 19 | Fs::DirListing { dir: beetsplug/tidal } |  |  | 0.700 |
| walker |  | 2369 | 23 | Fs::DirListing { dir: beetsplug/lastgenre } |  |  | 0.700 |
| ns | 2373 |  | 190 | Library class: models and schema migrations | 2.3 |  | 0.684 |
| walker |  | 2385 | 16 | Code::CodeKey { rung: ModuleDoc, file: beetsplug/web/__init__.py, decl: 0, sub: 0, line: 0 } |  |  | 0.684 |
| walker |  | 2402 | 17 | Code::CodeKey { rung: ModuleDoc, file: beetsplug/metasync/__init__.py, decl: 0, sub: 0, line: 0 } |  |  | 0.684 |
| walker |  | 2432 | 30 | Fs::DirListing { dir: beetsplug/_utils } |  |  | 0.684 |
| walker |  | 2471 | 39 | Code::CodeKey { rung: ModuleDoc, file: beetsplug/discogs/__init__.py, decl: 0, sub: 0, line: 0 } |  |  | 0.684 |
| ns | 2528 |  | 155 | Library method roster (complete) | 2.4 | 2.3 | 0.671 |
| walker |  | 2534 | 63 | Code::CodeKey { rung: ModuleDoc, file: beetsplug/bpd/__init__.py, decl: 0, sub: 0, line: 0 } |  |  | 0.671 |
| walker |  | 2556 | 22 | Fs::DirListing { dir: beetsplug/web/static } |  |  | 0.671 |
| walker |  | 2661 | 105 | Code::CodeKey { rung: ModuleDoc, file: beetsplug/lastgenre/__init__.py, decl: 0, sub: 0, line: 0 } |  |  | 0.671 |
| walker |  | 2872 | 211 | Code::CodeKey { rung: Names, file: beets/library/__init__.py, decl: 0, sub: 0, line: 0 } |  |  | 0.706 |
| walker |  | 2939 | 67 | Code::CodeKey { rung: Decl, file: beets/library/__init__.py, decl: 1, sub: 0, line: 8 } |  |  | 0.706 |
| ns | 2952 |  | 424 | models.py class headers: LibModel, FormattedItemMapping, Album, Item | 2.5 |  | 0.661 |
| ns | 3125 |  | 173 | beets.dbcore public export block + package docstring | 2.6 |  | 0.672 |
| walker |  | 3145 | 206 | Code::CodeKey { rung: Names, file: beetsplug/bpd/__init__.py, decl: 0, sub: 0, line: 0 } |  |  | 0.672 |
| ns | 3298 |  | 173 | config_default.yaml: library, directory, plugins, ignore rules | 3.1 |  | 0.654 |
| walker |  | 3589 | 444 | Code::CodeKey { rung: Names, file: beetsplug/web/__init__.py, decl: 0, sub: 0, line: 0 } |  |  | 0.654 |
| walker |  | 3597 | 8 | Code::CodeKey { rung: Decl, file: beetsplug/web/__init__.py, decl: 32, sub: 0, line: 439 } |  |  | 0.654 |
| walker |  | 3606 | 9 | Code::CodeKey { rung: Decl, file: beetsplug/web/__init__.py, decl: 18, sub: 0, line: 296 } |  |  | 0.654 |
| walker |  | 3616 | 10 | Code::CodeKey { rung: Decl, file: beetsplug/web/__init__.py, decl: 30, sub: 0, line: 412 } |  |  | 0.654 |
| walker |  | 3626 | 10 | Code::CodeKey { rung: Decl, file: beetsplug/web/__init__.py, decl: 31, sub: 0, line: 423 } |  |  | 0.654 |
| walker |  | 3640 | 14 | Code::CodeKey { rung: Decl, file: beetsplug/web/__init__.py, decl: 23, sub: 0, line: 344 } |  |  | 0.654 |
| walker |  | 3655 | 15 | Code::CodeKey { rung: Decl, file: beetsplug/web/__init__.py, decl: 24, sub: 0, line: 354 } |  |  | 0.654 |
| walker |  | 3670 | 15 | Code::CodeKey { rung: Decl, file: beetsplug/web/__init__.py, decl: 29, sub: 0, line: 397 } |  |  | 0.654 |
| walker |  | 3686 | 16 | Code::CodeKey { rung: Decl, file: beetsplug/web/__init__.py, decl: 21, sub: 0, line: 317 } |  |  | 0.654 |
| ns | 3687 |  | 389 | config_default.yaml: the complete `import:` option block | 3.2 |  | 0.621 |
| walker |  | 3703 | 17 | Code::CodeKey { rung: Decl, file: beetsplug/web/__init__.py, decl: 28, sub: 0, line: 388 } |  |  | 0.621 |
| walker |  | 3725 | 22 | Code::CodeKey { rung: Decl, file: beetsplug/web/__init__.py, decl: 16, sub: 0, line: 283 } |  |  | 0.621 |
| walker |  | 3747 | 22 | Code::CodeKey { rung: Decl, file: beetsplug/web/__init__.py, decl: 33, sub: 0, line: 447 } |  |  | 0.621 |
| walker |  | 3775 | 28 | Code::CodeKey { rung: Decl, file: beetsplug/web/__init__.py, decl: 10, sub: 0, line: 252 } |  |  | 0.621 |
| walker |  | 3803 | 28 | Code::CodeKey { rung: Decl, file: beetsplug/web/__init__.py, decl: 13, sub: 0, line: 268 } |  |  | 0.621 |
| walker |  | 3834 | 31 | Code::CodeKey { rung: Decl, file: beetsplug/web/__init__.py, decl: 20, sub: 0, line: 310 } |  |  | 0.621 |
| walker |  | 3865 | 31 | Code::CodeKey { rung: Decl, file: beetsplug/web/__init__.py, decl: 25, sub: 0, line: 369 } |  |  | 0.621 |
| ns | 3866 |  | 179 | config_default.yaml: tagging defaults, `paths:` templates, threading | 3.4 |  | 0.608 |
| walker |  | 3896 | 31 | Code::CodeKey { rung: Decl, file: beetsplug/web/__init__.py, decl: 26, sub: 0, line: 375 } |  |  | 0.608 |
| walker |  | 3928 | 32 | Code::CodeKey { rung: Decl, file: beetsplug/web/__init__.py, decl: 27, sub: 0, line: 382 } |  |  | 0.608 |
| walker |  | 3961 | 33 | Code::CodeKey { rung: Decl, file: beetsplug/web/__init__.py, decl: 36, sub: 0, line: 520 } |  |  | 0.608 |
| walker |  | 3999 | 38 | Code::CodeKey { rung: Decl, file: beetsplug/web/__init__.py, decl: 19, sub: 0, line: 304 } |  |  | 0.608 |
| walker |  | 4038 | 39 | Code::CodeKey { rung: Decl, file: beetsplug/web/__init__.py, decl: 22, sub: 0, line: 338 } |  |  | 0.608 |
| walker |  | 4053 | 15 | Code::CodeKey { rung: Doc, file: beetsplug/web/__init__.py, decl: 5, sub: 0, line: 117 } |  |  | 0.608 |
| ns | 4171 |  | 305 | Path-template function roster: DefaultTemplateFunctions (complete) | 3.5 |  | 0.594 |
| walker |  | 4264 | 211 | Code::CodeKey { rung: Names, file: beets/ui/commands/__init__.py, decl: 0, sub: 0, line: 0 } |  |  | 0.595 |
| ns | 4291 |  | 120 | config_default.yaml: display formats and default sorts | 3.6 |  | 0.588 |
| walker |  | 4371 | 107 | Code::CodeKey { rung: Decl, file: beets/ui/commands/__init__.py, decl: 2, sub: 0, line: 50 } |  |  | 0.603 |
| walker |  | 4381 | 10 | Code::CodeKey { rung: Doc, file: beets/ui/commands/__init__.py, decl: 1, sub: 0, line: 36 } |  |  | 0.603 |
| walker |  | 4571 | 190 | Code::CodeKey { rung: Names, file: beets/autotag/__init__.py, decl: 0, sub: 0, line: 0 } |  |  | 0.603 |
| walker |  | 4588 | 17 | Code::CodeKey { rung: Doc, file: beetsplug/web/__init__.py, decl: 3, sub: 0, line: 103 } |  |  | 0.603 |
| walker |  | 4605 | 17 | Code::CodeKey { rung: Doc, file: beetsplug/web/__init__.py, decl: 10, sub: 0, line: 252 } |  |  | 0.603 |
| ns | 4673 |  | 382 | config_default.yaml: autotagger thresholds and distance weights | 3.7 |  | 0.581 |
| ns | 4813 |  | 140 | config_default.yaml: terminal UI, colour names, import layout | 3.8 |  | 0.570 |
| walker |  | 4867 | 262 | Code::CodeKey { rung: Names, file: beetsplug/bpd/__init__.py, decl: 0, sub: 1, line: 0 } |  |  | 0.570 |
| walker |  | 4928 | 61 | Code::CodeKey { rung: Decl, file: beetsplug/bpd/__init__.py, decl: 25, sub: 0, line: 80 } |  |  | 0.570 |
| ns | 4986 |  | 173 | Album method roster (complete, names only) | 4.1 | 2.5 | 0.561 |
| walker |  | 4990 | 62 | Code::CodeKey { rung: Decl, file: beetsplug/bpd/__init__.py, decl: 27, sub: 0, line: 110 } |  |  | 0.561 |
| walker |  | 5105 | 115 | Code::CodeKey { rung: Decl, file: beetsplug/bpd/__init__.py, decl: 26, sub: 0, line: 90 } |  |  | 0.561 |
| walker |  | 5287 | 182 | Code::CodeKey { rung: Names, file: beetsplug/bpd/__init__.py, decl: 0, sub: 2, line: 0 } |  |  | 0.561 |
| ns | 5290 |  | 304 | Item method roster (complete, names only) | 4.3 | 2.5 | 0.547 |
| walker |  | 5303 | 16 | Code::CodeKey { rung: Decl, file: beetsplug/bpd/__init__.py, decl: 36, sub: 0, line: 180 } |  |  | 0.547 |
| walker |  | 5337 | 34 | Code::CodeKey { rung: Decl, file: beetsplug/bpd/__init__.py, decl: 118, sub: 0, line: 1064 } |  |  | 0.547 |
| walker |  | 5390 | 53 | Code::CodeKey { rung: Decl, file: beetsplug/bpd/__init__.py, decl: 168, sub: 0, line: 1583 } |  |  | 0.547 |
| ns | 5458 |  | 168 | Item search fields and MediaFile-backed field sets | 4.4 | 2.5 | 0.540 |
| walker |  | 5461 | 71 | Code::CodeKey { rung: Decl, file: beetsplug/bpd/__init__.py, decl: 94, sub: 0, line: 754 } |  |  | 0.540 |
| walker |  | 5469 | 8 | Code::CodeKey { rung: Decl, file: beetsplug/bpd/__init__.py, decl: 99, sub: 0, line: 785 } |  |  | 0.540 |
| walker |  | 5552 | 83 | Code::CodeKey { rung: Decl, file: beetsplug/bpd/__init__.py, decl: 100, sub: 0, line: 794 } |  |  | 0.540 |
| ns | 5629 |  | 171 | LibModel shared-method roster (complete after `_fields`) | 4.5 | 2.5 | 0.533 |
| walker |  | 5641 | 89 | Code::CodeKey { rung: Decl, file: beetsplug/bpd/__init__.py, decl: 107, sub: 0, line: 905 } |  |  | 0.533 |
| walker |  | 5744 | 103 | Code::CodeKey { rung: Decl, file: beetsplug/bpd/__init__.py, decl: 114, sub: 0, line: 960 } |  |  | 0.533 |
| walker |  | 5755 | 11 | Code::CodeKey { rung: Doc, file: beetsplug/bpd/__init__.py, decl: 112, sub: 0, line: 943 } |  |  | 0.533 |
| walker |  | 5768 | 13 | Code::CodeKey { rung: Doc, file: beetsplug/bpd/__init__.py, decl: 94, sub: 0, line: 754 } |  |  | 0.533 |
| walker |  | 5781 | 13 | Code::CodeKey { rung: Doc, file: beetsplug/bpd/__init__.py, decl: 170, sub: 0, line: 1601 } |  |  | 0.533 |
| ns | 5787 |  | 158 | beets.importer public exports + package docstring | 5.1 |  | 0.536 |
| walker |  | 5795 | 14 | Code::CodeKey { rung: Doc, file: beetsplug/bpd/__init__.py, decl: 96, sub: 0, line: 763 } |  |  | 0.536 |
| walker |  | 5809 | 14 | Code::CodeKey { rung: Doc, file: beetsplug/bpd/__init__.py, decl: 103, sub: 0, line: 816 } |  |  | 0.536 |
| walker |  | 5823 | 14 | Code::CodeKey { rung: Doc, file: beetsplug/bpd/__init__.py, decl: 105, sub: 0, line: 825 } |  |  | 0.536 |
| walker |  | 5837 | 14 | Code::CodeKey { rung: Doc, file: beetsplug/bpd/__init__.py, decl: 113, sub: 0, line: 950 } |  |  | 0.536 |
| walker |  | 5853 | 16 | Code::CodeKey { rung: Doc, file: beetsplug/bpd/__init__.py, decl: 100, sub: 0, line: 794 } |  |  | 0.536 |
| walker |  | 5869 | 16 | Code::CodeKey { rung: Doc, file: beetsplug/bpd/__init__.py, decl: 104, sub: 0, line: 821 } |  |  | 0.536 |
| walker |  | 5885 | 16 | Code::CodeKey { rung: Doc, file: beetsplug/bpd/__init__.py, decl: 111, sub: 0, line: 939 } |  |  | 0.536 |
| walker |  | 5901 | 16 | Code::CodeKey { rung: Doc, file: beetsplug/bpd/__init__.py, decl: 114, sub: 0, line: 960 } |  |  | 0.536 |
| walker |  | 5917 | 16 | Code::CodeKey { rung: Doc, file: beetsplug/bpd/__init__.py, decl: 120, sub: 0, line: 1079 } |  |  | 0.536 |
| walker |  | 5934 | 17 | Code::CodeKey { rung: Doc, file: beetsplug/bpd/__init__.py, decl: 107, sub: 0, line: 905 } |  |  | 0.536 |
| walker |  | 5952 | 18 | Code::CodeKey { rung: Doc, file: beetsplug/bpd/__init__.py, decl: 95, sub: 0, line: 757 } |  |  | 0.536 |
| walker |  | 5970 | 18 | Code::CodeKey { rung: Doc, file: beetsplug/bpd/__init__.py, decl: 101, sub: 0, line: 797 } |  |  | 0.536 |
| walker |  | 5988 | 18 | Code::CodeKey { rung: Doc, file: beetsplug/bpd/__init__.py, decl: 108, sub: 0, line: 908 } |  |  | 0.536 |
| walker |  | 6006 | 18 | Code::CodeKey { rung: Doc, file: beetsplug/web/__init__.py, decl: 9, sub: 0, line: 241 } |  |  | 0.536 |
| walker |  | 6024 | 18 | Code::CodeKey { rung: Doc, file: beetsplug/web/__init__.py, decl: 13, sub: 0, line: 268 } |  |  | 0.536 |
| ns | 6147 |  | 360 | importer/stages.py: complete stage-function roster with section banners | 5.2 |  | 0.525 |
| walker |  | 6210 | 186 | Code::CodeKey { rung: Names, file: beetsplug/metasync/__init__.py, decl: 0, sub: 0, line: 0 } |  |  | 0.525 |
| walker |  | 6239 | 29 | Code::CodeKey { rung: Decl, file: beetsplug/metasync/__init__.py, decl: 2, sub: 0, line: 34 } |  |  | 0.525 |
| walker |  | 6291 | 52 | Code::CodeKey { rung: Decl, file: beetsplug/metasync/__init__.py, decl: 3, sub: 0, line: 40 } |  |  | 0.525 |
| walker |  | 6300 | 9 | Code::CodeKey { rung: Decl, file: beetsplug/metasync/__init__.py, decl: 5, sub: 0, line: 47 } |  |  | 0.525 |
| walker |  | 6356 | 56 | Code::CodeKey { rung: Decl, file: beetsplug/metasync/__init__.py, decl: 9, sub: 0, line: 76 } |  |  | 0.525 |
| walker |  | 6361 | 5 | Code::CodeKey { rung: Body, file: beetsplug/metasync/__init__.py, decl: 5, sub: 0, line: 47 } |  |  | 0.525 |
| walker |  | 6375 | 14 | Code::CodeKey { rung: Doc, file: beetsplug/metasync/__init__.py, decl: 12, sub: 0, line: 104 } |  |  | 0.525 |
| ns | 6572 |  | 425 | ImportSession.run(): how the pipeline is assembled | 5.3 | 5.2 | 0.510 |
| walker |  | 6579 | 204 | Code::CodeKey { rung: Names, file: beets/ui/__init__.py, decl: 0, sub: 0, line: 0 } |  |  | 0.510 |
| walker |  | 6606 | 27 | Code::CodeKey { rung: Decl, file: beets/ui/__init__.py, decl: 2, sub: 0, line: 65 } |  |  | 0.510 |
| walker |  | 6618 | 12 | Code::CodeKey { rung: Body, file: beets/ui/__init__.py, decl: 4, sub: 0, line: 80 } |  |  | 0.510 |
| walker |  | 6632 | 14 | Code::CodeKey { rung: Body, file: beets/ui/__init__.py, decl: 5, sub: 0, line: 85 } |  |  | 0.510 |
| ns | 6829 |  | 257 | ImportSession class contract (attributes + the four decision hooks) | 5.4 | 5.3 | 0.501 |
| walker |  | 6846 | 214 | Code::CodeKey { rung: Names, file: beets/ui/__init__.py, decl: 0, sub: 1, line: 0 } |  |  | 0.501 |
| walker |  | 6853 | 7 | Code::CodeKey { rung: Decl, file: beets/ui/__init__.py, decl: 18, sub: 0, line: 459 } |  |  | 0.501 |
| walker |  | 6917 | 64 | Code::CodeKey { rung: Decl, file: beets/ui/__init__.py, decl: 13, sub: 0, line: 207 } |  |  | 0.501 |
| walker |  | 6989 | 72 | Code::CodeKey { rung: Decl, file: beets/ui/__init__.py, decl: 19, sub: 0, line: 466 } |  |  | 0.501 |
| walker |  | 7102 | 113 | Code::CodeKey { rung: Decl, file: beets/ui/__init__.py, decl: 20, sub: 0, line: 498 } |  |  | 0.501 |
| ns | 7103 |  | 274 | importer/tasks.py: Action enum and the complete task class roster | 5.5 |  | 0.492 |
| walker |  | 7169 | 67 | Code::CodeKey { rung: Decl, file: beets/ui/__init__.py, decl: 23, sub: 0, line: 534 } |  |  | 0.492 |
| walker |  | 7183 | 14 | Code::CodeKey { rung: Doc, file: beets/ui/__init__.py, decl: 26, sub: 0, line: 621 } |  |  | 0.492 |
| walker |  | 7199 | 16 | Code::CodeKey { rung: Doc, file: beets/ui/__init__.py, decl: 17, sub: 0, line: 445 } |  |  | 0.492 |
| walker |  | 7215 | 16 | Code::CodeKey { rung: Doc, file: beets/ui/__init__.py, decl: 18, sub: 0, line: 459 } |  |  | 0.492 |
| walker |  | 7338 | 123 | Code::CodeKey { rung: Names, file: beets/ui/__init__.py, decl: 0, sub: 2, line: 0 } |  |  | 0.492 |
| ns | 7349 |  | 246 | BeetsPlugin base class: docstring and declared attributes | 6.1 |  | 0.485 |
| walker |  | 7364 | 26 | Code::CodeKey { rung: Decl, file: beets/ui/__init__.py, decl: 40, sub: 0, line: 807 } |  |  | 0.485 |
| walker |  | 7462 | 98 | Code::CodeKey { rung: Decl, file: beets/ui/__init__.py, decl: 33, sub: 0, line: 676 } |  |  | 0.485 |
| walker |  | 7567 | 105 | Code::CodeKey { rung: Decl, file: beets/ui/__init__.py, decl: 27, sub: 0, line: 637 } |  |  | 0.485 |
| walker |  | 7575 | 8 | Code::CodeKey { rung: Decl, file: beets/ui/__init__.py, decl: 31, sub: 0, line: 664 } |  |  | 0.485 |
| walker |  | 7585 | 10 | Code::CodeKey { rung: Decl, file: beets/ui/__init__.py, decl: 32, sub: 0, line: 668 } |  |  | 0.485 |
| ns | 7595 |  | 246 | BeetsPlugin method roster (complete) | 6.2 | 6.1 | 0.480 |
| walker |  | 7600 | 15 | Code::CodeKey { rung: Doc, file: beets/ui/__init__.py, decl: 43, sub: 0, line: 879 } |  |  | 0.480 |
| walker |  | 7617 | 17 | Code::CodeKey { rung: Doc, file: beets/ui/__init__.py, decl: 41, sub: 0, line: 830 } |  |  | 0.480 |
| walker |  | 7689 | 72 | Code::CodeKey { rung: Names, file: beets/__init__.py, decl: 0, sub: 0, line: 0 } |  |  | 0.480 |
| walker |  | 7715 | 26 | Code::CodeKey { rung: Decl, file: beets/__init__.py, decl: 2, sub: 0, line: 35 } |  |  | 0.480 |
| walker |  | 7725 | 10 | Code::CodeKey { rung: Doc, file: beets/__init__.py, decl: 1, sub: 0, line: 26 } |  |  | 0.480 |
| walker |  | 7744 | 19 | Code::CodeKey { rung: Doc, file: beets/ui/__init__.py, decl: 4, sub: 0, line: 80 } |  |  | 0.480 |
| walker |  | 7763 | 19 | Code::CodeKey { rung: Doc, file: beets/ui/__init__.py, decl: 5, sub: 0, line: 85 } |  |  | 0.480 |
| walker |  | 7782 | 19 | Code::CodeKey { rung: Doc, file: beets/ui/__init__.py, decl: 35, sub: 0, line: 701 } |  |  | 0.480 |
| walker |  | 7801 | 19 | Code::CodeKey { rung: Doc, file: beetsplug/bpd/__init__.py, decl: 30, sub: 0, line: 138 } |  |  | 0.480 |
| walker |  | 7820 | 19 | Code::CodeKey { rung: Doc, file: beetsplug/bpd/__init__.py, decl: 110, sub: 0, line: 915 } |  |  | 0.480 |
| walker |  | 7839 | 19 | Code::CodeKey { rung: Doc, file: beetsplug/metasync/__init__.py, decl: 8, sub: 0, line: 68 } |  |  | 0.480 |
| walker |  | 7858 | 19 | Code::CodeKey { rung: Doc, file: beetsplug/web/__init__.py, decl: 7, sub: 0, line: 176 } |  |  | 0.480 |
| walker |  | 7877 | 19 | Code::CodeKey { rung: Body, file: beets/library/__init__.py, decl: 2, sub: 0, line: 15 } |  |  | 0.480 |
| ns | 8017 |  | 422 | The complete `EventType` literal — every plugin event name | 6.3 | 6.1 | 0.467 |
| walker |  | 8032 | 155 | Code::CodeKey { rung: Names, file: beetsplug/tidal/__init__.py, decl: 0, sub: 0, line: 0 } |  |  | 0.467 |
| walker |  | 8111 | 79 | Code::CodeKey { rung: Decl, file: beetsplug/tidal/__init__.py, decl: 34, sub: 0, line: 494 } |  |  | 0.467 |
| walker |  | 8288 | 177 | Code::CodeKey { rung: Decl, file: beetsplug/tidal/__init__.py, decl: 2, sub: 0, line: 37 } |  |  | 0.467 |
| walker |  | 8297 | 9 | Code::CodeKey { rung: Decl, file: beetsplug/tidal/__init__.py, decl: 4, sub: 0, line: 53 } |  |  | 0.467 |
| walker |  | 8312 | 15 | Code::CodeKey { rung: Body, file: beetsplug/tidal/__init__.py, decl: 9, sub: 0, line: 103 } |  |  | 0.467 |
| ns | 8321 |  | 304 | plugins.py module-level API roster (complete) | 6.4 |  | 0.461 |
| walker |  | 8331 | 19 | Code::CodeKey { rung: Doc, file: beetsplug/tidal/__init__.py, decl: 5, sub: 0, line: 60 } |  |  | 0.461 |
| walker |  | 8351 | 20 | Code::CodeKey { rung: Doc, file: beets/ui/__init__.py, decl: 9, sub: 0, line: 150 } |  |  | 0.461 |
| walker |  | 8371 | 20 | Code::CodeKey { rung: Doc, file: beetsplug/web/__init__.py, decl: 6, sub: 0, line: 122 } |  |  | 0.461 |
| walker |  | 8403 | 32 | Code::CodeKey { rung: Names, file: beetsplug/_utils/__init__.py, decl: 0, sub: 0, line: 0 } |  |  | 0.461 |
| ns | 8633 |  | 312 | MetadataSourcePlugin: the metadata-backend contract | 6.5 |  | 0.455 |
| walker |  | 8712 | 309 | Code::CodeKey { rung: Names, file: beetsplug/discogs/__init__.py, decl: 0, sub: 0, line: 0 } |  |  | 0.455 |
| walker |  | 8768 | 56 | Code::CodeKey { rung: Decl, file: beetsplug/discogs/__init__.py, decl: 4, sub: 0, line: 60 } |  |  | 0.455 |
| ns | 8830 |  | 197 | Query-string prefixes: how `beet ls artist:foo` is parsed | 7.1 |  | 0.450 |
| walker |  | 8839 | 71 | Code::CodeKey { rung: Decl, file: beetsplug/discogs/__init__.py, decl: 6, sub: 0, line: 83 } |  |  | 0.450 |
| walker |  | 9029 | 190 | Code::CodeKey { rung: Decl, file: beetsplug/discogs/__init__.py, decl: 5, sub: 0, line: 68 } |  |  | 0.450 |
| ns | 9187 |  | 357 | dbcore/query.py class roster (complete) | 7.2 |  | 0.441 |
| walker |  | 9449 | 420 | Code::CodeKey { rung: Decl, file: beetsplug/discogs/__init__.py, decl: 7, sub: 0, line: 93 } |  |  | 0.441 |
| walker |  | 9457 | 8 | Code::CodeKey { rung: Decl, file: beetsplug/discogs/__init__.py, decl: 20, sub: 0, line: 314 } |  |  | 0.441 |
| walker |  | 9466 | 9 | Code::CodeKey { rung: Decl, file: beetsplug/discogs/__init__.py, decl: 9, sub: 0, line: 120 } |  |  | 0.441 |
| walker |  | 9497 | 31 | Code::CodeKey { rung: Decl, file: beetsplug/discogs/__init__.py, decl: 15, sub: 0, line: 217 } |  |  | 0.441 |
| walker |  | 9531 | 34 | Code::CodeKey { rung: Decl, file: beetsplug/discogs/__init__.py, decl: 14, sub: 0, line: 203 } |  |  | 0.441 |
| ns | 9539 |  | 352 | beets.autotag exports, Recommendation, Proposal | 8.1 |  | 0.434 |
| walker |  | 9568 | 37 | Code::CodeKey { rung: Decl, file: beetsplug/discogs/__init__.py, decl: 29, sub: 0, line: 705 } |  |  | 0.434 |
| walker |  | 9607 | 39 | Code::CodeKey { rung: Decl, file: beetsplug/discogs/__init__.py, decl: 21, sub: 0, line: 341 } |  |  | 0.434 |
| walker |  | 9647 | 40 | Code::CodeKey { rung: Decl, file: beetsplug/discogs/__init__.py, decl: 26, sub: 0, line: 615 } |  |  | 0.434 |
| walker |  | 9690 | 43 | Code::CodeKey { rung: Decl, file: beetsplug/discogs/__init__.py, decl: 24, sub: 0, line: 488 } |  |  | 0.434 |
| walker |  | 9757 | 67 | Code::CodeKey { rung: Decl, file: beetsplug/discogs/__init__.py, decl: 28, sub: 0, line: 666 } |  |  | 0.434 |
| ns | 9775 |  | 236 | autotag/hooks.py and distance.py symbol roster | 8.2 |  | 0.429 |
| walker |  | 9831 | 74 | Code::CodeKey { rung: Decl, file: beetsplug/discogs/__init__.py, decl: 18, sub: 0, line: 264 } |  |  | 0.429 |
| walker |  | 9846 | 15 | Code::CodeKey { rung: Doc, file: beetsplug/discogs/__init__.py, decl: 11, sub: 0, line: 165 } |  |  | 0.429 |
| walker |  | 9861 | 15 | Code::CodeKey { rung: Doc, file: beetsplug/discogs/__init__.py, decl: 14, sub: 0, line: 203 } |  |  | 0.429 |
| walker |  | 9878 | 17 | Code::CodeKey { rung: Doc, file: beetsplug/discogs/__init__.py, decl: 28, sub: 0, line: 666 } |  |  | 0.429 |
| walker |  | 9896 | 18 | Code::CodeKey { rung: Doc, file: beetsplug/discogs/__init__.py, decl: 19, sub: 0, line: 308 } |  |  | 0.429 |
| walker |  | 9914 | 18 | Code::CodeKey { rung: Doc, file: beetsplug/discogs/__init__.py, decl: 22, sub: 0, line: 353 } |  |  | 0.429 |
| walker |  | 9932 | 18 | Code::CodeKey { rung: Doc, file: beetsplug/discogs/__init__.py, decl: 30, sub: 0, line: 722 } |  |  | 0.429 |
| walker |  | 9951 | 19 | Code::CodeKey { rung: Doc, file: beetsplug/discogs/__init__.py, decl: 10, sub: 0, line: 139 } |  |  | 0.429 |
| walker |  | 9970 | 19 | Code::CodeKey { rung: Doc, file: beetsplug/discogs/__init__.py, decl: 12, sub: 0, line: 170 } |  |  | 0.424 |
| ns | 9970 |  | 195 | beets/util/__init__.py: classes and the core path/file-operation helpers | 9.1 |  | 0.424 |
| walker |  | 9990 | 20 | Code::CodeKey { rung: Doc, file: beetsplug/discogs/__init__.py, decl: 24, sub: 0, line: 488 } |  |  | 0.424 |
