Score(3000)=0.517 I=0.519 C=0.516 ns_rows≤3K=17/44 grid(1000/1442/2080/3000/4327/6240/9000)=0.593/0.590/0.582/0.517/0.440/0.383/0.328

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
| walker |  | 690 | 67 | Fs::DirListing { dir: beets/ui/commands } |  |  | 0.615 |
| ns | 690 |  | 148 | README capability bullets, part 1 (plugin framing + metadata sources) | 1.7 |  | 0.615 |
| walker |  | 704 | 14 | Fs::DirListing { dir: beets/ui/commands/import_ } |  |  | 0.615 |
| walker |  | 724 | 20 | Code::CodeKey { rung: ModuleDoc, file: beets/ui/commands/import_/__init__.py, decl: 0, sub: 0, line: 0 } |  |  | 0.615 |
| walker |  | 757 | 33 | Code::CodeKey { rung: ModuleDoc, file: beets/ui/commands/__init__.py, decl: 0, sub: 0, line: 0 } |  |  | 0.615 |
| walker |  | 832 | 75 | Fs::DirListing { dir: beets/util } |  |  | 0.635 |
| walker |  | 846 | 14 | Code::CodeKey { rung: ModuleDoc, file: beets/util/__init__.py, decl: 0, sub: 0, line: 0 } |  |  | 0.635 |
| ns | 885 |  | 195 | README capability bullets, part 2 (files, art, web, MPD) | 1.8 |  | 0.589 |
| walker |  | 923 | 77 | Fs::DirListing { dir: docs } |  |  | 0.593 |
| walker |  | 927 | 4 | Fs::DirListing { dir: docs/_templates } |  |  | 0.593 |
| walker |  | 931 | 4 | Fs::DirListing { dir: docs/extensions } |  |  | 0.593 |
| walker |  | 945 | 14 | Fs::DirListing { dir: docs/_static } |  |  | 0.593 |
| walker |  | 967 | 22 | Fs::DirListing { dir: docs/api } |  |  | 0.593 |
| walker |  | 993 | 26 | Fs::DirListing { dir: docs/guides } |  |  | 0.593 |
| walker |  | 1019 | 26 | Fs::DirListing { dir: docs/reference } |  |  | 0.593 |
| walker |  | 1047 | 28 | Fs::DirListing { dir: docs/dev } |  |  | 0.593 |
| ns | 1051 |  | 166 | UI, util and test-helper subpackage listings | 1.9 |  | 0.604 |
| walker |  | 1074 | 27 | Toml::Operational { file: pyproject.toml } |  |  | 0.606 |
| walker |  | 1095 | 21 | Fs::DirListing { dir: docs/_templates/autosummary } |  |  | 0.606 |
| walker |  | 1119 | 24 | Fs::DirListing { dir: docs/dev/plugins } |  |  | 0.606 |
| walker |  | 1134 | 15 | Fs::DirListing { dir: beets/test } |  |  | 0.629 |
| walker |  | 1173 | 39 | Fs::DirListing { dir: .github } |  |  | 0.629 |
| walker |  | 1204 | 31 | Fs::DirListing { dir: .github/workflows } |  |  | 0.629 |
| ns | 1227 |  | 176 | Canonical test / lint / typecheck commands | 1.10 |  | 0.590 |
| walker |  | 1245 | 41 | Fs::DirListing { dir: docs/dev/plugins/other } |  |  | 0.590 |
| ns | 1445 |  | 218 | test/ and docs/ top-level listings (complete) | 1.11 |  | 0.524 |
| walker |  | 1456 | 211 | Code::CodeKey { rung: Names, file: beets/ui/commands/__init__.py, decl: 0, sub: 0, line: 0 } |  |  | 0.524 |
| walker |  | 1466 | 10 | Code::CodeKey { rung: Doc, file: beets/ui/commands/__init__.py, decl: 1, sub: 0, line: 36 } |  |  | 0.524 |
| walker |  | 1573 | 107 | Code::CodeKey { rung: Decl, file: beets/ui/commands/__init__.py, decl: 2, sub: 0, line: 50 } |  |  | 0.527 |
| walker |  | 1651 | 78 | Code::CodeKey { rung: Body, file: beets/ui/commands/__init__.py, decl: 1, sub: 0, line: 36 } |  |  | 0.527 |
| ns | 1828 |  | 383 | beetsplug/ bundled plugin listing (complete, 81 entries) | 1.12 |  | 0.432 |
| ns | 2006 |  | 178 | beets.library public export block (complete) | 2.1 |  | 0.417 |
| walker |  | 2034 | 383 | Fs::DirListing { dir: beetsplug } |  |  | 0.582 |
| walker |  | 2045 | 11 | Fs::DirListing { dir: beetsplug/bpd } |  |  | 0.582 |
| walker |  | 2057 | 12 | Fs::DirListing { dir: beetsplug/web } |  |  | 0.582 |
| walker |  | 2073 | 16 | Code::CodeKey { rung: ModuleDoc, file: beetsplug/web/__init__.py, decl: 0, sub: 0, line: 0 } |  |  | 0.582 |
| walker |  | 2087 | 14 | Fs::DirListing { dir: beetsplug/discogs } |  |  | 0.582 |
| walker |  | 2103 | 16 | Fs::DirListing { dir: beetsplug/metasync } |  |  | 0.582 |
| walker |  | 2120 | 17 | Code::CodeKey { rung: ModuleDoc, file: beetsplug/metasync/__init__.py, decl: 0, sub: 0, line: 0 } |  |  | 0.582 |
| walker |  | 2124 | 4 | Fs::DirListing { dir: beetsplug/web/templates } |  |  | 0.582 |
| walker |  | 2143 | 19 | Fs::DirListing { dir: beetsplug/tidal } |  |  | 0.582 |
| walker |  | 2166 | 23 | Fs::DirListing { dir: beetsplug/lastgenre } |  |  | 0.582 |
| ns | 2183 |  | 177 | The `beet` subcommand roster (default_commands) | 2.2 |  | 0.577 |
| walker |  | 2205 | 39 | Code::CodeKey { rung: ModuleDoc, file: beetsplug/discogs/__init__.py, decl: 0, sub: 0, line: 0 } |  |  | 0.577 |
| walker |  | 2235 | 30 | Fs::DirListing { dir: beetsplug/_utils } |  |  | 0.577 |
| walker |  | 2298 | 63 | Code::CodeKey { rung: ModuleDoc, file: beetsplug/bpd/__init__.py, decl: 0, sub: 0, line: 0 } |  |  | 0.577 |
| walker |  | 2320 | 22 | Code::CodeKey { rung: ModuleDoc, file: beetsplug/zero.py, decl: 0, sub: 0, line: 0 } |  |  | 0.577 |
| walker |  | 2343 | 23 | Code::CodeKey { rung: ModuleDoc, file: beetsplug/subsonicupdate.py, decl: 0, sub: 0, line: 0 } |  |  | 0.577 |
| ns | 2373 |  | 190 | Library class: models and schema migrations | 2.3 |  | 0.563 |
| walker |  | 2448 | 105 | Code::CodeKey { rung: ModuleDoc, file: beetsplug/lastgenre/__init__.py, decl: 0, sub: 0, line: 0 } |  |  | 0.563 |
| walker |  | 2472 | 24 | Code::CodeKey { rung: ModuleDoc, file: beetsplug/subsonicplaylist.py, decl: 0, sub: 0, line: 0 } |  |  | 0.563 |
| walker |  | 2496 | 24 | Code::CodeKey { rung: ModuleDoc, file: beetsplug/unimported.py, decl: 0, sub: 0, line: 0 } |  |  | 0.563 |
| walker |  | 2518 | 22 | Fs::DirListing { dir: beetsplug/web/static } |  |  | 0.563 |
| ns | 2528 |  | 155 | Library method roster (complete) | 2.4 | 2.3 | 0.553 |
| walker |  | 2552 | 34 | Code::CodeKey { rung: ModuleDoc, file: beetsplug/titlecase.py, decl: 0, sub: 0, line: 0 } |  |  | 0.553 |
| walker |  | 2587 | 35 | Code::CodeKey { rung: ModuleDoc, file: beetsplug/ihate.py, decl: 0, sub: 0, line: 0 } |  |  | 0.553 |
| walker |  | 2622 | 35 | Code::CodeKey { rung: ModuleDoc, file: beetsplug/the.py, decl: 0, sub: 0, line: 0 } |  |  | 0.553 |
| ns | 2952 |  | 424 | models.py class headers: LibModel, FormattedItemMapping, Album, Item | 2.5 |  | 0.517 |
| walker |  | 3066 | 444 | Code::CodeKey { rung: Names, file: beetsplug/web/__init__.py, decl: 0, sub: 0, line: 0 } |  |  | 0.517 |
| walker |  | 3074 | 8 | Code::CodeKey { rung: Decl, file: beetsplug/web/__init__.py, decl: 32, sub: 0, line: 439 } |  |  | 0.517 |
| walker |  | 3083 | 9 | Code::CodeKey { rung: Decl, file: beetsplug/web/__init__.py, decl: 18, sub: 0, line: 296 } |  |  | 0.517 |
| walker |  | 3093 | 10 | Code::CodeKey { rung: Decl, file: beetsplug/web/__init__.py, decl: 30, sub: 0, line: 412 } |  |  | 0.517 |
| walker |  | 3103 | 10 | Code::CodeKey { rung: Decl, file: beetsplug/web/__init__.py, decl: 31, sub: 0, line: 423 } |  |  | 0.517 |
| walker |  | 3117 | 14 | Code::CodeKey { rung: Decl, file: beetsplug/web/__init__.py, decl: 23, sub: 0, line: 344 } |  |  | 0.517 |
| ns | 3125 |  | 173 | beets.dbcore public export block + package docstring | 2.6 |  | 0.503 |
| walker |  | 3132 | 15 | Code::CodeKey { rung: Decl, file: beetsplug/web/__init__.py, decl: 24, sub: 0, line: 354 } |  |  | 0.503 |
| walker |  | 3147 | 15 | Code::CodeKey { rung: Decl, file: beetsplug/web/__init__.py, decl: 29, sub: 0, line: 397 } |  |  | 0.503 |
| walker |  | 3163 | 16 | Code::CodeKey { rung: Decl, file: beetsplug/web/__init__.py, decl: 21, sub: 0, line: 317 } |  |  | 0.503 |
| walker |  | 3180 | 17 | Code::CodeKey { rung: Decl, file: beetsplug/web/__init__.py, decl: 28, sub: 0, line: 388 } |  |  | 0.503 |
| walker |  | 3202 | 22 | Code::CodeKey { rung: Decl, file: beetsplug/web/__init__.py, decl: 16, sub: 0, line: 283 } |  |  | 0.503 |
| walker |  | 3224 | 22 | Code::CodeKey { rung: Decl, file: beetsplug/web/__init__.py, decl: 33, sub: 0, line: 447 } |  |  | 0.503 |
| walker |  | 3252 | 28 | Code::CodeKey { rung: Decl, file: beetsplug/web/__init__.py, decl: 10, sub: 0, line: 252 } |  |  | 0.503 |
| walker |  | 3280 | 28 | Code::CodeKey { rung: Decl, file: beetsplug/web/__init__.py, decl: 13, sub: 0, line: 268 } |  |  | 0.503 |
| ns | 3298 |  | 173 | config_default.yaml: library, directory, plugins, ignore rules | 3.1 |  | 0.490 |
| walker |  | 3311 | 31 | Code::CodeKey { rung: Decl, file: beetsplug/web/__init__.py, decl: 20, sub: 0, line: 310 } |  |  | 0.490 |
| walker |  | 3342 | 31 | Code::CodeKey { rung: Decl, file: beetsplug/web/__init__.py, decl: 25, sub: 0, line: 369 } |  |  | 0.490 |
| walker |  | 3373 | 31 | Code::CodeKey { rung: Decl, file: beetsplug/web/__init__.py, decl: 26, sub: 0, line: 375 } |  |  | 0.490 |
| walker |  | 3405 | 32 | Code::CodeKey { rung: Decl, file: beetsplug/web/__init__.py, decl: 27, sub: 0, line: 382 } |  |  | 0.490 |
| walker |  | 3438 | 33 | Code::CodeKey { rung: Decl, file: beetsplug/web/__init__.py, decl: 36, sub: 0, line: 520 } |  |  | 0.490 |
| walker |  | 3476 | 38 | Code::CodeKey { rung: Decl, file: beetsplug/web/__init__.py, decl: 19, sub: 0, line: 304 } |  |  | 0.490 |
| walker |  | 3515 | 39 | Code::CodeKey { rung: Decl, file: beetsplug/web/__init__.py, decl: 22, sub: 0, line: 338 } |  |  | 0.490 |
| walker |  | 3530 | 15 | Code::CodeKey { rung: Doc, file: beetsplug/web/__init__.py, decl: 5, sub: 0, line: 117 } |  |  | 0.490 |
| walker |  | 3547 | 17 | Code::CodeKey { rung: Doc, file: beetsplug/web/__init__.py, decl: 3, sub: 0, line: 103 } |  |  | 0.490 |
| walker |  | 3564 | 17 | Code::CodeKey { rung: Doc, file: beetsplug/web/__init__.py, decl: 10, sub: 0, line: 252 } |  |  | 0.490 |
| walker |  | 3582 | 18 | Code::CodeKey { rung: Doc, file: beetsplug/web/__init__.py, decl: 13, sub: 0, line: 268 } |  |  | 0.490 |
| walker |  | 3601 | 19 | Code::CodeKey { rung: Doc, file: beetsplug/web/__init__.py, decl: 7, sub: 0, line: 176 } |  |  | 0.490 |
| walker |  | 3621 | 20 | Code::CodeKey { rung: Doc, file: beetsplug/web/__init__.py, decl: 6, sub: 0, line: 122 } |  |  | 0.490 |
| walker |  | 3651 | 30 | Code::CodeKey { rung: Doc, file: beetsplug/web/__init__.py, decl: 4, sub: 0, line: 109 } |  |  | 0.490 |
| walker |  | 3685 | 34 | Code::CodeKey { rung: Doc, file: beetsplug/web/__init__.py, decl: 8, sub: 0, line: 223 } |  |  | 0.490 |
| ns | 3687 |  | 389 | config_default.yaml: the complete `import:` option block | 3.2 |  | 0.465 |
| walker |  | 3703 | 18 | Code::CodeKey { rung: Doc, file: beetsplug/web/__init__.py, decl: 9, sub: 0, line: 241 } |  |  | 0.465 |
| ns | 3866 |  | 179 | config_default.yaml: tagging defaults, `paths:` templates, threading | 3.4 |  | 0.455 |
| walker |  | 3960 | 257 | Code::CodeKey { rung: Names, file: beetsplug/bpd/__init__.py, decl: 0, sub: 0, line: 0 } |  |  | 0.455 |
| ns | 4171 |  | 305 | Path-template function roster: DefaultTemplateFunctions (complete) | 3.5 |  | 0.445 |
| ns | 4291 |  | 120 | config_default.yaml: display formats and default sorts | 3.6 |  | 0.440 |
| walker |  | 4353 | 393 | Code::CodeKey { rung: Names, file: beetsplug/bpd/__init__.py, decl: 0, sub: 1, line: 0 } |  |  | 0.440 |
| walker |  | 4369 | 16 | Code::CodeKey { rung: Decl, file: beetsplug/bpd/__init__.py, decl: 36, sub: 0, line: 180 } |  |  | 0.440 |
| walker |  | 4403 | 34 | Code::CodeKey { rung: Decl, file: beetsplug/bpd/__init__.py, decl: 118, sub: 0, line: 1064 } |  |  | 0.440 |
| walker |  | 4456 | 53 | Code::CodeKey { rung: Decl, file: beetsplug/bpd/__init__.py, decl: 168, sub: 0, line: 1583 } |  |  | 0.440 |
| walker |  | 4517 | 61 | Code::CodeKey { rung: Decl, file: beetsplug/bpd/__init__.py, decl: 25, sub: 0, line: 80 } |  |  | 0.440 |
| walker |  | 4579 | 62 | Code::CodeKey { rung: Decl, file: beetsplug/bpd/__init__.py, decl: 27, sub: 0, line: 110 } |  |  | 0.440 |
| walker |  | 4650 | 71 | Code::CodeKey { rung: Decl, file: beetsplug/bpd/__init__.py, decl: 94, sub: 0, line: 754 } |  |  | 0.440 |
| walker |  | 4658 | 8 | Code::CodeKey { rung: Decl, file: beetsplug/bpd/__init__.py, decl: 99, sub: 0, line: 785 } |  |  | 0.440 |
| walker |  | 4671 | 13 | Code::CodeKey { rung: Doc, file: beetsplug/bpd/__init__.py, decl: 94, sub: 0, line: 754 } |  |  | 0.440 |
| ns | 4673 |  | 382 | config_default.yaml: autotagger thresholds and distance weights | 3.7 |  | 0.424 |
| walker |  | 4684 | 13 | Code::CodeKey { rung: Doc, file: beetsplug/bpd/__init__.py, decl: 170, sub: 0, line: 1601 } |  |  | 0.424 |
| walker |  | 4698 | 14 | Code::CodeKey { rung: Doc, file: beetsplug/bpd/__init__.py, decl: 96, sub: 0, line: 763 } |  |  | 0.424 |
| walker |  | 4781 | 83 | Code::CodeKey { rung: Decl, file: beetsplug/bpd/__init__.py, decl: 100, sub: 0, line: 794 } |  |  | 0.424 |
| walker |  | 4795 | 14 | Code::CodeKey { rung: Doc, file: beetsplug/bpd/__init__.py, decl: 103, sub: 0, line: 816 } |  |  | 0.424 |
| walker |  | 4809 | 14 | Code::CodeKey { rung: Doc, file: beetsplug/bpd/__init__.py, decl: 105, sub: 0, line: 825 } |  |  | 0.424 |
| ns | 4813 |  | 140 | config_default.yaml: terminal UI, colour names, import layout | 3.8 |  | 0.417 |
| walker |  | 4898 | 89 | Code::CodeKey { rung: Decl, file: beetsplug/bpd/__init__.py, decl: 107, sub: 0, line: 905 } |  |  | 0.417 |
| walker |  | 4909 | 11 | Code::CodeKey { rung: Doc, file: beetsplug/bpd/__init__.py, decl: 112, sub: 0, line: 943 } |  |  | 0.417 |
| walker |  | 4923 | 14 | Code::CodeKey { rung: Doc, file: beetsplug/bpd/__init__.py, decl: 113, sub: 0, line: 950 } |  |  | 0.417 |
| walker |  | 4939 | 16 | Code::CodeKey { rung: Doc, file: beetsplug/bpd/__init__.py, decl: 100, sub: 0, line: 794 } |  |  | 0.417 |
| walker |  | 4955 | 16 | Code::CodeKey { rung: Doc, file: beetsplug/bpd/__init__.py, decl: 104, sub: 0, line: 821 } |  |  | 0.417 |
| walker |  | 4971 | 16 | Code::CodeKey { rung: Doc, file: beetsplug/bpd/__init__.py, decl: 111, sub: 0, line: 939 } |  |  | 0.417 |
| ns | 4986 |  | 173 | Album method roster (complete, names only) | 4.1 | 2.5 | 0.410 |
| walker |  | 4987 | 16 | Code::CodeKey { rung: Doc, file: beetsplug/bpd/__init__.py, decl: 120, sub: 0, line: 1079 } |  |  | 0.410 |
| walker |  | 5004 | 17 | Code::CodeKey { rung: Doc, file: beetsplug/bpd/__init__.py, decl: 107, sub: 0, line: 905 } |  |  | 0.410 |
| walker |  | 5107 | 103 | Code::CodeKey { rung: Decl, file: beetsplug/bpd/__init__.py, decl: 114, sub: 0, line: 960 } |  |  | 0.410 |
| walker |  | 5123 | 16 | Code::CodeKey { rung: Doc, file: beetsplug/bpd/__init__.py, decl: 114, sub: 0, line: 960 } |  |  | 0.410 |
| walker |  | 5141 | 18 | Code::CodeKey { rung: Doc, file: beetsplug/bpd/__init__.py, decl: 95, sub: 0, line: 757 } |  |  | 0.410 |
| walker |  | 5159 | 18 | Code::CodeKey { rung: Doc, file: beetsplug/bpd/__init__.py, decl: 101, sub: 0, line: 797 } |  |  | 0.410 |
| walker |  | 5177 | 18 | Code::CodeKey { rung: Doc, file: beetsplug/bpd/__init__.py, decl: 108, sub: 0, line: 908 } |  |  | 0.410 |
| walker |  | 5196 | 19 | Code::CodeKey { rung: Doc, file: beetsplug/bpd/__init__.py, decl: 30, sub: 0, line: 138 } |  |  | 0.410 |
| walker |  | 5215 | 19 | Code::CodeKey { rung: Doc, file: beetsplug/bpd/__init__.py, decl: 110, sub: 0, line: 915 } |  |  | 0.410 |
| ns | 5290 |  | 304 | Item method roster (complete, names only) | 4.3 | 2.5 | 0.399 |
| walker |  | 5330 | 115 | Code::CodeKey { rung: Decl, file: beetsplug/bpd/__init__.py, decl: 26, sub: 0, line: 90 } |  |  | 0.399 |
| walker |  | 5355 | 25 | Code::CodeKey { rung: Doc, file: beetsplug/bpd/__init__.py, decl: 102, sub: 0, line: 804 } |  |  | 0.399 |
| walker |  | 5385 | 30 | Code::CodeKey { rung: Doc, file: beetsplug/bpd/__init__.py, decl: 35, sub: 0, line: 174 } |  |  | 0.399 |
| walker |  | 5416 | 31 | Code::CodeKey { rung: Doc, file: beetsplug/bpd/__init__.py, decl: 27, sub: 0, line: 110 } |  |  | 0.399 |
| walker |  | 5448 | 32 | Code::CodeKey { rung: Doc, file: beetsplug/bpd/__init__.py, decl: 106, sub: 0, line: 838 } |  |  | 0.399 |
| ns | 5458 |  | 168 | Item search fields and MediaFile-backed field sets | 4.4 | 2.5 | 0.394 |
| walker |  | 5480 | 32 | Code::CodeKey { rung: Doc, file: beetsplug/bpd/__init__.py, decl: 117, sub: 0, line: 1017 } |  |  | 0.394 |
| walker |  | 5514 | 34 | Code::CodeKey { rung: Doc, file: beetsplug/bpd/__init__.py, decl: 29, sub: 0, line: 123 } |  |  | 0.394 |
| walker |  | 5549 | 35 | Code::CodeKey { rung: Doc, file: beetsplug/bpd/__init__.py, decl: 168, sub: 0, line: 1583 } |  |  | 0.394 |
| walker |  | 5589 | 40 | Code::CodeKey { rung: Doc, file: beetsplug/bpd/__init__.py, decl: 36, sub: 0, line: 180 } |  |  | 0.394 |
| ns | 5629 |  | 171 | LibModel shared-method roster (complete after `_fields`) | 4.5 | 2.5 | 0.389 |
| walker |  | 5630 | 41 | Code::CodeKey { rung: Doc, file: beetsplug/bpd/__init__.py, decl: 115, sub: 0, line: 966 } |  |  | 0.389 |
| walker |  | 5678 | 48 | Code::CodeKey { rung: Doc, file: beetsplug/bpd/__init__.py, decl: 119, sub: 0, line: 1070 } |  |  | 0.389 |
| walker |  | 5744 | 66 | Code::CodeKey { rung: Doc, file: beetsplug/bpd/__init__.py, decl: 118, sub: 0, line: 1064 } |  |  | 0.389 |
| ns | 5787 |  | 158 | beets.importer public exports + package docstring | 5.1 |  | 0.384 |
| walker |  | 5813 | 69 | Code::CodeKey { rung: Doc, file: beetsplug/bpd/__init__.py, decl: 98, sub: 0, line: 770 } |  |  | 0.384 |
| walker |  | 5885 | 72 | Code::CodeKey { rung: Doc, file: beetsplug/bpd/__init__.py, decl: 116, sub: 0, line: 985 } |  |  | 0.384 |
| walker |  | 5966 | 81 | Toml::PackageMetadata { file: pyproject.toml } |  |  | 0.391 |
| ns | 6147 |  | 360 | importer/stages.py: complete stage-function roster with section banners | 5.2 |  | 0.383 |
| walker |  | 6224 | 258 | Code::CodeKey { rung: Names, file: beets/ui/__init__.py, decl: 0, sub: 0, line: 0 } |  |  | 0.383 |
| walker |  | 6251 | 27 | Code::CodeKey { rung: Decl, file: beets/ui/__init__.py, decl: 2, sub: 0, line: 65 } |  |  | 0.383 |
| walker |  | 6279 | 28 | Code::CodeKey { rung: Body, file: beets/ui/__init__.py, decl: 7, sub: 0, line: 111 } |  |  | 0.383 |
| walker |  | 6316 | 37 | Code::CodeKey { rung: Doc, file: beets/ui/__init__.py, decl: 3, sub: 0, line: 71 } |  |  | 0.383 |
| walker |  | 6328 | 12 | Code::CodeKey { rung: Body, file: beets/ui/__init__.py, decl: 4, sub: 0, line: 80 } |  |  | 0.383 |
| walker |  | 6340 | 12 | Code::CodeKey { rung: Body, file: beets/ui/__init__.py, decl: 5, sub: 0, line: 85 } |  |  | 0.383 |
| walker |  | 6359 | 19 | Code::CodeKey { rung: Doc, file: beets/ui/__init__.py, decl: 4, sub: 0, line: 80 } |  |  | 0.383 |
| walker |  | 6378 | 19 | Code::CodeKey { rung: Doc, file: beets/ui/__init__.py, decl: 5, sub: 0, line: 85 } |  |  | 0.383 |
| walker |  | 6457 | 79 | Code::CodeKey { rung: Doc, file: beets/ui/__init__.py, decl: 7, sub: 0, line: 111 } |  |  | 0.383 |
| walker |  | 6543 | 86 | Code::CodeKey { rung: Doc, file: beetsplug/bpd/__init__.py, decl: 34, sub: 0, line: 158 } |  |  | 0.383 |
| ns | 6572 |  | 425 | ImportSession.run(): how the pipeline is assembled | 5.3 | 5.2 | 0.372 |
| walker |  | 6729 | 186 | Code::CodeKey { rung: Names, file: beetsplug/metasync/__init__.py, decl: 0, sub: 0, line: 0 } |  |  | 0.372 |
| walker |  | 6758 | 29 | Code::CodeKey { rung: Decl, file: beetsplug/metasync/__init__.py, decl: 2, sub: 0, line: 34 } |  |  | 0.372 |
| walker |  | 6810 | 52 | Code::CodeKey { rung: Decl, file: beetsplug/metasync/__init__.py, decl: 3, sub: 0, line: 40 } |  |  | 0.372 |
| walker |  | 6819 | 9 | Code::CodeKey { rung: Decl, file: beetsplug/metasync/__init__.py, decl: 5, sub: 0, line: 47 } |  |  | 0.372 |
| walker |  | 6824 | 5 | Code::CodeKey { rung: Body, file: beetsplug/metasync/__init__.py, decl: 5, sub: 0, line: 47 } |  |  | 0.372 |
| ns | 6829 |  | 257 | ImportSession class contract (attributes + the four decision hooks) | 5.4 | 5.3 | 0.365 |
| walker |  | 6880 | 56 | Code::CodeKey { rung: Decl, file: beetsplug/metasync/__init__.py, decl: 9, sub: 0, line: 76 } |  |  | 0.365 |
| walker |  | 6894 | 14 | Code::CodeKey { rung: Doc, file: beetsplug/metasync/__init__.py, decl: 12, sub: 0, line: 104 } |  |  | 0.365 |
| walker |  | 6913 | 19 | Code::CodeKey { rung: Doc, file: beetsplug/metasync/__init__.py, decl: 8, sub: 0, line: 68 } |  |  | 0.365 |
| walker |  | 6920 | 7 | Code::CodeKey { rung: Body, file: beetsplug/metasync/__init__.py, decl: 10, sub: 0, line: 79 } |  |  | 0.365 |
| walker |  | 6965 | 45 | Code::CodeKey { rung: Doc, file: beetsplug/metasync/__init__.py, decl: 6, sub: 0, line: 52 } |  |  | 0.365 |
| walker |  | 7037 | 72 | Code::CodeKey { rung: Names, file: beets/__init__.py, decl: 0, sub: 0, line: 0 } |  |  | 0.365 |
| walker |  | 7063 | 26 | Code::CodeKey { rung: Decl, file: beets/__init__.py, decl: 2, sub: 0, line: 35 } |  |  | 0.365 |
| walker |  | 7073 | 10 | Code::CodeKey { rung: Doc, file: beets/__init__.py, decl: 1, sub: 0, line: 26 } |  |  | 0.365 |
| ns | 7103 |  | 274 | importer/tasks.py: Action enum and the complete task class roster | 5.5 |  | 0.359 |
| walker |  | 7113 | 40 | Code::CodeKey { rung: Doc, file: beets/__init__.py, decl: 2, sub: 0, line: 35 } |  |  | 0.359 |
| walker |  | 7170 | 57 | Code::CodeKey { rung: Body, file: beets/__init__.py, decl: 1, sub: 0, line: 26 } |  |  | 0.359 |
| ns | 7349 |  | 246 | BeetsPlugin base class: docstring and declared attributes | 6.1 |  | 0.354 |
| walker |  | 7455 | 285 | Code::CodeKey { rung: Names, file: beets/ui/__init__.py, decl: 0, sub: 1, line: 0 } |  |  | 0.354 |
| walker |  | 7462 | 7 | Code::CodeKey { rung: Decl, file: beets/ui/__init__.py, decl: 18, sub: 0, line: 459 } |  |  | 0.354 |
| walker |  | 7526 | 64 | Code::CodeKey { rung: Decl, file: beets/ui/__init__.py, decl: 13, sub: 0, line: 207 } |  |  | 0.354 |
| ns | 7595 |  | 246 | BeetsPlugin method roster (complete) | 6.2 | 6.1 | 0.350 |
| walker |  | 7598 | 72 | Code::CodeKey { rung: Decl, file: beets/ui/__init__.py, decl: 19, sub: 0, line: 466 } |  |  | 0.350 |
| walker |  | 7624 | 26 | Code::CodeKey { rung: Decl, file: beets/ui/__init__.py, decl: 40, sub: 0, line: 807 } |  |  | 0.350 |
| walker |  | 7640 | 16 | Code::CodeKey { rung: Doc, file: beets/ui/__init__.py, decl: 17, sub: 0, line: 445 } |  |  | 0.350 |
| walker |  | 7656 | 16 | Code::CodeKey { rung: Doc, file: beets/ui/__init__.py, decl: 18, sub: 0, line: 459 } |  |  | 0.350 |
| walker |  | 7754 | 98 | Code::CodeKey { rung: Decl, file: beets/ui/__init__.py, decl: 33, sub: 0, line: 676 } |  |  | 0.350 |
| walker |  | 7859 | 105 | Code::CodeKey { rung: Decl, file: beets/ui/__init__.py, decl: 27, sub: 0, line: 637 } |  |  | 0.350 |
| walker |  | 7867 | 8 | Code::CodeKey { rung: Decl, file: beets/ui/__init__.py, decl: 31, sub: 0, line: 664 } |  |  | 0.350 |
| walker |  | 7877 | 10 | Code::CodeKey { rung: Decl, file: beets/ui/__init__.py, decl: 32, sub: 0, line: 668 } |  |  | 0.350 |
| walker |  | 7896 | 19 | Code::CodeKey { rung: Doc, file: beets/ui/__init__.py, decl: 35, sub: 0, line: 701 } |  |  | 0.350 |
| walker |  | 8009 | 113 | Code::CodeKey { rung: Decl, file: beets/ui/__init__.py, decl: 20, sub: 0, line: 498 } |  |  | 0.350 |
| ns | 8017 |  | 422 | The complete `EventType` literal — every plugin event name | 6.3 | 6.1 | 0.340 |
| walker |  | 8023 | 14 | Code::CodeKey { rung: Doc, file: beets/ui/__init__.py, decl: 26, sub: 0, line: 621 } |  |  | 0.340 |
| walker |  | 8052 | 29 | Code::CodeKey { rung: Doc, file: beets/ui/__init__.py, decl: 33, sub: 0, line: 676 } |  |  | 0.340 |
| walker |  | 8084 | 32 | Code::CodeKey { rung: Doc, file: beets/ui/__init__.py, decl: 16, sub: 0, line: 433 } |  |  | 0.340 |
| walker |  | 8120 | 36 | Code::CodeKey { rung: Doc, file: beets/ui/__init__.py, decl: 27, sub: 0, line: 637 } |  |  | 0.340 |
| walker |  | 8159 | 39 | Code::CodeKey { rung: Doc, file: beets/ui/__init__.py, decl: 10, sub: 0, line: 160 } |  |  | 0.340 |
| walker |  | 8198 | 39 | Code::CodeKey { rung: Doc, file: beets/ui/__init__.py, decl: 38, sub: 0, line: 770 } |  |  | 0.340 |
| walker |  | 8237 | 39 | Code::CodeKey { rung: Doc, file: beets/ui/__init__.py, decl: 45, sub: 0, line: 996 } |  |  | 0.340 |
| walker |  | 8304 | 67 | Code::CodeKey { rung: Decl, file: beets/ui/__init__.py, decl: 23, sub: 0, line: 534 } |  |  | 0.340 |
| walker |  | 8319 | 15 | Code::CodeKey { rung: Doc, file: beets/ui/__init__.py, decl: 43, sub: 0, line: 879 } |  |  | 0.340 |
| ns | 8321 |  | 304 | plugins.py module-level API roster (complete) | 6.4 |  | 0.336 |
| walker |  | 8373 | 54 | Code::CodeKey { rung: Doc, file: beets/ui/__init__.py, decl: 39, sub: 0, line: 783 } |  |  | 0.336 |
| walker |  | 8428 | 55 | Code::CodeKey { rung: Doc, file: beets/ui/__init__.py, decl: 14, sub: 0, line: 383 } |  |  | 0.336 |
| walker |  | 8486 | 58 | Code::CodeKey { rung: Doc, file: beets/ui/__init__.py, decl: 34, sub: 0, line: 681 } |  |  | 0.336 |
| walker |  | 8503 | 17 | Code::CodeKey { rung: Doc, file: beets/ui/__init__.py, decl: 41, sub: 0, line: 830 } |  |  | 0.336 |
| walker |  | 8523 | 20 | Code::CodeKey { rung: Doc, file: beets/ui/__init__.py, decl: 9, sub: 0, line: 150 } |  |  | 0.336 |
| walker |  | 8596 | 73 | Code::CodeKey { rung: Doc, file: beets/ui/__init__.py, decl: 12, sub: 0, line: 187 } |  |  | 0.336 |
| ns | 8633 |  | 312 | MetadataSourcePlugin: the metadata-backend contract | 6.5 |  | 0.332 |
| walker |  | 8676 | 80 | Code::CodeKey { rung: Doc, file: beets/ui/__init__.py, decl: 22, sub: 0, line: 521 } |  |  | 0.332 |
| walker |  | 8765 | 89 | Code::CodeKey { rung: Doc, file: beets/ui/__init__.py, decl: 8, sub: 0, line: 122 } |  |  | 0.332 |
| ns | 8830 |  | 197 | Query-string prefixes: how `beet ls artist:foo` is parsed | 7.1 |  | 0.328 |
| walker |  | 8920 | 155 | Code::CodeKey { rung: Names, file: beetsplug/tidal/__init__.py, decl: 0, sub: 0, line: 0 } |  |  | 0.328 |
| walker |  | 8999 | 79 | Code::CodeKey { rung: Decl, file: beetsplug/tidal/__init__.py, decl: 34, sub: 0, line: 494 } |  |  | 0.328 |
| ns | 9187 |  | 357 | dbcore/query.py class roster (complete) | 7.2 |  | 0.321 |
| walker |  | 9216 | 217 | Code::CodeKey { rung: Decl, file: beetsplug/tidal/__init__.py, decl: 2, sub: 0, line: 37 } |  |  | 0.321 |
| walker |  | 9225 | 9 | Code::CodeKey { rung: Decl, file: beetsplug/tidal/__init__.py, decl: 4, sub: 0, line: 53 } |  |  | 0.321 |
| walker |  | 9233 | 8 | Code::CodeKey { rung: Decl, file: beetsplug/tidal/__init__.py, decl: 14, sub: 0, line: 161 } |  |  | 0.321 |
| walker |  | 9264 | 31 | Code::CodeKey { rung: Decl, file: beetsplug/tidal/__init__.py, decl: 13, sub: 0, line: 142 } |  |  | 0.321 |
| walker |  | 9304 | 40 | Code::CodeKey { rung: Decl, file: beetsplug/tidal/__init__.py, decl: 12, sub: 0, line: 119 } |  |  | 0.321 |
| walker |  | 9315 | 11 | Code::CodeKey { rung: Doc, file: beetsplug/tidal/__init__.py, decl: 14, sub: 0, line: 161 } |  |  | 0.321 |
| walker |  | 9329 | 14 | Code::CodeKey { rung: Body, file: beetsplug/tidal/__init__.py, decl: 11, sub: 0, line: 116 } |  |  | 0.321 |
| walker |  | 9348 | 19 | Code::CodeKey { rung: Doc, file: beetsplug/tidal/__init__.py, decl: 5, sub: 0, line: 60 } |  |  | 0.321 |
| ns | 9539 |  | 352 | beets.autotag exports, Recommendation, Proposal | 8.1 |  | 0.316 |
| walker |  | 9643 | 295 | Code::CodeKey { rung: Decl, file: beetsplug/tidal/__init__.py, decl: 2, sub: 1, line: 37 } |  |  | 0.316 |
| walker |  | 9651 | 8 | Code::CodeKey { rung: Decl, file: beetsplug/tidal/__init__.py, decl: 15, sub: 0, line: 169 } |  |  | 0.316 |
| walker |  | 9659 | 8 | Code::CodeKey { rung: Decl, file: beetsplug/tidal/__init__.py, decl: 29, sub: 0, line: 442 } |  |  | 0.316 |
| walker |  | 9667 | 8 | Code::CodeKey { rung: Decl, file: beetsplug/tidal/__init__.py, decl: 31, sub: 0, line: 461 } |  |  | 0.316 |
| walker |  | 9703 | 36 | Code::CodeKey { rung: Decl, file: beetsplug/tidal/__init__.py, decl: 24, sub: 0, line: 290 } |  |  | 0.316 |
| walker |  | 9739 | 36 | Code::CodeKey { rung: Decl, file: beetsplug/tidal/__init__.py, decl: 23, sub: 0, line: 285 } |  |  | 0.316 |
| walker |  | 9774 | 35 | Code::CodeKey { rung: Decl, file: beetsplug/tidal/__init__.py, decl: 22, sub: 0, line: 280 } |  |  | 0.316 |
| ns | 9775 |  | 236 | autotag/hooks.py and distance.py symbol roster | 8.2 |  | 0.313 |
| walker |  | 9811 | 37 | Code::CodeKey { rung: Decl, file: beetsplug/tidal/__init__.py, decl: 18, sub: 0, line: 221 } |  |  | 0.313 |
| walker |  | 9847 | 36 | Code::CodeKey { rung: Decl, file: beetsplug/tidal/__init__.py, decl: 19, sub: 0, line: 226 } |  |  | 0.313 |
| walker |  | 9882 | 35 | Code::CodeKey { rung: Decl, file: beetsplug/tidal/__init__.py, decl: 20, sub: 0, line: 231 } |  |  | 0.313 |
| walker |  | 9950 | 68 | Code::CodeKey { rung: Decl, file: beetsplug/tidal/__init__.py, decl: 25, sub: 0, line: 295 } |  |  | 0.313 |
| ns | 9970 |  | 195 | beets/util/__init__.py: classes and the core path/file-operation helpers | 9.1 |  | 0.309 |
