Score(3000)=0.517 I=0.519 C=0.516 ns_rows≤3K=17/44 grid(1000/1442/2080/3000/4327/6240/9000)=0.593/0.568/0.414/0.517/0.440/0.376/0.337

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
| walker |  | 1191 | 72 | Code::CodeKey { rung: Names, file: beets/__init__.py, decl: 0, sub: 0, line: 0 } |  |  | 0.606 |
| walker |  | 1217 | 26 | Code::CodeKey { rung: Decl, file: beets/__init__.py, decl: 2, sub: 0, line: 35 } |  |  | 0.606 |
| walker |  | 1227 | 10 | Code::CodeKey { rung: Doc, file: beets/__init__.py, decl: 1, sub: 0, line: 26 } |  |  | 0.568 |
| ns | 1227 |  | 176 | Canonical test / lint / typecheck commands | 1.10 |  | 0.568 |
| walker |  | 1267 | 40 | Code::CodeKey { rung: Doc, file: beets/__init__.py, decl: 2, sub: 0, line: 35 } |  |  | 0.568 |
| ns | 1445 |  | 218 | test/ and docs/ top-level listings (complete) | 1.11 |  | 0.505 |
| walker |  | 1525 | 258 | Code::CodeKey { rung: Names, file: beets/ui/__init__.py, decl: 0, sub: 0, line: 0 } |  |  | 0.505 |
| walker |  | 1552 | 27 | Code::CodeKey { rung: Decl, file: beets/ui/__init__.py, decl: 2, sub: 0, line: 65 } |  |  | 0.505 |
| walker |  | 1580 | 28 | Code::CodeKey { rung: Body, file: beets/ui/__init__.py, decl: 7, sub: 0, line: 111 } |  |  | 0.505 |
| walker |  | 1617 | 37 | Code::CodeKey { rung: Doc, file: beets/ui/__init__.py, decl: 3, sub: 0, line: 71 } |  |  | 0.505 |
| walker |  | 1629 | 12 | Code::CodeKey { rung: Body, file: beets/ui/__init__.py, decl: 4, sub: 0, line: 80 } |  |  | 0.505 |
| walker |  | 1644 | 15 | Fs::DirListing { dir: beets/test } |  |  | 0.524 |
| walker |  | 1701 | 57 | Code::CodeKey { rung: Body, file: beets/__init__.py, decl: 1, sub: 0, line: 26 } |  |  | 0.524 |
| walker |  | 1740 | 39 | Fs::DirListing { dir: .github } |  |  | 0.524 |
| walker |  | 1771 | 31 | Fs::DirListing { dir: .github/workflows } |  |  | 0.524 |
| walker |  | 1783 | 12 | Code::CodeKey { rung: Body, file: beets/ui/__init__.py, decl: 5, sub: 0, line: 85 } |  |  | 0.524 |
| walker |  | 1797 | 14 | Code::CodeKey { rung: Names, file: beets/library/library.py, decl: 0, sub: 0, line: 0 } |  |  | 0.524 |
| ns | 1828 |  | 383 | beetsplug/ bundled plugin listing (complete, 81 entries) | 1.12 |  | 0.430 |
| walker |  | 1838 | 41 | Fs::DirListing { dir: docs/dev/plugins/other } |  |  | 0.430 |
| ns | 2006 |  | 178 | beets.library public export block (complete) | 2.1 |  | 0.414 |
| walker |  | 2049 | 211 | Code::CodeKey { rung: Names, file: beets/ui/commands/__init__.py, decl: 0, sub: 0, line: 0 } |  |  | 0.414 |
| walker |  | 2059 | 10 | Code::CodeKey { rung: Doc, file: beets/ui/commands/__init__.py, decl: 1, sub: 0, line: 36 } |  |  | 0.414 |
| walker |  | 2166 | 107 | Code::CodeKey { rung: Decl, file: beets/ui/commands/__init__.py, decl: 2, sub: 0, line: 50 } |  |  | 0.417 |
| ns | 2183 |  | 177 | The `beet` subcommand roster (default_commands) | 2.2 |  | 0.421 |
| walker |  | 2244 | 78 | Code::CodeKey { rung: Body, file: beets/ui/commands/__init__.py, decl: 1, sub: 0, line: 36 } |  |  | 0.421 |
| ns | 2373 |  | 190 | Library class: models and schema migrations | 2.3 |  | 0.411 |
| ns | 2528 |  | 155 | Library method roster (complete) | 2.4 | 2.3 | 0.404 |
| walker |  | 2627 | 383 | Fs::DirListing { dir: beetsplug } |  |  | 0.553 |
| walker |  | 2638 | 11 | Fs::DirListing { dir: beetsplug/bpd } |  |  | 0.553 |
| walker |  | 2650 | 12 | Fs::DirListing { dir: beetsplug/web } |  |  | 0.553 |
| walker |  | 2666 | 16 | Code::CodeKey { rung: ModuleDoc, file: beetsplug/web/__init__.py, decl: 0, sub: 0, line: 0 } |  |  | 0.553 |
| walker |  | 2680 | 14 | Fs::DirListing { dir: beetsplug/discogs } |  |  | 0.553 |
| walker |  | 2696 | 16 | Fs::DirListing { dir: beetsplug/metasync } |  |  | 0.553 |
| walker |  | 2713 | 17 | Code::CodeKey { rung: ModuleDoc, file: beetsplug/metasync/__init__.py, decl: 0, sub: 0, line: 0 } |  |  | 0.553 |
| walker |  | 2717 | 4 | Fs::DirListing { dir: beetsplug/web/templates } |  |  | 0.553 |
| walker |  | 2736 | 19 | Fs::DirListing { dir: beetsplug/tidal } |  |  | 0.553 |
| walker |  | 2759 | 23 | Fs::DirListing { dir: beetsplug/lastgenre } |  |  | 0.553 |
| walker |  | 2798 | 39 | Code::CodeKey { rung: ModuleDoc, file: beetsplug/discogs/__init__.py, decl: 0, sub: 0, line: 0 } |  |  | 0.553 |
| walker |  | 2828 | 30 | Fs::DirListing { dir: beetsplug/_utils } |  |  | 0.553 |
| walker |  | 2891 | 63 | Code::CodeKey { rung: ModuleDoc, file: beetsplug/bpd/__init__.py, decl: 0, sub: 0, line: 0 } |  |  | 0.553 |
| walker |  | 2913 | 22 | Code::CodeKey { rung: ModuleDoc, file: beetsplug/zero.py, decl: 0, sub: 0, line: 0 } |  |  | 0.553 |
| walker |  | 2936 | 23 | Code::CodeKey { rung: ModuleDoc, file: beetsplug/subsonicupdate.py, decl: 0, sub: 0, line: 0 } |  |  | 0.553 |
| ns | 2952 |  | 424 | models.py class headers: LibModel, FormattedItemMapping, Album, Item | 2.5 |  | 0.517 |
| walker |  | 3041 | 105 | Code::CodeKey { rung: ModuleDoc, file: beetsplug/lastgenre/__init__.py, decl: 0, sub: 0, line: 0 } |  |  | 0.517 |
| walker |  | 3065 | 24 | Code::CodeKey { rung: ModuleDoc, file: beetsplug/subsonicplaylist.py, decl: 0, sub: 0, line: 0 } |  |  | 0.517 |
| walker |  | 3089 | 24 | Code::CodeKey { rung: ModuleDoc, file: beetsplug/unimported.py, decl: 0, sub: 0, line: 0 } |  |  | 0.517 |
| walker |  | 3121 | 32 | Code::CodeKey { rung: Names, file: beetsplug/_utils/__init__.py, decl: 0, sub: 0, line: 0 } |  |  | 0.517 |
| ns | 3125 |  | 173 | beets.dbcore public export block + package docstring | 2.6 |  | 0.503 |
| walker |  | 3143 | 22 | Fs::DirListing { dir: beetsplug/web/static } |  |  | 0.503 |
| walker |  | 3177 | 34 | Code::CodeKey { rung: ModuleDoc, file: beetsplug/titlecase.py, decl: 0, sub: 0, line: 0 } |  |  | 0.503 |
| walker |  | 3212 | 35 | Code::CodeKey { rung: ModuleDoc, file: beetsplug/ihate.py, decl: 0, sub: 0, line: 0 } |  |  | 0.503 |
| walker |  | 3247 | 35 | Code::CodeKey { rung: ModuleDoc, file: beetsplug/the.py, decl: 0, sub: 0, line: 0 } |  |  | 0.503 |
| walker |  | 3259 | 12 | Code::CodeKey { rung: Names, file: beetsplug/unimported.py, decl: 0, sub: 0, line: 0 } |  |  | 0.503 |
| walker |  | 3281 | 22 | Code::CodeKey { rung: Decl, file: beetsplug/unimported.py, decl: 1, sub: 0, line: 29 } |  |  | 0.503 |
| walker |  | 3294 | 13 | Code::CodeKey { rung: Names, file: beetsplug/subsonicupdate.py, decl: 0, sub: 0, line: 0 } |  |  | 0.503 |
| ns | 3298 |  | 173 | config_default.yaml: library, directory, plugins, ignore rules | 3.1 |  | 0.490 |
| walker |  | 3308 | 14 | Code::CodeKey { rung: Names, file: beetsplug/substitute.py, decl: 0, sub: 0, line: 0 } |  |  | 0.490 |
| walker |  | 3336 | 28 | Code::CodeKey { rung: Decl, file: beetsplug/substitute.py, decl: 1, sub: 0, line: 25 } |  |  | 0.490 |
| walker |  | 3351 | 15 | Code::CodeKey { rung: Names, file: beetsplug/replace.py, decl: 0, sub: 0, line: 0 } |  |  | 0.490 |
| walker |  | 3366 | 15 | Code::CodeKey { rung: Names, file: beetsplug/types.py, decl: 0, sub: 0, line: 0 } |  |  | 0.490 |
| walker |  | 3402 | 36 | Code::CodeKey { rung: Decl, file: beetsplug/types.py, decl: 1, sub: 0, line: 22 } |  |  | 0.490 |
| walker |  | 3408 | 6 | Code::CodeKey { rung: Decl, file: beetsplug/types.py, decl: 2, sub: 0, line: 23 } |  |  | 0.490 |
| walker |  | 3416 | 8 | Code::CodeKey { rung: Decl, file: beetsplug/types.py, decl: 3, sub: 0, line: 27 } |  |  | 0.490 |
| walker |  | 3432 | 16 | Code::CodeKey { rung: Names, file: beetsplug/albumtypes.py, decl: 0, sub: 0, line: 0 } |  |  | 0.490 |
| walker |  | 3461 | 29 | Code::CodeKey { rung: Decl, file: beetsplug/albumtypes.py, decl: 1, sub: 0, line: 29 } |  |  | 0.490 |
| walker |  | 3477 | 16 | Code::CodeKey { rung: Names, file: beetsplug/filefilter.py, decl: 0, sub: 0, line: 0 } |  |  | 0.490 |
| walker |  | 3521 | 44 | Code::CodeKey { rung: Decl, file: beetsplug/filefilter.py, decl: 1, sub: 0, line: 25 } |  |  | 0.490 |
| walker |  | 3537 | 16 | Code::CodeKey { rung: Names, file: beetsplug/importadded.py, decl: 0, sub: 0, line: 0 } |  |  | 0.490 |
| walker |  | 3553 | 16 | Code::CodeKey { rung: Names, file: beetsplug/importsource.py, decl: 0, sub: 0, line: 0 } |  |  | 0.490 |
| walker |  | 3569 | 16 | Code::CodeKey { rung: Names, file: beetsplug/ipfs.py, decl: 0, sub: 0, line: 0 } |  |  | 0.490 |
| walker |  | 3585 | 16 | Code::CodeKey { rung: Names, file: beetsplug/keyfinder.py, decl: 0, sub: 0, line: 0 } |  |  | 0.490 |
| walker |  | 3601 | 16 | Code::CodeKey { rung: Names, file: beetsplug/loadext.py, decl: 0, sub: 0, line: 0 } |  |  | 0.490 |
| walker |  | 3627 | 26 | Code::CodeKey { rung: Decl, file: beetsplug/loadext.py, decl: 1, sub: 0, line: 23 } |  |  | 0.490 |
| walker |  | 3643 | 16 | Code::CodeKey { rung: Names, file: beetsplug/mbsubmit.py, decl: 0, sub: 0, line: 0 } |  |  | 0.490 |
| walker |  | 3659 | 16 | Code::CodeKey { rung: Names, file: beetsplug/sonosupdate.py, decl: 0, sub: 0, line: 0 } |  |  | 0.490 |
| ns | 3687 |  | 389 | config_default.yaml: the complete `import:` option block | 3.2 |  | 0.466 |
| walker |  | 3701 | 42 | Code::CodeKey { rung: Decl, file: beetsplug/sonosupdate.py, decl: 1, sub: 0, line: 24 } |  |  | 0.466 |
| walker |  | 3718 | 17 | Code::CodeKey { rung: Names, file: beetsplug/autobpm.py, decl: 0, sub: 0, line: 0 } |  |  | 0.466 |
| walker |  | 3735 | 17 | Code::CodeKey { rung: Names, file: beetsplug/bpsync.py, decl: 0, sub: 0, line: 0 } |  |  | 0.466 |
| walker |  | 3752 | 17 | Code::CodeKey { rung: Names, file: beetsplug/freedesktop.py, decl: 0, sub: 0, line: 0 } |  |  | 0.466 |
| walker |  | 3780 | 28 | Code::CodeKey { rung: Decl, file: beetsplug/freedesktop.py, decl: 1, sub: 0, line: 21 } |  |  | 0.466 |
| walker |  | 3797 | 17 | Code::CodeKey { rung: Names, file: beetsplug/mbsync.py, decl: 0, sub: 0, line: 0 } |  |  | 0.466 |
| ns | 3866 |  | 179 | config_default.yaml: tagging defaults, `paths:` templates, threading | 3.4 |  | 0.455 |
| walker |  | 4054 | 257 | Code::CodeKey { rung: Names, file: beetsplug/bpd/__init__.py, decl: 0, sub: 0, line: 0 } |  |  | 0.455 |
| walker |  | 4116 | 62 | Code::CodeKey { rung: Decl, file: beetsplug/importsource.py, decl: 1, sub: 0, line: 17 } |  |  | 0.455 |
| walker |  | 4124 | 8 | Code::CodeKey { rung: Doc, file: beetsplug/importsource.py, decl: 1, sub: 0, line: 17 } |  |  | 0.455 |
| walker |  | 4146 | 22 | Code::CodeKey { rung: Names, file: beetsplug/ihate.py, decl: 0, sub: 0, line: 0 } |  |  | 0.455 |
| ns | 4171 |  | 305 | Path-template function roster: DefaultTemplateFunctions (complete) | 3.5 |  | 0.445 |
| walker |  | 4195 | 49 | Code::CodeKey { rung: Decl, file: beetsplug/ihate.py, decl: 2, sub: 0, line: 36 } |  |  | 0.445 |
| walker |  | 4203 | 8 | Code::CodeKey { rung: Decl, file: beetsplug/ihate.py, decl: 4, sub: 0, line: 49 } |  |  | 0.445 |
| walker |  | 4274 | 71 | Code::CodeKey { rung: Decl, file: beetsplug/keyfinder.py, decl: 1, sub: 0, line: 24 } |  |  | 0.445 |
| ns | 4291 |  | 120 | config_default.yaml: display formats and default sorts | 3.6 |  | 0.440 |
| walker |  | 4298 | 24 | Code::CodeKey { rung: Names, file: beetsplug/parentwork.py, decl: 0, sub: 0, line: 0 } |  |  | 0.440 |
| walker |  | 4377 | 79 | Code::CodeKey { rung: Decl, file: beetsplug/subsonicupdate.py, decl: 1, sub: 0, line: 44 } |  |  | 0.440 |
| walker |  | 4402 | 25 | Code::CodeKey { rung: Names, file: beetsplug/listenbrainz.py, decl: 0, sub: 0, line: 0 } |  |  | 0.440 |
| walker |  | 4416 | 14 | Code::CodeKey { rung: Doc, file: beetsplug/albumtypes.py, decl: 1, sub: 0, line: 29 } |  |  | 0.440 |
| walker |  | 4498 | 82 | Code::CodeKey { rung: Decl, file: beetsplug/mbsync.py, decl: 1, sub: 0, line: 25 } |  |  | 0.440 |
| walker |  | 4505 | 7 | Code::CodeKey { rung: Body, file: beetsplug/mbsync.py, decl: 2, sub: 0, line: 26 } |  |  | 0.440 |
| walker |  | 4531 | 26 | Code::CodeKey { rung: Names, file: beetsplug/titlecase.py, decl: 0, sub: 0, line: 0 } |  |  | 0.440 |
| walker |  | 4555 | 24 | Code::CodeKey { rung: Decl, file: beetsplug/titlecase.py, decl: 1, sub: 0, line: 40 } |  |  | 0.440 |
| walker |  | 4642 | 87 | Code::CodeKey { rung: Decl, file: beetsplug/mbsubmit.py, decl: 1, sub: 0, line: 33 } |  |  | 0.440 |
| walker |  | 4669 | 27 | Code::CodeKey { rung: Names, file: beetsplug/badfiles.py, decl: 0, sub: 0, line: 0 } |  |  | 0.440 |
| ns | 4673 |  | 382 | config_default.yaml: autotagger thresholds and distance weights | 3.7 |  | 0.424 |
| walker |  | 4688 | 19 | Code::CodeKey { rung: Decl, file: beetsplug/badfiles.py, decl: 1, sub: 0, line: 32 } |  |  | 0.424 |
| walker |  | 4715 | 27 | Code::CodeKey { rung: Names, file: beetsplug/bpm.py, decl: 0, sub: 0, line: 0 } |  |  | 0.424 |
| walker |  | 4772 | 57 | Code::CodeKey { rung: Decl, file: beetsplug/bpm.py, decl: 2, sub: 0, line: 47 } |  |  | 0.424 |
| walker |  | 4799 | 27 | Code::CodeKey { rung: Names, file: beetsplug/mpdupdate.py, decl: 0, sub: 0, line: 0 } |  |  | 0.424 |
| ns | 4813 |  | 140 | config_default.yaml: terminal UI, colour names, import layout | 3.8 |  | 0.417 |
| walker |  | 4856 | 57 | Code::CodeKey { rung: Decl, file: beetsplug/mpdupdate.py, decl: 1, sub: 0, line: 34 } |  |  | 0.417 |
| walker |  | 4921 | 65 | Code::CodeKey { rung: Decl, file: beetsplug/mpdupdate.py, decl: 6, sub: 0, line: 66 } |  |  | 0.417 |
| walker |  | 4933 | 12 | Code::CodeKey { rung: Doc, file: beetsplug/mpdupdate.py, decl: 1, sub: 0, line: 34 } |  |  | 0.417 |
| walker |  | 4940 | 7 | Code::CodeKey { rung: Body, file: beetsplug/mpdupdate.py, decl: 5, sub: 0, line: 62 } |  |  | 0.417 |
| ns | 4986 |  | 173 | Album method roster (complete, names only) | 4.1 | 2.5 | 0.410 |
| walker |  | 5028 | 88 | Code::CodeKey { rung: Decl, file: beetsplug/autobpm.py, decl: 1, sub: 0, line: 32 } |  |  | 0.410 |
| walker |  | 5036 | 8 | Code::CodeKey { rung: Body, file: beetsplug/types.py, decl: 2, sub: 0, line: 23 } |  |  | 0.410 |
| walker |  | 5065 | 29 | Code::CodeKey { rung: Names, file: beetsplug/subsonicplaylist.py, decl: 0, sub: 0, line: 0 } |  |  | 0.410 |
| walker |  | 5095 | 30 | Code::CodeKey { rung: Names, file: beetsplug/hook.py, decl: 0, sub: 0, line: 0 } |  |  | 0.410 |
| walker |  | 5120 | 25 | Code::CodeKey { rung: Decl, file: beetsplug/hook.py, decl: 1, sub: 0, line: 28 } |  |  | 0.410 |
| walker |  | 5151 | 31 | Code::CodeKey { rung: Decl, file: beetsplug/hook.py, decl: 3, sub: 0, line: 44 } |  |  | 0.410 |
| walker |  | 5181 | 30 | Code::CodeKey { rung: Names, file: beetsplug/scrub.py, decl: 0, sub: 0, line: 0 } |  |  | 0.410 |
| walker |  | 5268 | 87 | Code::CodeKey { rung: Decl, file: beetsplug/scrub.py, decl: 2, sub: 0, line: 44 } |  |  | 0.410 |
| walker |  | 5276 | 8 | Code::CodeKey { rung: Decl, file: beetsplug/scrub.py, decl: 5, sub: 0, line: 78 } |  |  | 0.410 |
| walker |  | 5290 | 14 | Code::CodeKey { rung: Doc, file: beetsplug/scrub.py, decl: 2, sub: 0, line: 44 } |  |  | 0.400 |
| ns | 5290 |  | 304 | Item method roster (complete, names only) | 4.3 | 2.5 | 0.400 |
| walker |  | 5387 | 97 | Code::CodeKey { rung: Decl, file: beetsplug/subsonicplaylist.py, decl: 2, sub: 0, line: 59 } |  |  | 0.400 |
| ns | 5458 |  | 168 | Item search fields and MediaFile-backed field sets | 4.4 | 2.5 | 0.394 |
| ns | 5629 |  | 171 | LibModel shared-method roster (complete after `_fields`) | 4.5 | 2.5 | 0.389 |
| ns | 5787 |  | 158 | beets.importer public exports + package docstring | 5.1 |  | 0.384 |
| walker |  | 5831 | 444 | Code::CodeKey { rung: Names, file: beetsplug/web/__init__.py, decl: 0, sub: 0, line: 0 } |  |  | 0.384 |
| walker |  | 5839 | 8 | Code::CodeKey { rung: Decl, file: beetsplug/web/__init__.py, decl: 32, sub: 0, line: 439 } |  |  | 0.384 |
| walker |  | 5848 | 9 | Code::CodeKey { rung: Decl, file: beetsplug/web/__init__.py, decl: 18, sub: 0, line: 296 } |  |  | 0.384 |
| walker |  | 5858 | 10 | Code::CodeKey { rung: Decl, file: beetsplug/web/__init__.py, decl: 30, sub: 0, line: 412 } |  |  | 0.384 |
| walker |  | 5868 | 10 | Code::CodeKey { rung: Decl, file: beetsplug/web/__init__.py, decl: 31, sub: 0, line: 423 } |  |  | 0.384 |
| walker |  | 5882 | 14 | Code::CodeKey { rung: Decl, file: beetsplug/web/__init__.py, decl: 23, sub: 0, line: 344 } |  |  | 0.384 |
| walker |  | 5897 | 15 | Code::CodeKey { rung: Decl, file: beetsplug/web/__init__.py, decl: 24, sub: 0, line: 354 } |  |  | 0.384 |
| walker |  | 5912 | 15 | Code::CodeKey { rung: Decl, file: beetsplug/web/__init__.py, decl: 29, sub: 0, line: 397 } |  |  | 0.384 |
| walker |  | 5928 | 16 | Code::CodeKey { rung: Decl, file: beetsplug/web/__init__.py, decl: 21, sub: 0, line: 317 } |  |  | 0.384 |
| walker |  | 5945 | 17 | Code::CodeKey { rung: Decl, file: beetsplug/web/__init__.py, decl: 28, sub: 0, line: 388 } |  |  | 0.384 |
| walker |  | 5967 | 22 | Code::CodeKey { rung: Decl, file: beetsplug/web/__init__.py, decl: 16, sub: 0, line: 283 } |  |  | 0.384 |
| walker |  | 5989 | 22 | Code::CodeKey { rung: Decl, file: beetsplug/web/__init__.py, decl: 33, sub: 0, line: 447 } |  |  | 0.384 |
| walker |  | 6017 | 28 | Code::CodeKey { rung: Decl, file: beetsplug/web/__init__.py, decl: 10, sub: 0, line: 252 } |  |  | 0.384 |
| walker |  | 6045 | 28 | Code::CodeKey { rung: Decl, file: beetsplug/web/__init__.py, decl: 13, sub: 0, line: 268 } |  |  | 0.384 |
| walker |  | 6076 | 31 | Code::CodeKey { rung: Decl, file: beetsplug/web/__init__.py, decl: 20, sub: 0, line: 310 } |  |  | 0.384 |
| walker |  | 6107 | 31 | Code::CodeKey { rung: Decl, file: beetsplug/web/__init__.py, decl: 25, sub: 0, line: 369 } |  |  | 0.384 |
| walker |  | 6138 | 31 | Code::CodeKey { rung: Decl, file: beetsplug/web/__init__.py, decl: 26, sub: 0, line: 375 } |  |  | 0.384 |
| ns | 6147 |  | 360 | importer/stages.py: complete stage-function roster with section banners | 5.2 |  | 0.376 |
| walker |  | 6170 | 32 | Code::CodeKey { rung: Decl, file: beetsplug/web/__init__.py, decl: 27, sub: 0, line: 382 } |  |  | 0.376 |
| walker |  | 6203 | 33 | Code::CodeKey { rung: Decl, file: beetsplug/web/__init__.py, decl: 36, sub: 0, line: 520 } |  |  | 0.376 |
| walker |  | 6241 | 38 | Code::CodeKey { rung: Decl, file: beetsplug/web/__init__.py, decl: 19, sub: 0, line: 304 } |  |  | 0.376 |
| walker |  | 6280 | 39 | Code::CodeKey { rung: Decl, file: beetsplug/web/__init__.py, decl: 22, sub: 0, line: 338 } |  |  | 0.376 |
| walker |  | 6311 | 31 | Code::CodeKey { rung: Names, file: beetsplug/bareasc.py, decl: 0, sub: 0, line: 0 } |  |  | 0.376 |
| walker |  | 6339 | 28 | Code::CodeKey { rung: Decl, file: beetsplug/bareasc.py, decl: 1, sub: 0, line: 29 } |  |  | 0.376 |
| walker |  | 6347 | 8 | Code::CodeKey { rung: Decl, file: beetsplug/bareasc.py, decl: 2, sub: 0, line: 32 } |  |  | 0.376 |
| walker |  | 6402 | 55 | Code::CodeKey { rung: Decl, file: beetsplug/bareasc.py, decl: 4, sub: 0, line: 55 } |  |  | 0.376 |
| walker |  | 6416 | 14 | Code::CodeKey { rung: Doc, file: beetsplug/bareasc.py, decl: 1, sub: 0, line: 29 } |  |  | 0.376 |
| walker |  | 6447 | 31 | Code::CodeKey { rung: Names, file: beetsplug/embedart.py, decl: 0, sub: 0, line: 0 } |  |  | 0.376 |
| walker |  | 6517 | 70 | Code::CodeKey { rung: Decl, file: beetsplug/embedart.py, decl: 2, sub: 0, line: 52 } |  |  | 0.376 |
| walker |  | 6532 | 15 | Code::CodeKey { rung: Doc, file: beetsplug/embedart.py, decl: 2, sub: 0, line: 52 } |  |  | 0.376 |
| walker |  | 6563 | 31 | Code::CodeKey { rung: Names, file: beetsplug/fuzzy.py, decl: 0, sub: 0, line: 0 } |  |  | 0.376 |
| ns | 6572 |  | 425 | ImportSession.run(): how the pipeline is assembled | 5.3 | 5.2 | 0.365 |
| walker |  | 6588 | 25 | Code::CodeKey { rung: Decl, file: beetsplug/fuzzy.py, decl: 4, sub: 0, line: 51 } |  |  | 0.365 |
| walker |  | 6636 | 48 | Code::CodeKey { rung: Decl, file: beetsplug/fuzzy.py, decl: 1, sub: 0, line: 24 } |  |  | 0.365 |
| walker |  | 6644 | 8 | Code::CodeKey { rung: Decl, file: beetsplug/fuzzy.py, decl: 3, sub: 0, line: 29 } |  |  | 0.365 |
| walker |  | 6745 | 101 | Code::CodeKey { rung: Decl, file: beetsplug/parentwork.py, decl: 1, sub: 0, line: 34 } |  |  | 0.365 |
| walker |  | 6778 | 33 | Code::CodeKey { rung: Names, file: beetsplug/advancedrewrite.py, decl: 0, sub: 0, line: 0 } |  |  | 0.365 |
| walker |  | 6802 | 24 | Code::CodeKey { rung: Decl, file: beetsplug/advancedrewrite.py, decl: 2, sub: 0, line: 57 } |  |  | 0.365 |
| walker |  | 6816 | 14 | Code::CodeKey { rung: Doc, file: beetsplug/advancedrewrite.py, decl: 2, sub: 0, line: 57 } |  |  | 0.365 |
| ns | 6829 |  | 257 | ImportSession class contract (attributes + the four decision hooks) | 5.4 | 5.3 | 0.358 |
| walker |  | 6849 | 33 | Code::CodeKey { rung: Names, file: beetsplug/kodiupdate.py, decl: 0, sub: 0, line: 0 } |  |  | 0.358 |
| walker |  | 6891 | 42 | Code::CodeKey { rung: Decl, file: beetsplug/kodiupdate.py, decl: 2, sub: 0, line: 53 } |  |  | 0.358 |
| walker |  | 6972 | 81 | Toml::PackageMetadata { file: pyproject.toml } |  |  | 0.365 |
| ns | 7103 |  | 274 | importer/tasks.py: Action enum and the complete task class roster | 5.5 |  | 0.359 |
| walker |  | 7124 | 152 | Code::CodeKey { rung: Names, file: beets/importer/__init__.py, decl: 0, sub: 0, line: 0 } |  |  | 0.369 |
| walker |  | 7143 | 19 | Code::CodeKey { rung: Doc, file: beetsplug/hook.py, decl: 3, sub: 0, line: 44 } |  |  | 0.369 |
| walker |  | 7162 | 19 | Code::CodeKey { rung: Doc, file: beetsplug/kodiupdate.py, decl: 1, sub: 0, line: 31 } |  |  | 0.369 |
| walker |  | 7274 | 112 | Code::CodeKey { rung: Decl, file: beetsplug/bpsync.py, decl: 1, sub: 0, line: 26 } |  |  | 0.369 |
| walker |  | 7282 | 8 | Code::CodeKey { rung: Decl, file: beetsplug/bpsync.py, decl: 6, sub: 0, line: 103 } |  |  | 0.369 |
| ns | 7349 |  | 246 | BeetsPlugin base class: docstring and declared attributes | 6.1 |  | 0.363 |
| walker |  | 7437 | 155 | Code::CodeKey { rung: Names, file: beetsplug/tidal/__init__.py, decl: 0, sub: 0, line: 0 } |  |  | 0.363 |
| walker |  | 7516 | 79 | Code::CodeKey { rung: Decl, file: beetsplug/tidal/__init__.py, decl: 34, sub: 0, line: 494 } |  |  | 0.363 |
| ns | 7595 |  | 246 | BeetsPlugin method roster (complete) | 6.2 | 6.1 | 0.360 |
| walker |  | 7733 | 217 | Code::CodeKey { rung: Decl, file: beetsplug/tidal/__init__.py, decl: 2, sub: 0, line: 37 } |  |  | 0.360 |
| walker |  | 7742 | 9 | Code::CodeKey { rung: Decl, file: beetsplug/tidal/__init__.py, decl: 4, sub: 0, line: 53 } |  |  | 0.360 |
| walker |  | 7750 | 8 | Code::CodeKey { rung: Decl, file: beetsplug/tidal/__init__.py, decl: 14, sub: 0, line: 161 } |  |  | 0.360 |
| walker |  | 7781 | 31 | Code::CodeKey { rung: Decl, file: beetsplug/tidal/__init__.py, decl: 13, sub: 0, line: 142 } |  |  | 0.360 |
| walker |  | 7821 | 40 | Code::CodeKey { rung: Decl, file: beetsplug/tidal/__init__.py, decl: 12, sub: 0, line: 119 } |  |  | 0.360 |
| walker |  | 7835 | 14 | Code::CodeKey { rung: Body, file: beetsplug/tidal/__init__.py, decl: 11, sub: 0, line: 116 } |  |  | 0.360 |
| walker |  | 7895 | 60 | Code::CodeKey { rung: Decl, file: beetsplug/autobpm.py, decl: 6, sub: 0, line: 95 } |  |  | 0.360 |
| ns | 8017 |  | 422 | The complete `EventType` literal — every plugin event name | 6.3 | 6.1 | 0.350 |
| walker |  | 8081 | 186 | Code::CodeKey { rung: Names, file: beetsplug/metasync/__init__.py, decl: 0, sub: 0, line: 0 } |  |  | 0.350 |
| walker |  | 8110 | 29 | Code::CodeKey { rung: Decl, file: beetsplug/metasync/__init__.py, decl: 2, sub: 0, line: 34 } |  |  | 0.350 |
| walker |  | 8162 | 52 | Code::CodeKey { rung: Decl, file: beetsplug/metasync/__init__.py, decl: 3, sub: 0, line: 40 } |  |  | 0.350 |
| walker |  | 8171 | 9 | Code::CodeKey { rung: Decl, file: beetsplug/metasync/__init__.py, decl: 5, sub: 0, line: 47 } |  |  | 0.350 |
| walker |  | 8227 | 56 | Code::CodeKey { rung: Decl, file: beetsplug/metasync/__init__.py, decl: 9, sub: 0, line: 76 } |  |  | 0.350 |
| walker |  | 8232 | 5 | Code::CodeKey { rung: Body, file: beetsplug/metasync/__init__.py, decl: 5, sub: 0, line: 47 } |  |  | 0.350 |
| walker |  | 8239 | 7 | Code::CodeKey { rung: Body, file: beetsplug/metasync/__init__.py, decl: 10, sub: 0, line: 79 } |  |  | 0.350 |
| walker |  | 8258 | 19 | Code::CodeKey { rung: Doc, file: beetsplug/metasync/__init__.py, decl: 8, sub: 0, line: 68 } |  |  | 0.350 |
| walker |  | 8294 | 36 | Code::CodeKey { rung: Names, file: beetsplug/duplicates.py, decl: 0, sub: 0, line: 0 } |  |  | 0.350 |
| ns | 8321 |  | 304 | plugins.py module-level API roster (complete) | 6.4 |  | 0.345 |
| walker |  | 8411 | 117 | Code::CodeKey { rung: Decl, file: beetsplug/replace.py, decl: 1, sub: 0, line: 16 } |  |  | 0.345 |
| walker |  | 8448 | 37 | Code::CodeKey { rung: Names, file: beetsplug/deezer.py, decl: 0, sub: 0, line: 0 } |  |  | 0.345 |
| ns | 8633 |  | 312 | MetadataSourcePlugin: the metadata-backend contract | 6.5 |  | 0.341 |
| walker |  | 8767 | 319 | Code::CodeKey { rung: Names, file: beets/util/__init__.py, decl: 0, sub: 0, line: 0 } |  |  | 0.341 |
| walker |  | 8801 | 34 | Code::CodeKey { rung: Decl, file: beets/util/__init__.py, decl: 14, sub: 0, line: 130 } |  |  | 0.341 |
| ns | 8830 |  | 197 | Query-string prefixes: how `beet ls artist:foo` is parsed | 7.1 |  | 0.337 |
| walker |  | 8897 | 96 | Code::CodeKey { rung: Decl, file: beets/util/__init__.py, decl: 8, sub: 0, line: 74 } |  |  | 0.337 |
| walker |  | 8907 | 10 | Code::CodeKey { rung: Body, file: beets/util/__init__.py, decl: 12, sub: 0, line: 115 } |  |  | 0.337 |
| walker |  | 8930 | 23 | Code::CodeKey { rung: Body, file: beets/util/__init__.py, decl: 15, sub: 0, line: 136 } |  |  | 0.337 |
| walker |  | 8958 | 28 | Code::CodeKey { rung: Doc, file: beets/util/__init__.py, decl: 12, sub: 0, line: 115 } |  |  | 0.337 |
| walker |  | 8969 | 11 | Code::CodeKey { rung: Doc, file: beetsplug/albumtypes.py, decl: 2, sub: 0, line: 32 } |  |  | 0.337 |
| walker |  | 8980 | 11 | Code::CodeKey { rung: Doc, file: beetsplug/scrub.py, decl: 8, sub: 0, line: 147 } |  |  | 0.337 |
| walker |  | 8991 | 11 | Code::CodeKey { rung: Doc, file: beetsplug/substitute.py, decl: 2, sub: 0, line: 33 } |  |  | 0.337 |
| walker |  | 9163 | 172 | Code::CodeKey { rung: Names, file: beets/ui/commands/import_/__init__.py, decl: 0, sub: 0, line: 0 } |  |  | 0.337 |
| ns | 9187 |  | 357 | dbcore/query.py class roster (complete) | 7.2 |  | 0.330 |
| walker |  | 9189 | 26 | Code::CodeKey { rung: Decl, file: beets/ui/commands/import_/__init__.py, decl: 7, sub: 0, line: 160 } |  |  | 0.330 |
| walker |  | 9207 | 18 | Code::CodeKey { rung: Doc, file: beets/ui/commands/import_/__init__.py, decl: 3, sub: 0, line: 34 } |  |  | 0.330 |
| walker |  | 9239 | 32 | Code::CodeKey { rung: Doc, file: beets/ui/commands/import_/__init__.py, decl: 4, sub: 0, line: 49 } |  |  | 0.330 |
| walker |  | 9272 | 33 | Code::CodeKey { rung: Doc, file: beets/ui/commands/import_/__init__.py, decl: 2, sub: 0, line: 14 } |  |  | 0.330 |
| walker |  | 9292 | 20 | Code::CodeKey { rung: Names, file: beets/library/fields.py, decl: 0, sub: 0, line: 0 } |  |  | 0.330 |
| walker |  | 9312 | 20 | Code::CodeKey { rung: Names, file: beets/util/hidden.py, decl: 0, sub: 0, line: 0 } |  |  | 0.330 |
| walker |  | 9323 | 11 | Code::CodeKey { rung: Body, file: beetsplug/keyfinder.py, decl: 5, sub: 0, line: 48 } |  |  | 0.330 |
| walker |  | 9345 | 22 | Code::CodeKey { rung: Doc, file: beetsplug/hook.py, decl: 1, sub: 0, line: 28 } |  |  | 0.330 |
| ns | 9539 |  | 352 | beets.autotag exports, Recommendation, Proposal | 8.1 |  | 0.324 |
| walker |  | 9550 | 205 | Markdown::Section { file: README.rst, section_index: 0, keeps_default_concavity: false } |  |  | 0.402 |
| walker |  | 9564 | 14 | Code::CodeKey { rung: Doc, file: beetsplug/metasync/__init__.py, decl: 12, sub: 0, line: 104 } |  |  | 0.402 |
| ns | 9775 |  | 236 | autotag/hooks.py and distance.py symbol roster | 8.2 |  | 0.398 |
| walker |  | 9873 | 309 | Code::CodeKey { rung: Names, file: beetsplug/discogs/__init__.py, decl: 0, sub: 0, line: 0 } |  |  | 0.398 |
| walker |  | 9929 | 56 | Code::CodeKey { rung: Decl, file: beetsplug/discogs/__init__.py, decl: 4, sub: 0, line: 60 } |  |  | 0.398 |
| ns | 9970 |  | 195 | beets/util/__init__.py: classes and the core path/file-operation helpers | 9.1 |  | 0.393 |
| walker |  | 10000 | 71 | Code::CodeKey { rung: Decl, file: beetsplug/discogs/__init__.py, decl: 6, sub: 0, line: 83 } |  |  | 0.393 |
