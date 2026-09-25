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
| walker |  | 1802 | 19 | Code::CodeKey { rung: Doc, file: beets/ui/__init__.py, decl: 4, sub: 0, line: 80 } |  |  | 0.524 |
| walker |  | 1821 | 19 | Code::CodeKey { rung: Doc, file: beets/ui/__init__.py, decl: 5, sub: 0, line: 85 } |  |  | 0.524 |
| ns | 1828 |  | 383 | beetsplug/ bundled plugin listing (complete, 81 entries) | 1.12 |  | 0.430 |
| walker |  | 1835 | 14 | Code::CodeKey { rung: Names, file: beets/library/library.py, decl: 0, sub: 0, line: 0 } |  |  | 0.430 |
| walker |  | 1876 | 41 | Fs::DirListing { dir: docs/dev/plugins/other } |  |  | 0.430 |
| ns | 2006 |  | 178 | beets.library public export block (complete) | 2.1 |  | 0.414 |
| walker |  | 2087 | 211 | Code::CodeKey { rung: Names, file: beets/ui/commands/__init__.py, decl: 0, sub: 0, line: 0 } |  |  | 0.414 |
| walker |  | 2097 | 10 | Code::CodeKey { rung: Doc, file: beets/ui/commands/__init__.py, decl: 1, sub: 0, line: 36 } |  |  | 0.414 |
| ns | 2183 |  | 177 | The `beet` subcommand roster (default_commands) | 2.2 |  | 0.400 |
| walker |  | 2204 | 107 | Code::CodeKey { rung: Decl, file: beets/ui/commands/__init__.py, decl: 2, sub: 0, line: 50 } |  |  | 0.421 |
| walker |  | 2283 | 79 | Code::CodeKey { rung: Doc, file: beets/ui/__init__.py, decl: 7, sub: 0, line: 111 } |  |  | 0.421 |
| walker |  | 2361 | 78 | Code::CodeKey { rung: Body, file: beets/ui/commands/__init__.py, decl: 1, sub: 0, line: 36 } |  |  | 0.421 |
| ns | 2373 |  | 190 | Library class: models and schema migrations | 2.3 |  | 0.411 |
| ns | 2528 |  | 155 | Library method roster (complete) | 2.4 | 2.3 | 0.404 |
| walker |  | 2744 | 383 | Fs::DirListing { dir: beetsplug } |  |  | 0.553 |
| walker |  | 2755 | 11 | Fs::DirListing { dir: beetsplug/bpd } |  |  | 0.553 |
| walker |  | 2767 | 12 | Fs::DirListing { dir: beetsplug/web } |  |  | 0.553 |
| walker |  | 2783 | 16 | Code::CodeKey { rung: ModuleDoc, file: beetsplug/web/__init__.py, decl: 0, sub: 0, line: 0 } |  |  | 0.553 |
| walker |  | 2797 | 14 | Fs::DirListing { dir: beetsplug/discogs } |  |  | 0.553 |
| walker |  | 2813 | 16 | Fs::DirListing { dir: beetsplug/metasync } |  |  | 0.553 |
| walker |  | 2830 | 17 | Code::CodeKey { rung: ModuleDoc, file: beetsplug/metasync/__init__.py, decl: 0, sub: 0, line: 0 } |  |  | 0.553 |
| walker |  | 2834 | 4 | Fs::DirListing { dir: beetsplug/web/templates } |  |  | 0.553 |
| walker |  | 2853 | 19 | Fs::DirListing { dir: beetsplug/tidal } |  |  | 0.553 |
| walker |  | 2876 | 23 | Fs::DirListing { dir: beetsplug/lastgenre } |  |  | 0.553 |
| walker |  | 2915 | 39 | Code::CodeKey { rung: ModuleDoc, file: beetsplug/discogs/__init__.py, decl: 0, sub: 0, line: 0 } |  |  | 0.553 |
| walker |  | 2945 | 30 | Fs::DirListing { dir: beetsplug/_utils } |  |  | 0.553 |
| ns | 2952 |  | 424 | models.py class headers: LibModel, FormattedItemMapping, Album, Item | 2.5 |  | 0.517 |
| walker |  | 3008 | 63 | Code::CodeKey { rung: ModuleDoc, file: beetsplug/bpd/__init__.py, decl: 0, sub: 0, line: 0 } |  |  | 0.517 |
| walker |  | 3030 | 22 | Code::CodeKey { rung: ModuleDoc, file: beetsplug/zero.py, decl: 0, sub: 0, line: 0 } |  |  | 0.517 |
| walker |  | 3053 | 23 | Code::CodeKey { rung: ModuleDoc, file: beetsplug/subsonicupdate.py, decl: 0, sub: 0, line: 0 } |  |  | 0.517 |
| ns | 3125 |  | 173 | beets.dbcore public export block + package docstring | 2.6 |  | 0.503 |
| walker |  | 3158 | 105 | Code::CodeKey { rung: ModuleDoc, file: beetsplug/lastgenre/__init__.py, decl: 0, sub: 0, line: 0 } |  |  | 0.503 |
| walker |  | 3182 | 24 | Code::CodeKey { rung: ModuleDoc, file: beetsplug/subsonicplaylist.py, decl: 0, sub: 0, line: 0 } |  |  | 0.503 |
| walker |  | 3206 | 24 | Code::CodeKey { rung: ModuleDoc, file: beetsplug/unimported.py, decl: 0, sub: 0, line: 0 } |  |  | 0.503 |
| walker |  | 3238 | 32 | Code::CodeKey { rung: Names, file: beetsplug/_utils/__init__.py, decl: 0, sub: 0, line: 0 } |  |  | 0.503 |
| walker |  | 3260 | 22 | Fs::DirListing { dir: beetsplug/web/static } |  |  | 0.503 |
| walker |  | 3294 | 34 | Code::CodeKey { rung: ModuleDoc, file: beetsplug/titlecase.py, decl: 0, sub: 0, line: 0 } |  |  | 0.503 |
| ns | 3298 |  | 173 | config_default.yaml: library, directory, plugins, ignore rules | 3.1 |  | 0.490 |
| walker |  | 3329 | 35 | Code::CodeKey { rung: ModuleDoc, file: beetsplug/ihate.py, decl: 0, sub: 0, line: 0 } |  |  | 0.490 |
| walker |  | 3364 | 35 | Code::CodeKey { rung: ModuleDoc, file: beetsplug/the.py, decl: 0, sub: 0, line: 0 } |  |  | 0.490 |
| walker |  | 3376 | 12 | Code::CodeKey { rung: Names, file: beetsplug/unimported.py, decl: 0, sub: 0, line: 0 } |  |  | 0.490 |
| walker |  | 3398 | 22 | Code::CodeKey { rung: Decl, file: beetsplug/unimported.py, decl: 1, sub: 0, line: 29 } |  |  | 0.490 |
| walker |  | 3411 | 13 | Code::CodeKey { rung: Names, file: beetsplug/subsonicupdate.py, decl: 0, sub: 0, line: 0 } |  |  | 0.490 |
| walker |  | 3425 | 14 | Code::CodeKey { rung: Names, file: beetsplug/substitute.py, decl: 0, sub: 0, line: 0 } |  |  | 0.490 |
| walker |  | 3453 | 28 | Code::CodeKey { rung: Decl, file: beetsplug/substitute.py, decl: 1, sub: 0, line: 25 } |  |  | 0.490 |
| walker |  | 3468 | 15 | Code::CodeKey { rung: Names, file: beetsplug/replace.py, decl: 0, sub: 0, line: 0 } |  |  | 0.490 |
| walker |  | 3483 | 15 | Code::CodeKey { rung: Names, file: beetsplug/types.py, decl: 0, sub: 0, line: 0 } |  |  | 0.490 |
| walker |  | 3519 | 36 | Code::CodeKey { rung: Decl, file: beetsplug/types.py, decl: 1, sub: 0, line: 22 } |  |  | 0.490 |
| walker |  | 3525 | 6 | Code::CodeKey { rung: Decl, file: beetsplug/types.py, decl: 2, sub: 0, line: 23 } |  |  | 0.490 |
| walker |  | 3533 | 8 | Code::CodeKey { rung: Decl, file: beetsplug/types.py, decl: 3, sub: 0, line: 27 } |  |  | 0.490 |
| walker |  | 3549 | 16 | Code::CodeKey { rung: Names, file: beetsplug/albumtypes.py, decl: 0, sub: 0, line: 0 } |  |  | 0.490 |
| walker |  | 3578 | 29 | Code::CodeKey { rung: Decl, file: beetsplug/albumtypes.py, decl: 1, sub: 0, line: 29 } |  |  | 0.490 |
| walker |  | 3594 | 16 | Code::CodeKey { rung: Names, file: beetsplug/filefilter.py, decl: 0, sub: 0, line: 0 } |  |  | 0.490 |
| walker |  | 3638 | 44 | Code::CodeKey { rung: Decl, file: beetsplug/filefilter.py, decl: 1, sub: 0, line: 25 } |  |  | 0.490 |
| walker |  | 3654 | 16 | Code::CodeKey { rung: Names, file: beetsplug/importadded.py, decl: 0, sub: 0, line: 0 } |  |  | 0.490 |
| walker |  | 3670 | 16 | Code::CodeKey { rung: Names, file: beetsplug/importsource.py, decl: 0, sub: 0, line: 0 } |  |  | 0.490 |
| walker |  | 3686 | 16 | Code::CodeKey { rung: Names, file: beetsplug/ipfs.py, decl: 0, sub: 0, line: 0 } |  |  | 0.490 |
| ns | 3687 |  | 389 | config_default.yaml: the complete `import:` option block | 3.2 |  | 0.466 |
| walker |  | 3702 | 16 | Code::CodeKey { rung: Names, file: beetsplug/keyfinder.py, decl: 0, sub: 0, line: 0 } |  |  | 0.466 |
| walker |  | 3718 | 16 | Code::CodeKey { rung: Names, file: beetsplug/loadext.py, decl: 0, sub: 0, line: 0 } |  |  | 0.466 |
| walker |  | 3744 | 26 | Code::CodeKey { rung: Decl, file: beetsplug/loadext.py, decl: 1, sub: 0, line: 23 } |  |  | 0.466 |
| walker |  | 3760 | 16 | Code::CodeKey { rung: Names, file: beetsplug/mbsubmit.py, decl: 0, sub: 0, line: 0 } |  |  | 0.466 |
| walker |  | 3776 | 16 | Code::CodeKey { rung: Names, file: beetsplug/sonosupdate.py, decl: 0, sub: 0, line: 0 } |  |  | 0.466 |
| walker |  | 3818 | 42 | Code::CodeKey { rung: Decl, file: beetsplug/sonosupdate.py, decl: 1, sub: 0, line: 24 } |  |  | 0.466 |
| walker |  | 3835 | 17 | Code::CodeKey { rung: Names, file: beetsplug/autobpm.py, decl: 0, sub: 0, line: 0 } |  |  | 0.466 |
| walker |  | 3852 | 17 | Code::CodeKey { rung: Names, file: beetsplug/bpsync.py, decl: 0, sub: 0, line: 0 } |  |  | 0.466 |
| ns | 3866 |  | 179 | config_default.yaml: tagging defaults, `paths:` templates, threading | 3.4 |  | 0.455 |
| walker |  | 3869 | 17 | Code::CodeKey { rung: Names, file: beetsplug/freedesktop.py, decl: 0, sub: 0, line: 0 } |  |  | 0.455 |
| walker |  | 3897 | 28 | Code::CodeKey { rung: Decl, file: beetsplug/freedesktop.py, decl: 1, sub: 0, line: 21 } |  |  | 0.455 |
| walker |  | 3914 | 17 | Code::CodeKey { rung: Names, file: beetsplug/mbsync.py, decl: 0, sub: 0, line: 0 } |  |  | 0.455 |
| walker |  | 4171 | 257 | Code::CodeKey { rung: Names, file: beetsplug/bpd/__init__.py, decl: 0, sub: 0, line: 0 } |  |  | 0.445 |
| ns | 4171 |  | 305 | Path-template function roster: DefaultTemplateFunctions (complete) | 3.5 |  | 0.445 |
| walker |  | 4233 | 62 | Code::CodeKey { rung: Decl, file: beetsplug/importsource.py, decl: 1, sub: 0, line: 17 } |  |  | 0.445 |
| walker |  | 4241 | 8 | Code::CodeKey { rung: Doc, file: beetsplug/importsource.py, decl: 1, sub: 0, line: 17 } |  |  | 0.445 |
| walker |  | 4263 | 22 | Code::CodeKey { rung: Names, file: beetsplug/ihate.py, decl: 0, sub: 0, line: 0 } |  |  | 0.445 |
| ns | 4291 |  | 120 | config_default.yaml: display formats and default sorts | 3.6 |  | 0.440 |
| walker |  | 4312 | 49 | Code::CodeKey { rung: Decl, file: beetsplug/ihate.py, decl: 2, sub: 0, line: 36 } |  |  | 0.440 |
| walker |  | 4320 | 8 | Code::CodeKey { rung: Decl, file: beetsplug/ihate.py, decl: 4, sub: 0, line: 49 } |  |  | 0.440 |
| walker |  | 4391 | 71 | Code::CodeKey { rung: Decl, file: beetsplug/keyfinder.py, decl: 1, sub: 0, line: 24 } |  |  | 0.440 |
| walker |  | 4415 | 24 | Code::CodeKey { rung: Names, file: beetsplug/parentwork.py, decl: 0, sub: 0, line: 0 } |  |  | 0.440 |
| walker |  | 4494 | 79 | Code::CodeKey { rung: Decl, file: beetsplug/subsonicupdate.py, decl: 1, sub: 0, line: 44 } |  |  | 0.440 |
| walker |  | 4519 | 25 | Code::CodeKey { rung: Names, file: beetsplug/listenbrainz.py, decl: 0, sub: 0, line: 0 } |  |  | 0.440 |
| walker |  | 4533 | 14 | Code::CodeKey { rung: Doc, file: beetsplug/albumtypes.py, decl: 1, sub: 0, line: 29 } |  |  | 0.440 |
| walker |  | 4615 | 82 | Code::CodeKey { rung: Decl, file: beetsplug/mbsync.py, decl: 1, sub: 0, line: 25 } |  |  | 0.440 |
| walker |  | 4622 | 7 | Code::CodeKey { rung: Body, file: beetsplug/mbsync.py, decl: 2, sub: 0, line: 26 } |  |  | 0.440 |
| walker |  | 4648 | 26 | Code::CodeKey { rung: Names, file: beetsplug/titlecase.py, decl: 0, sub: 0, line: 0 } |  |  | 0.440 |
| walker |  | 4672 | 24 | Code::CodeKey { rung: Decl, file: beetsplug/titlecase.py, decl: 1, sub: 0, line: 40 } |  |  | 0.440 |
| ns | 4673 |  | 382 | config_default.yaml: autotagger thresholds and distance weights | 3.7 |  | 0.424 |
| walker |  | 4759 | 87 | Code::CodeKey { rung: Decl, file: beetsplug/mbsubmit.py, decl: 1, sub: 0, line: 33 } |  |  | 0.424 |
| walker |  | 4786 | 27 | Code::CodeKey { rung: Names, file: beetsplug/badfiles.py, decl: 0, sub: 0, line: 0 } |  |  | 0.424 |
| walker |  | 4805 | 19 | Code::CodeKey { rung: Decl, file: beetsplug/badfiles.py, decl: 1, sub: 0, line: 32 } |  |  | 0.424 |
| ns | 4813 |  | 140 | config_default.yaml: terminal UI, colour names, import layout | 3.8 |  | 0.417 |
| walker |  | 4832 | 27 | Code::CodeKey { rung: Names, file: beetsplug/bpm.py, decl: 0, sub: 0, line: 0 } |  |  | 0.417 |
| walker |  | 4889 | 57 | Code::CodeKey { rung: Decl, file: beetsplug/bpm.py, decl: 2, sub: 0, line: 47 } |  |  | 0.417 |
| walker |  | 4916 | 27 | Code::CodeKey { rung: Names, file: beetsplug/mpdupdate.py, decl: 0, sub: 0, line: 0 } |  |  | 0.417 |
| walker |  | 4973 | 57 | Code::CodeKey { rung: Decl, file: beetsplug/mpdupdate.py, decl: 1, sub: 0, line: 34 } |  |  | 0.417 |
| ns | 4986 |  | 173 | Album method roster (complete, names only) | 4.1 | 2.5 | 0.410 |
| walker |  | 5038 | 65 | Code::CodeKey { rung: Decl, file: beetsplug/mpdupdate.py, decl: 6, sub: 0, line: 66 } |  |  | 0.410 |
| walker |  | 5050 | 12 | Code::CodeKey { rung: Doc, file: beetsplug/mpdupdate.py, decl: 1, sub: 0, line: 34 } |  |  | 0.410 |
| walker |  | 5057 | 7 | Code::CodeKey { rung: Body, file: beetsplug/mpdupdate.py, decl: 5, sub: 0, line: 62 } |  |  | 0.410 |
| walker |  | 5145 | 88 | Code::CodeKey { rung: Decl, file: beetsplug/autobpm.py, decl: 1, sub: 0, line: 32 } |  |  | 0.410 |
| walker |  | 5153 | 8 | Code::CodeKey { rung: Body, file: beetsplug/types.py, decl: 2, sub: 0, line: 23 } |  |  | 0.410 |
| walker |  | 5182 | 29 | Code::CodeKey { rung: Names, file: beetsplug/subsonicplaylist.py, decl: 0, sub: 0, line: 0 } |  |  | 0.410 |
| walker |  | 5212 | 30 | Code::CodeKey { rung: Names, file: beetsplug/hook.py, decl: 0, sub: 0, line: 0 } |  |  | 0.410 |
| walker |  | 5237 | 25 | Code::CodeKey { rung: Decl, file: beetsplug/hook.py, decl: 1, sub: 0, line: 28 } |  |  | 0.410 |
| walker |  | 5268 | 31 | Code::CodeKey { rung: Decl, file: beetsplug/hook.py, decl: 3, sub: 0, line: 44 } |  |  | 0.410 |
| ns | 5290 |  | 304 | Item method roster (complete, names only) | 4.3 | 2.5 | 0.400 |
| walker |  | 5298 | 30 | Code::CodeKey { rung: Names, file: beetsplug/scrub.py, decl: 0, sub: 0, line: 0 } |  |  | 0.400 |
| walker |  | 5385 | 87 | Code::CodeKey { rung: Decl, file: beetsplug/scrub.py, decl: 2, sub: 0, line: 44 } |  |  | 0.400 |
| walker |  | 5393 | 8 | Code::CodeKey { rung: Decl, file: beetsplug/scrub.py, decl: 5, sub: 0, line: 78 } |  |  | 0.400 |
| walker |  | 5407 | 14 | Code::CodeKey { rung: Doc, file: beetsplug/scrub.py, decl: 2, sub: 0, line: 44 } |  |  | 0.400 |
| ns | 5458 |  | 168 | Item search fields and MediaFile-backed field sets | 4.4 | 2.5 | 0.394 |
| walker |  | 5504 | 97 | Code::CodeKey { rung: Decl, file: beetsplug/subsonicplaylist.py, decl: 2, sub: 0, line: 59 } |  |  | 0.394 |
| ns | 5629 |  | 171 | LibModel shared-method roster (complete after `_fields`) | 4.5 | 2.5 | 0.389 |
| ns | 5787 |  | 158 | beets.importer public exports + package docstring | 5.1 |  | 0.384 |
| walker |  | 5948 | 444 | Code::CodeKey { rung: Names, file: beetsplug/web/__init__.py, decl: 0, sub: 0, line: 0 } |  |  | 0.384 |
| walker |  | 5956 | 8 | Code::CodeKey { rung: Decl, file: beetsplug/web/__init__.py, decl: 32, sub: 0, line: 439 } |  |  | 0.384 |
| walker |  | 5965 | 9 | Code::CodeKey { rung: Decl, file: beetsplug/web/__init__.py, decl: 18, sub: 0, line: 296 } |  |  | 0.384 |
| walker |  | 5975 | 10 | Code::CodeKey { rung: Decl, file: beetsplug/web/__init__.py, decl: 30, sub: 0, line: 412 } |  |  | 0.384 |
| walker |  | 5985 | 10 | Code::CodeKey { rung: Decl, file: beetsplug/web/__init__.py, decl: 31, sub: 0, line: 423 } |  |  | 0.384 |
| walker |  | 5999 | 14 | Code::CodeKey { rung: Decl, file: beetsplug/web/__init__.py, decl: 23, sub: 0, line: 344 } |  |  | 0.384 |
| walker |  | 6014 | 15 | Code::CodeKey { rung: Decl, file: beetsplug/web/__init__.py, decl: 24, sub: 0, line: 354 } |  |  | 0.384 |
| walker |  | 6029 | 15 | Code::CodeKey { rung: Decl, file: beetsplug/web/__init__.py, decl: 29, sub: 0, line: 397 } |  |  | 0.384 |
| walker |  | 6045 | 16 | Code::CodeKey { rung: Decl, file: beetsplug/web/__init__.py, decl: 21, sub: 0, line: 317 } |  |  | 0.384 |
| walker |  | 6062 | 17 | Code::CodeKey { rung: Decl, file: beetsplug/web/__init__.py, decl: 28, sub: 0, line: 388 } |  |  | 0.384 |
| walker |  | 6084 | 22 | Code::CodeKey { rung: Decl, file: beetsplug/web/__init__.py, decl: 16, sub: 0, line: 283 } |  |  | 0.384 |
| walker |  | 6106 | 22 | Code::CodeKey { rung: Decl, file: beetsplug/web/__init__.py, decl: 33, sub: 0, line: 447 } |  |  | 0.384 |
| walker |  | 6134 | 28 | Code::CodeKey { rung: Decl, file: beetsplug/web/__init__.py, decl: 10, sub: 0, line: 252 } |  |  | 0.384 |
| ns | 6147 |  | 360 | importer/stages.py: complete stage-function roster with section banners | 5.2 |  | 0.376 |
| walker |  | 6162 | 28 | Code::CodeKey { rung: Decl, file: beetsplug/web/__init__.py, decl: 13, sub: 0, line: 268 } |  |  | 0.376 |
| walker |  | 6193 | 31 | Code::CodeKey { rung: Decl, file: beetsplug/web/__init__.py, decl: 20, sub: 0, line: 310 } |  |  | 0.376 |
| walker |  | 6224 | 31 | Code::CodeKey { rung: Decl, file: beetsplug/web/__init__.py, decl: 25, sub: 0, line: 369 } |  |  | 0.376 |
| walker |  | 6255 | 31 | Code::CodeKey { rung: Decl, file: beetsplug/web/__init__.py, decl: 26, sub: 0, line: 375 } |  |  | 0.376 |
| walker |  | 6287 | 32 | Code::CodeKey { rung: Decl, file: beetsplug/web/__init__.py, decl: 27, sub: 0, line: 382 } |  |  | 0.376 |
| walker |  | 6320 | 33 | Code::CodeKey { rung: Decl, file: beetsplug/web/__init__.py, decl: 36, sub: 0, line: 520 } |  |  | 0.376 |
| walker |  | 6358 | 38 | Code::CodeKey { rung: Decl, file: beetsplug/web/__init__.py, decl: 19, sub: 0, line: 304 } |  |  | 0.376 |
| walker |  | 6397 | 39 | Code::CodeKey { rung: Decl, file: beetsplug/web/__init__.py, decl: 22, sub: 0, line: 338 } |  |  | 0.376 |
| walker |  | 6412 | 15 | Code::CodeKey { rung: Doc, file: beetsplug/web/__init__.py, decl: 5, sub: 0, line: 117 } |  |  | 0.376 |
| walker |  | 6429 | 17 | Code::CodeKey { rung: Doc, file: beetsplug/web/__init__.py, decl: 3, sub: 0, line: 103 } |  |  | 0.376 |
| walker |  | 6446 | 17 | Code::CodeKey { rung: Doc, file: beetsplug/web/__init__.py, decl: 10, sub: 0, line: 252 } |  |  | 0.376 |
| walker |  | 6464 | 18 | Code::CodeKey { rung: Doc, file: beetsplug/web/__init__.py, decl: 13, sub: 0, line: 268 } |  |  | 0.376 |
| walker |  | 6483 | 19 | Code::CodeKey { rung: Doc, file: beetsplug/web/__init__.py, decl: 7, sub: 0, line: 176 } |  |  | 0.376 |
| walker |  | 6503 | 20 | Code::CodeKey { rung: Doc, file: beetsplug/web/__init__.py, decl: 6, sub: 0, line: 122 } |  |  | 0.376 |
| walker |  | 6533 | 30 | Code::CodeKey { rung: Doc, file: beetsplug/web/__init__.py, decl: 4, sub: 0, line: 109 } |  |  | 0.376 |
| walker |  | 6567 | 34 | Code::CodeKey { rung: Doc, file: beetsplug/web/__init__.py, decl: 8, sub: 0, line: 223 } |  |  | 0.376 |
| ns | 6572 |  | 425 | ImportSession.run(): how the pipeline is assembled | 5.3 | 5.2 | 0.365 |
| walker |  | 6585 | 18 | Code::CodeKey { rung: Doc, file: beetsplug/web/__init__.py, decl: 9, sub: 0, line: 241 } |  |  | 0.365 |
| walker |  | 6616 | 31 | Code::CodeKey { rung: Names, file: beetsplug/bareasc.py, decl: 0, sub: 0, line: 0 } |  |  | 0.365 |
| walker |  | 6644 | 28 | Code::CodeKey { rung: Decl, file: beetsplug/bareasc.py, decl: 1, sub: 0, line: 29 } |  |  | 0.365 |
| walker |  | 6652 | 8 | Code::CodeKey { rung: Decl, file: beetsplug/bareasc.py, decl: 2, sub: 0, line: 32 } |  |  | 0.365 |
| walker |  | 6707 | 55 | Code::CodeKey { rung: Decl, file: beetsplug/bareasc.py, decl: 4, sub: 0, line: 55 } |  |  | 0.365 |
| walker |  | 6721 | 14 | Code::CodeKey { rung: Doc, file: beetsplug/bareasc.py, decl: 1, sub: 0, line: 29 } |  |  | 0.365 |
| walker |  | 6737 | 16 | Code::CodeKey { rung: Doc, file: beetsplug/bareasc.py, decl: 4, sub: 0, line: 55 } |  |  | 0.365 |
| walker |  | 6768 | 31 | Code::CodeKey { rung: Names, file: beetsplug/embedart.py, decl: 0, sub: 0, line: 0 } |  |  | 0.365 |
| ns | 6829 |  | 257 | ImportSession class contract (attributes + the four decision hooks) | 5.4 | 5.3 | 0.358 |
| walker |  | 6838 | 70 | Code::CodeKey { rung: Decl, file: beetsplug/embedart.py, decl: 2, sub: 0, line: 52 } |  |  | 0.358 |
| walker |  | 6853 | 15 | Code::CodeKey { rung: Doc, file: beetsplug/embedart.py, decl: 2, sub: 0, line: 52 } |  |  | 0.358 |
| walker |  | 6884 | 31 | Code::CodeKey { rung: Names, file: beetsplug/fuzzy.py, decl: 0, sub: 0, line: 0 } |  |  | 0.358 |
| walker |  | 6909 | 25 | Code::CodeKey { rung: Decl, file: beetsplug/fuzzy.py, decl: 4, sub: 0, line: 51 } |  |  | 0.358 |
| walker |  | 6957 | 48 | Code::CodeKey { rung: Decl, file: beetsplug/fuzzy.py, decl: 1, sub: 0, line: 24 } |  |  | 0.358 |
| walker |  | 6965 | 8 | Code::CodeKey { rung: Decl, file: beetsplug/fuzzy.py, decl: 3, sub: 0, line: 29 } |  |  | 0.358 |
| walker |  | 7066 | 101 | Code::CodeKey { rung: Decl, file: beetsplug/parentwork.py, decl: 1, sub: 0, line: 34 } |  |  | 0.358 |
| walker |  | 7099 | 33 | Code::CodeKey { rung: Names, file: beetsplug/advancedrewrite.py, decl: 0, sub: 0, line: 0 } |  |  | 0.358 |
| ns | 7103 |  | 274 | importer/tasks.py: Action enum and the complete task class roster | 5.5 |  | 0.352 |
| walker |  | 7123 | 24 | Code::CodeKey { rung: Decl, file: beetsplug/advancedrewrite.py, decl: 2, sub: 0, line: 57 } |  |  | 0.352 |
| walker |  | 7137 | 14 | Code::CodeKey { rung: Doc, file: beetsplug/advancedrewrite.py, decl: 2, sub: 0, line: 57 } |  |  | 0.352 |
| walker |  | 7170 | 33 | Code::CodeKey { rung: Names, file: beetsplug/kodiupdate.py, decl: 0, sub: 0, line: 0 } |  |  | 0.352 |
| walker |  | 7212 | 42 | Code::CodeKey { rung: Decl, file: beetsplug/kodiupdate.py, decl: 2, sub: 0, line: 53 } |  |  | 0.352 |
| walker |  | 7293 | 81 | Toml::PackageMetadata { file: pyproject.toml } |  |  | 0.359 |
| ns | 7349 |  | 246 | BeetsPlugin base class: docstring and declared attributes | 6.1 |  | 0.354 |
| walker |  | 7445 | 152 | Code::CodeKey { rung: Names, file: beets/importer/__init__.py, decl: 0, sub: 0, line: 0 } |  |  | 0.363 |
| walker |  | 7464 | 19 | Code::CodeKey { rung: Doc, file: beetsplug/hook.py, decl: 3, sub: 0, line: 44 } |  |  | 0.363 |
| walker |  | 7483 | 19 | Code::CodeKey { rung: Doc, file: beetsplug/kodiupdate.py, decl: 1, sub: 0, line: 31 } |  |  | 0.363 |
| walker |  | 7595 | 112 | Code::CodeKey { rung: Decl, file: beetsplug/bpsync.py, decl: 1, sub: 0, line: 26 } |  |  | 0.360 |
| ns | 7595 |  | 246 | BeetsPlugin method roster (complete) | 6.2 | 6.1 | 0.360 |
| walker |  | 7603 | 8 | Code::CodeKey { rung: Decl, file: beetsplug/bpsync.py, decl: 6, sub: 0, line: 103 } |  |  | 0.360 |
| walker |  | 7758 | 155 | Code::CodeKey { rung: Names, file: beetsplug/tidal/__init__.py, decl: 0, sub: 0, line: 0 } |  |  | 0.360 |
| walker |  | 7837 | 79 | Code::CodeKey { rung: Decl, file: beetsplug/tidal/__init__.py, decl: 34, sub: 0, line: 494 } |  |  | 0.360 |
| ns | 8017 |  | 422 | The complete `EventType` literal — every plugin event name | 6.3 | 6.1 | 0.350 |
| walker |  | 8054 | 217 | Code::CodeKey { rung: Decl, file: beetsplug/tidal/__init__.py, decl: 2, sub: 0, line: 37 } |  |  | 0.350 |
| walker |  | 8063 | 9 | Code::CodeKey { rung: Decl, file: beetsplug/tidal/__init__.py, decl: 4, sub: 0, line: 53 } |  |  | 0.350 |
| walker |  | 8071 | 8 | Code::CodeKey { rung: Decl, file: beetsplug/tidal/__init__.py, decl: 14, sub: 0, line: 161 } |  |  | 0.350 |
| walker |  | 8102 | 31 | Code::CodeKey { rung: Decl, file: beetsplug/tidal/__init__.py, decl: 13, sub: 0, line: 142 } |  |  | 0.350 |
| walker |  | 8142 | 40 | Code::CodeKey { rung: Decl, file: beetsplug/tidal/__init__.py, decl: 12, sub: 0, line: 119 } |  |  | 0.350 |
| walker |  | 8153 | 11 | Code::CodeKey { rung: Doc, file: beetsplug/tidal/__init__.py, decl: 14, sub: 0, line: 161 } |  |  | 0.350 |
| walker |  | 8213 | 60 | Code::CodeKey { rung: Decl, file: beetsplug/autobpm.py, decl: 6, sub: 0, line: 95 } |  |  | 0.350 |
| ns | 8321 |  | 304 | plugins.py module-level API roster (complete) | 6.4 |  | 0.345 |
| walker |  | 8399 | 186 | Code::CodeKey { rung: Names, file: beetsplug/metasync/__init__.py, decl: 0, sub: 0, line: 0 } |  |  | 0.345 |
| walker |  | 8428 | 29 | Code::CodeKey { rung: Decl, file: beetsplug/metasync/__init__.py, decl: 2, sub: 0, line: 34 } |  |  | 0.345 |
| walker |  | 8480 | 52 | Code::CodeKey { rung: Decl, file: beetsplug/metasync/__init__.py, decl: 3, sub: 0, line: 40 } |  |  | 0.345 |
| walker |  | 8489 | 9 | Code::CodeKey { rung: Decl, file: beetsplug/metasync/__init__.py, decl: 5, sub: 0, line: 47 } |  |  | 0.345 |
| walker |  | 8545 | 56 | Code::CodeKey { rung: Decl, file: beetsplug/metasync/__init__.py, decl: 9, sub: 0, line: 76 } |  |  | 0.345 |
| walker |  | 8550 | 5 | Code::CodeKey { rung: Body, file: beetsplug/metasync/__init__.py, decl: 5, sub: 0, line: 47 } |  |  | 0.345 |
| walker |  | 8569 | 19 | Code::CodeKey { rung: Doc, file: beetsplug/metasync/__init__.py, decl: 8, sub: 0, line: 68 } |  |  | 0.345 |
| walker |  | 8583 | 14 | Code::CodeKey { rung: Doc, file: beetsplug/metasync/__init__.py, decl: 12, sub: 0, line: 104 } |  |  | 0.345 |
| walker |  | 8628 | 45 | Code::CodeKey { rung: Doc, file: beetsplug/metasync/__init__.py, decl: 6, sub: 0, line: 52 } |  |  | 0.345 |
| ns | 8633 |  | 312 | MetadataSourcePlugin: the metadata-backend contract | 6.5 |  | 0.341 |
| walker |  | 8635 | 7 | Code::CodeKey { rung: Body, file: beetsplug/metasync/__init__.py, decl: 10, sub: 0, line: 79 } |  |  | 0.341 |
| walker |  | 8671 | 36 | Code::CodeKey { rung: Names, file: beetsplug/duplicates.py, decl: 0, sub: 0, line: 0 } |  |  | 0.341 |
| walker |  | 8760 | 89 | Code::CodeKey { rung: Doc, file: beets/ui/__init__.py, decl: 8, sub: 0, line: 122 } |  |  | 0.341 |
| ns | 8830 |  | 197 | Query-string prefixes: how `beet ls artist:foo` is parsed | 7.1 |  | 0.337 |
| walker |  | 8877 | 117 | Code::CodeKey { rung: Decl, file: beetsplug/replace.py, decl: 1, sub: 0, line: 16 } |  |  | 0.337 |
| walker |  | 8914 | 37 | Code::CodeKey { rung: Names, file: beetsplug/deezer.py, decl: 0, sub: 0, line: 0 } |  |  | 0.337 |
| ns | 9187 |  | 357 | dbcore/query.py class roster (complete) | 7.2 |  | 0.330 |
| walker |  | 9233 | 319 | Code::CodeKey { rung: Names, file: beets/util/__init__.py, decl: 0, sub: 0, line: 0 } |  |  | 0.330 |
| walker |  | 9267 | 34 | Code::CodeKey { rung: Decl, file: beets/util/__init__.py, decl: 14, sub: 0, line: 130 } |  |  | 0.330 |
| walker |  | 9363 | 96 | Code::CodeKey { rung: Decl, file: beets/util/__init__.py, decl: 8, sub: 0, line: 74 } |  |  | 0.330 |
| walker |  | 9373 | 10 | Code::CodeKey { rung: Body, file: beets/util/__init__.py, decl: 12, sub: 0, line: 115 } |  |  | 0.330 |
| walker |  | 9396 | 23 | Code::CodeKey { rung: Body, file: beets/util/__init__.py, decl: 15, sub: 0, line: 136 } |  |  | 0.330 |
| walker |  | 9424 | 28 | Code::CodeKey { rung: Doc, file: beets/util/__init__.py, decl: 12, sub: 0, line: 115 } |  |  | 0.330 |
| walker |  | 9478 | 54 | Code::CodeKey { rung: Doc, file: beets/util/__init__.py, decl: 14, sub: 0, line: 130 } |  |  | 0.330 |
| walker |  | 9519 | 41 | Code::CodeKey { rung: Doc, file: beets/util/__init__.py, decl: 13, sub: 0, line: 121 } |  |  | 0.330 |
| walker |  | 9532 | 13 | Code::CodeKey { rung: Doc, file: beets/util/__init__.py, decl: 11, sub: 0, line: 104 } |  |  | 0.330 |
| ns | 9539 |  | 352 | beets.autotag exports, Recommendation, Proposal | 8.1 |  | 0.324 |
| walker |  | 9543 | 11 | Code::CodeKey { rung: Doc, file: beetsplug/albumtypes.py, decl: 2, sub: 0, line: 32 } |  |  | 0.324 |
| walker |  | 9554 | 11 | Code::CodeKey { rung: Doc, file: beetsplug/scrub.py, decl: 8, sub: 0, line: 147 } |  |  | 0.324 |
| walker |  | 9565 | 11 | Code::CodeKey { rung: Doc, file: beetsplug/substitute.py, decl: 2, sub: 0, line: 33 } |  |  | 0.324 |
| walker |  | 9737 | 172 | Code::CodeKey { rung: Names, file: beets/ui/commands/import_/__init__.py, decl: 0, sub: 0, line: 0 } |  |  | 0.324 |
| walker |  | 9763 | 26 | Code::CodeKey { rung: Decl, file: beets/ui/commands/import_/__init__.py, decl: 7, sub: 0, line: 160 } |  |  | 0.324 |
| ns | 9775 |  | 236 | autotag/hooks.py and distance.py symbol roster | 8.2 |  | 0.321 |
| walker |  | 9781 | 18 | Code::CodeKey { rung: Doc, file: beets/ui/commands/import_/__init__.py, decl: 3, sub: 0, line: 34 } |  |  | 0.321 |
| walker |  | 9813 | 32 | Code::CodeKey { rung: Doc, file: beets/ui/commands/import_/__init__.py, decl: 4, sub: 0, line: 49 } |  |  | 0.321 |
| walker |  | 9846 | 33 | Code::CodeKey { rung: Doc, file: beets/ui/commands/import_/__init__.py, decl: 2, sub: 0, line: 14 } |  |  | 0.321 |
| walker |  | 9866 | 20 | Code::CodeKey { rung: Names, file: beets/library/fields.py, decl: 0, sub: 0, line: 0 } |  |  | 0.321 |
| walker |  | 9886 | 20 | Code::CodeKey { rung: Names, file: beets/util/hidden.py, decl: 0, sub: 0, line: 0 } |  |  | 0.321 |
| walker |  | 9897 | 11 | Code::CodeKey { rung: Body, file: beetsplug/keyfinder.py, decl: 5, sub: 0, line: 48 } |  |  | 0.321 |
| walker |  | 9919 | 22 | Code::CodeKey { rung: Doc, file: beetsplug/hook.py, decl: 1, sub: 0, line: 28 } |  |  | 0.321 |
| ns | 9970 |  | 195 | beets/util/__init__.py: classes and the core path/file-operation helpers | 9.1 |  | 0.317 |
