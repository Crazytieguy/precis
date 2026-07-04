Score(3000)=0.677 I=0.805 C=0.569 ns_rows≤3K=17/50 (reached=8 partial=2 missing=7)

| source | ns_cum | walker_cum | marginal | descriptor | id | predecessor | Score(B=cum) |
|:-------|-------:|-----------:|---------:|:-----------|:---|:------------|-------------:|
| walker |  | 83 | 83 | listing of '.' |  |  | 0.000 |
| ns | 98 |  | 98 | README lede + tagline | 1.1 |  | 0.000 |
| walker |  | 108 | 25 | entry-point scripts in pyproject.toml |  |  | 0.000 |
| walker |  | 127 | 19 | listing of 'extra' |  |  | 0.000 |
| ns | 181 |  | 83 | Top-level repo listing | 1.2 |  | 0.482 |
| walker |  | 194 | 67 | listing of 'beets' |  |  | 0.516 |
| walker |  | 203 | 9 | listing of 'beets/ui' |  |  | 0.516 |
| ns | 211 |  | 30 | Entry-point scripts (`beet = beets.ui:main`) | 1.3 |  | 0.508 |
| ns | 303 |  | 92 | `python -m beets` shim | 1.4 |  | 0.427 |
| ns | 390 |  | 87 | Package version + global config singleton | 1.5 |  | 0.398 |
| walker |  | 488 | 285 | python imports in beets/__init__.py |  |  | 0.406 |
| walker |  | 506 | 18 | listing of 'beets/autotag' |  |  | 0.407 |
| walker |  | 528 | 22 | listing of 'beets/importer' |  |  | 0.410 |
| walker |  | 543 | 15 | python decl names surface in beets/autotag/__init__.py |  |  | 0.410 |
| walker |  | 543 | 0 | python decl at beets/autotag/__init__.py:28 |  |  | 0.410 |
| walker |  | 571 | 28 | listing of 'beets/dbcore' |  |  | 0.356 |
| ns | 571 |  | 181 | Project metadata + Python version constraint | 1.6 |  | 0.356 |
| walker |  | 601 | 30 | listing of 'beets/library' |  |  | 0.369 |
| walker |  | 629 | 28 | python decl names surface in beets/library/__init__.py |  |  | 0.369 |
| walker |  | 629 | 0 | python decl at beets/library/__init__.py:15 |  |  | 0.369 |
| walker |  | 696 | 67 | listing of 'beets/ui/commands' |  |  | 0.377 |
| walker |  | 710 | 14 | listing of 'beets/ui/commands/import_' |  |  | 0.379 |
| walker |  | 733 | 23 | python decl names surface in beets/ui/commands/__init__.py |  |  | 0.379 |
| walker |  | 733 | 0 | python decl at beets/ui/commands/__init__.py:36 |  |  | 0.379 |
| walker |  | 741 | 8 | python decl doc at beets/ui/commands/__init__.py:36 |  |  | 0.379 |
| ns | 802 |  | 231 | Core (non-optional) runtime deps | 1.7 |  | 0.329 |
| walker |  | 828 | 87 | python imports in beets/ui/commands/import_/__init__.py |  |  | 0.329 |
| ns | 869 |  | 67 | `beets/` core package listing | 2.1 |  | 0.384 |
| walker |  | 903 | 75 | listing of 'beets/util' |  |  | 0.403 |
| walker |  | 913 | 10 | python imports in beets/util/__init__.py |  |  | 0.403 |
| walker |  | 924 | 11 | python imports #1 in beets/util/__init__.py |  |  | 0.403 |
| walker |  | 932 | 8 | python imports #2 in beets/util/__init__.py |  |  | 0.403 |
| walker |  | 939 | 7 | python imports #3 in beets/util/__init__.py |  |  | 0.403 |
| walker |  | 947 | 8 | python imports #4 in beets/util/__init__.py |  |  | 0.403 |
| walker |  | 954 | 7 | python imports #5 in beets/util/__init__.py |  |  | 0.403 |
| walker |  | 961 | 7 | python imports #6 in beets/util/__init__.py |  |  | 0.403 |
| ns | 967 |  | 98 | Core subpackage listings: dbcore, library, autotag, importer | 2.2 |  | 0.444 |
| walker |  | 968 | 7 | python imports #7 in beets/util/__init__.py |  |  | 0.444 |
| walker |  | 1015 | 47 | python decl names surface in beets/__init__.py |  |  | 0.458 |
| walker |  | 1015 | 0 | python decl at beets/__init__.py:26 |  |  | 0.458 |
| walker |  | 1015 | 0 | python decl at beets/__init__.py:35 |  |  | 0.458 |
| walker |  | 1023 | 8 | python decl doc at beets/__init__.py:26 |  |  | 0.458 |
| walker |  | 1049 | 26 | python method sigs in beets/__init__.py |  |  | 0.458 |
| walker |  | 1049 | 0 | python method at beets/__init__.py:40 |  |  | 0.458 |
| walker |  | 1068 | 19 | python decl body at beets/library/__init__.py:15 body 16 |  |  | 0.458 |
| walker |  | 1076 | 8 | python imports #8 in beets/util/__init__.py |  |  | 0.458 |
| walker |  | 1083 | 7 | python imports #9 in beets/util/__init__.py |  |  | 0.458 |
| walker |  | 1123 | 40 | python decl doc at beets/__init__.py:35 |  |  | 0.472 |
| ns | 1132 |  | 165 | UI + util + commands listings | 2.3 |  | 0.500 |
| walker |  | 1299 | 176 | python imports in beets/library/__init__.py |  |  | 0.509 |
| walker |  | 1306 | 7 | python imports #10 in beets/util/__init__.py |  |  | 0.509 |
| walker |  | 1313 | 7 | python imports #11 in beets/util/__init__.py |  |  | 0.509 |
| walker |  | 1325 | 12 | python decl names surface in beets/library/library.py |  |  | 0.509 |
| walker |  | 1325 | 0 | python decl at beets/library/library.py:20 |  |  | 0.509 |
| walker |  | 1338 | 13 | python decl doc at beets/library/library.py:20 |  |  | 0.509 |
| walker |  | 1405 | 67 | [package] in pyproject.toml |  |  | 0.464 |
| ns | 1405 |  | 273 | `beets.dbcore` public re-exports | 2.4 |  | 0.464 |
| walker |  | 1412 | 7 | python imports #12 in beets/util/__init__.py |  |  | 0.464 |
| walker |  | 1528 | 116 | python decl names surface in beets/ui/commands/import_/__init__.py |  |  | 0.464 |
| walker |  | 1528 | 0 | python decl at beets/ui/commands/import_/__init__.py:14 |  |  | 0.464 |
| walker |  | 1528 | 0 | python decl at beets/ui/commands/import_/__init__.py:34 |  |  | 0.464 |
| walker |  | 1528 | 0 | python decl at beets/ui/commands/import_/__init__.py:49 |  |  | 0.464 |
| walker |  | 1528 | 0 | python decl at beets/ui/commands/import_/__init__.py:81 |  |  | 0.464 |
| walker |  | 1528 | 0 | python decl at beets/ui/commands/import_/__init__.py:134 |  |  | 0.464 |
| walker |  | 1544 | 16 | python decl doc at beets/ui/commands/import_/__init__.py:34 |  |  | 0.464 |
| walker |  | 1570 | 26 | python decl at beets/ui/commands/import_/__init__.py:160 |  |  | 0.464 |
| walker |  | 1600 | 30 | python decl doc at beets/ui/commands/import_/__init__.py:49 |  |  | 0.464 |
| walker |  | 1631 | 31 | python decl doc at beets/ui/commands/import_/__init__.py:14 |  |  | 0.464 |
| walker |  | 1708 | 77 | listing of 'docs' |  |  | 0.466 |
| ns | 1722 |  | 317 | `beets.library` public re-exports | 2.5 |  | 0.458 |
| walker |  | 1730 | 22 | listing of 'docs/api' |  |  | 0.459 |
| walker |  | 1756 | 26 | listing of 'docs/guides' |  |  | 0.461 |
| walker |  | 1782 | 26 | listing of 'docs/reference' |  |  | 0.464 |
| walker |  | 1810 | 28 | listing of 'docs/dev' |  |  | 0.467 |
| walker |  | 1817 | 7 | python imports in beets/util/units.py |  |  | 0.467 |
| walker |  | 1833 | 16 | python decl names surface in beets/library/fields.py |  |  | 0.467 |
| walker |  | 1840 | 7 | python imports #13 in beets/util/__init__.py |  |  | 0.467 |
| walker |  | 1844 | 4 | listing of 'docs/_templates' |  |  | 0.467 |
| walker |  | 1865 | 21 | listing of 'docs/_templates/autosummary' |  |  | 0.467 |
| walker |  | 1869 | 4 | listing of 'docs/extensions' |  |  | 0.467 |
| walker |  | 1938 | 69 | python decl at beets/library/__init__.py:8 |  |  | 0.489 |
| walker |  | 1953 | 15 | listing of 'beets/test' |  |  | 0.489 |
| walker |  | 1971 | 18 | python decl names surface in beets/util/hidden.py |  |  | 0.489 |
| walker |  | 1971 | 0 | python decl at beets/util/hidden.py:25 |  |  | 0.489 |
| walker |  | 1995 | 24 | listing of 'docs/dev/plugins' |  |  | 0.492 |
| ns | 2088 |  | 366 | `beets.autotag` public re-exports | 2.6 |  | 0.450 |
| ns | 2306 |  | 218 | `beets.importer` public re-exports | 2.7 |  | 0.426 |
| ns | 2509 |  | 203 | Docs tree + reference + dev-doc table-of-contents | 2.8 |  | 0.453 |
| walker |  | 2585 | 590 | README headline in README.rst |  |  | 0.677 |
| ns | 2681 |  | 172 | `UserError` + `Subcommand` class header | 3.1 |  | 0.661 |
| ns | 2928 |  | 247 | Top-level fns in `beets/ui/__init__.py` (signature heads only) | 3.2 |  | 0.639 |
| walker |  | 2973 | 388 | python imports in beets/importer/__init__.py |  |  | 0.677 |
| walker |  | 2986 | 13 | python decl names surface in beets/ui/commands/help.py |  |  | 0.677 |
| walker |  | 2986 | 0 | python decl at beets/ui/commands/help.py:6 |  |  | 0.677 |
| ns | 3376 |  | 448 | `main()` top-level exception handlers | 3.3 | 3.2 | 0.642 |
| walker |  | 3381 | 395 | python imports in beets/autotag/__init__.py |  |  | 0.665 |
| walker |  | 3391 | 10 | python imports in beets/library/exceptions.py |  |  | 0.665 |
| ns | 3470 |  | 94 | `_raw_main()` global-option setup | 3.4 | 3.2 | 0.659 |
| ns | 3728 |  | 258 | `_raw_main()` parse + dispatch tail | 3.5 | 3.4 | 0.638 |
| walker |  | 3800 | 409 | python imports in beets/ui/commands/__init__.py |  |  | 0.639 |
| walker |  | 3809 | 9 | python decl names surface #1 in beets/util/color.py |  |  | 0.639 |
| walker |  | 3833 | 24 | python decl names surface in beets/util/m3u.py |  |  | 0.639 |
| walker |  | 3833 | 0 | python decl at beets/util/m3u.py:22 |  |  | 0.639 |
| walker |  | 3833 | 0 | python decl at beets/util/m3u.py:28 |  |  | 0.639 |
| walker |  | 3840 | 7 | python class body at beets/util/m3u.py:22 |  |  | 0.639 |
| walker |  | 3857 | 17 | python decl doc at beets/util/m3u.py:22 |  |  | 0.639 |
| walker |  | 3875 | 18 | python decl doc at beets/util/m3u.py:28 |  |  | 0.639 |
| walker |  | 3884 | 9 | python imports #14 in beets/util/__init__.py |  |  | 0.639 |
| walker |  | 3912 | 28 | python method sigs in beets/ui/commands/help.py |  |  | 0.639 |
| walker |  | 3912 | 0 | python method at beets/ui/commands/help.py:7 |  |  | 0.639 |
| walker |  | 3912 | 0 | python method at beets/ui/commands/help.py:14 |  |  | 0.639 |
| walker |  | 3924 | 12 | python imports in beets/library/fields.py |  |  | 0.639 |
| ns | 3939 |  | 211 | `_setup()` body: load plugins, build commands, open library | 3.6 | 3.2 | 0.622 |
| walker |  | 3946 | 22 | python imports in beets/context.py |  |  | 0.622 |
| ns | 4347 |  | 408 | `default_commands` list (built-in subcommands) | 3.7 |  | 0.600 |
| walker |  | 4447 | 501 | python imports in beets/ui/__init__.py |  |  | 0.600 |
| ns | 4535 |  | 188 | Built-in command registrations (one `Subcommand` line per cmd) | 3.8 |  | 0.591 |
| ns | 4796 |  | 261 | Smallest command as concrete template (`list`) | 3.9 | 3.8 | 0.574 |
| walker |  | 4953 | 506 | python imports in beets/dbcore/__init__.py |  |  | 0.626 |
| ns | 4982 |  | 186 | `Library` class header + `_models` + `_migrations` | 4.1 |  | 0.618 |
| walker |  | 5118 | 165 | python decl names surface in beets/ui/__init__.py |  |  | 0.621 |
| walker |  | 5118 | 0 | python decl at beets/ui/__init__.py:71 |  |  | 0.621 |
| walker |  | 5118 | 0 | python decl at beets/ui/__init__.py:80 |  |  | 0.621 |
| walker |  | 5118 | 0 | python decl at beets/ui/__init__.py:85 |  |  | 0.621 |
| walker |  | 5118 | 0 | python decl at beets/ui/__init__.py:90 |  |  | 0.621 |
| walker |  | 5118 | 0 | python decl at beets/ui/__init__.py:111 |  |  | 0.621 |
| walker |  | 5118 | 0 | python decl at beets/ui/__init__.py:122 |  |  | 0.621 |
| walker |  | 5118 | 0 | python decl at beets/ui/__init__.py:150 |  |  | 0.621 |
| walker |  | 5118 | 0 | python decl at beets/ui/__init__.py:160 |  |  | 0.621 |
| walker |  | 5118 | 0 | python decl at beets/ui/__init__.py:167 |  |  | 0.621 |
| walker |  | 5118 | 0 | python decl at beets/ui/__init__.py:187 |  |  | 0.621 |
| walker |  | 5118 | 0 | python decl at beets/ui/__init__.py:383 |  |  | 0.621 |
| ns | 5125 |  | 143 | `Library` query/fetch method headers | 4.2 | 4.1 | 0.614 |
| walker |  | 5153 | 35 | python decl doc at beets/ui/__init__.py:71 |  |  | 0.616 |
| walker |  | 5175 | 22 | python decl body at beets/ui/__init__.py:160 body 164 |  |  | 0.616 |
| walker |  | 5237 | 62 | python decl at beets/ui/__init__.py:207 |  |  | 0.616 |
| walker |  | 5274 | 37 | python decl doc at beets/ui/__init__.py:160 |  |  | 0.616 |
| walker |  | 5286 | 12 | python decl body at beets/ui/__init__.py:80 body 82 |  |  | 0.616 |
| walker |  | 5298 | 12 | python decl body at beets/ui/__init__.py:85 body 87 |  |  | 0.616 |
| walker |  | 5316 | 18 | python decl doc at beets/ui/__init__.py:150 |  |  | 0.616 |
| ns | 5328 |  | 203 | `LibModel` + `Item` + `Album` class headers | 4.3 |  | 0.603 |
| walker |  | 5335 | 19 | python decl doc at beets/ui/__init__.py:80 |  |  | 0.603 |
| walker |  | 5354 | 19 | python decl doc at beets/ui/__init__.py:85 |  |  | 0.603 |
| walker |  | 5407 | 53 | python decl doc at beets/ui/__init__.py:383 |  |  | 0.603 |
| ns | 5412 |  | 84 | `_search_fields` for Item + Album (default query targets) | 4.4 | 4.3 | 0.597 |
| walker |  | 5416 | 9 | python decl body at beets/ui/__init__.py:111 body 119 |  |  | 0.597 |
| walker |  | 5487 | 71 | python decl doc at beets/ui/__init__.py:187 |  |  | 0.597 |
| walker |  | 5566 | 79 | python decl doc at beets/ui/__init__.py:111 |  |  | 0.597 |
| ns | 5614 |  | 202 | `TYPE_BY_FIELD` lede + first dozen fields | 4.5 |  | 0.589 |
| walker |  | 5653 | 87 | python decl doc at beets/ui/__init__.py:122 |  |  | 0.589 |
| walker |  | 5707 | 54 | python decl body at beets/ui/__init__.py:167 body 177 |  |  | 0.589 |
| walker |  | 5766 | 59 | python decl body at beets/__init__.py:26 body 28 |  |  | 0.589 |
| walker |  | 5875 | 109 | python decl at beets/ui/commands/__init__.py:50 |  |  | 0.605 |
| ns | 5986 |  | 372 | `parse_query_parts` + `parse_query_string` (library-level query entry) | 4.6 |  | 0.587 |
| walker |  | 6055 | 180 | python decl names surface in beets/util/__init__.py |  |  | 0.587 |
| walker |  | 6055 | 0 | python decl at beets/util/__init__.py:74 |  |  | 0.587 |
| walker |  | 6055 | 0 | python decl at beets/util/__init__.py:130 |  |  | 0.587 |
| walker |  | 6055 | 0 | python decl at beets/util/__init__.py:158 |  |  | 0.587 |
| walker |  | 6055 | 0 | python decl at beets/util/__init__.py:169 |  |  | 0.587 |
| walker |  | 6055 | 0 | python decl at beets/util/__init__.py:175 |  |  | 0.587 |
| walker |  | 6055 | 0 | python decl at beets/util/__init__.py:184 |  |  | 0.587 |
| walker |  | 6055 | 0 | python decl at beets/util/__init__.py:273 |  |  | 0.587 |
| walker |  | 6055 | 0 | python decl at beets/util/__init__.py:280 |  |  | 0.587 |
| walker |  | 6055 | 0 | python decl at beets/util/__init__.py:294 |  |  | 0.587 |
| walker |  | 6055 | 0 | python decl at beets/util/__init__.py:356 |  |  | 0.587 |
| walker |  | 6075 | 20 | python class body at beets/util/__init__.py:74 |  |  | 0.587 |
| walker |  | 6093 | 18 | python decl doc at beets/util/__init__.py:158 |  |  | 0.587 |
| walker |  | 6118 | 25 | python class body at beets/util/__init__.py:169 |  |  | 0.587 |
| walker |  | 6133 | 15 | python decl body at beets/util/__init__.py:273 body 277 |  |  | 0.587 |
| walker |  | 6182 | 49 | python decl at beets/util/__init__.py:309 |  |  | 0.587 |
| walker |  | 6211 | 29 | python decl doc at beets/util/__init__.py:273 |  |  | 0.587 |
| walker |  | 6241 | 30 | python decl doc at beets/util/__init__.py:175 |  |  | 0.587 |
| walker |  | 6273 | 32 | python decl doc at beets/util/__init__.py:280 |  |  | 0.587 |
| walker |  | 6313 | 40 | python decl doc at beets/util/__init__.py:294 |  |  | 0.587 |
| ns | 6325 |  | 339 | `LibModel` + `Item` + `Album` method names (locations) | 4.7 | 4.3 | 0.575 |
| walker |  | 6381 | 68 | python decl at beets/util/__init__.py:208 |  |  | 0.575 |
| walker |  | 6447 | 66 | python class body at beets/util/__init__.py:158 |  |  | 0.575 |
| walker |  | 6501 | 54 | python decl doc at beets/util/__init__.py:130 |  |  | 0.575 |
| walker |  | 6509 | 8 | python decl body at beets/util/__init__.py:294 body 306 |  |  | 0.575 |
| ns | 6560 |  | 235 | `DefaultTemplateFunctions` header + `tmpl_*` method locations | 4.8 |  | 0.567 |
| walker |  | 6591 | 82 | python decl doc at beets/util/__init__.py:356 |  |  | 0.567 |
| ns | 6673 |  | 113 | Library migration class headers (full chain) | 4.9 |  | 0.563 |
| walker |  | 6674 | 83 | python decl doc at beets/util/__init__.py:208 |  |  | 0.563 |
| walker |  | 6770 | 96 | python decl doc at beets/util/__init__.py:184 |  |  | 0.563 |
| ns | 6903 |  | 230 | `Model` + `Database` class headers | 5.1 |  | 0.556 |
| walker |  | 6979 | 209 | python method sigs in beets/util/__init__.py |  |  | 0.556 |
| walker |  | 6979 | 0 | python method at beets/util/__init__.py:90 |  |  | 0.556 |
| walker |  | 6979 | 0 | python method at beets/util/__init__.py:96 |  |  | 0.556 |
| walker |  | 6979 | 0 | python method at beets/util/__init__.py:104 |  |  | 0.556 |
| walker |  | 6979 | 0 | python method at beets/util/__init__.py:115 |  |  | 0.556 |
| walker |  | 6979 | 0 | python method at beets/util/__init__.py:121 |  |  | 0.556 |
| walker |  | 6979 | 0 | python method at beets/util/__init__.py:136 |  |  | 0.556 |
| walker |  | 6979 | 0 | python method at beets/util/__init__.py:1094 |  |  | 0.556 |
| walker |  | 6979 | 0 | python method at beets/util/__init__.py:1098 |  |  | 0.556 |
| walker |  | 6979 | 0 | python method at beets/util/__init__.py:1102 |  |  | 0.556 |
| walker |  | 6979 | 0 | python method at beets/util/__init__.py:1145 |  |  | 0.556 |
| walker |  | 6994 | 15 | python method at beets/util/__init__.py:140 |  |  | 0.556 |
| ns | 6999 |  | 96 | Top-level classes in `beets/dbcore/db.py` (locations) | 5.2 |  | 0.551 |
| walker |  | 7008 | 14 | python method doc at beets/util/__init__.py:1094 |  |  | 0.551 |
| walker |  | 7018 | 10 | python method body at beets/util/__init__.py:115 body 119 |  |  | 0.551 |
| walker |  | 7033 | 15 | python method doc at beets/util/__init__.py:1098 |  |  | 0.551 |
| walker |  | 7044 | 11 | python method body at beets/util/__init__.py:1098 body 1100 |  |  | 0.551 |
| walker |  | 7061 | 17 | python method doc at beets/util/__init__.py:1102 |  |  | 0.551 |
| walker |  | 7072 | 11 | python method doc at beets/util/__init__.py:104 |  |  | 0.551 |
| walker |  | 7089 | 17 | python method body at beets/util/__init__.py:1094 body 1096 |  |  | 0.551 |
| walker |  | 7117 | 28 | python method doc at beets/util/__init__.py:115 |  |  | 0.551 |
| walker |  | 7134 | 17 | python method doc at beets/util/__init__.py:96 |  |  | 0.551 |
| walker |  | 7173 | 39 | python method doc at beets/util/__init__.py:121 |  |  | 0.551 |
| walker |  | 7196 | 23 | python method body at beets/util/__init__.py:136 body 137 |  |  | 0.551 |
| walker |  | 7215 | 19 | python decl names surface in beets/ui/commands/utils.py |  |  | 0.551 |
| walker |  | 7215 | 0 | python decl at beets/ui/commands/utils.py:6 |  |  | 0.551 |
| ns | 7271 |  | 272 | `Query` hierarchy class headers (full chain) | 5.3 |  | 0.541 |
| walker |  | 7333 | 118 | python decl doc at beets/ui/__init__.py:167 |  |  | 0.541 |
| ns | 7346 |  | 75 | `Sort` class hierarchy (locations) | 5.4 |  | 0.538 |
| walker |  | 7451 | 118 | python decl doc at beets/util/__init__.py:309 |  |  | 0.538 |
| walker |  | 7527 | 76 | python decl names surface in beets/context.py |  |  | 0.538 |
| walker |  | 7527 | 0 | python decl at beets/context.py:8 |  |  | 0.538 |
| walker |  | 7527 | 0 | python decl at beets/context.py:13 |  |  | 0.538 |
| walker |  | 7536 | 9 | python decl at beets/context.py:18 |  |  | 0.538 |
| walker |  | 7547 | 11 | python decl doc at beets/context.py:8 |  |  | 0.531 |
| ns | 7547 |  | 201 | `Type` hierarchy class headers | 5.5 |  | 0.531 |
| walker |  | 7558 | 11 | python decl doc at beets/context.py:13 |  |  | 0.531 |
| walker |  | 7570 | 12 | python decl body at beets/context.py:8 body 10 |  |  | 0.531 |
| walker |  | 7582 | 12 | python decl body at beets/context.py:13 body 15 |  |  | 0.531 |
| walker |  | 7600 | 18 | python decl doc at beets/context.py:18 |  |  | 0.531 |
| ns | 7601 |  | 54 | `queryparse` top-level fn locations | 5.6 |  | 0.528 |
| walker |  | 7651 | 51 | python decl doc at beets/ui/commands/import_/__init__.py:134 |  |  | 0.528 |
| walker |  | 7709 | 58 | python method sigs in beets/util/m3u.py |  |  | 0.528 |
| walker |  | 7709 | 0 | python method at beets/util/m3u.py:31 |  |  | 0.528 |
| walker |  | 7709 | 0 | python method at beets/util/m3u.py:43 |  |  | 0.528 |
| walker |  | 7709 | 0 | python method at beets/util/m3u.py:63 |  |  | 0.528 |
| walker |  | 7709 | 0 | python method at beets/util/m3u.py:77 |  |  | 0.528 |
| walker |  | 7727 | 18 | python method doc at beets/util/m3u.py:43 |  |  | 0.528 |
| ns | 7733 |  | 132 | `PARSE_QUERY_PART_REGEX` — the query-string syntax | 5.7 | 5.6 | 0.524 |
| walker |  | 7759 | 32 | python decl doc at beets/util/hidden.py:25 |  |  | 0.524 |
| walker |  | 7791 | 32 | python decl names surface in beets/util/config.py |  |  | 0.524 |
| walker |  | 7791 | 0 | python decl at beets/util/config.py:78 |  |  | 0.524 |
| walker |  | 7806 | 15 | python method sigs in beets/util/config.py |  |  | 0.524 |
| walker |  | 7806 | 0 | python method at beets/util/config.py:79 |  |  | 0.524 |
| walker |  | 7829 | 23 | python decl at beets/util/config.py:9 |  |  | 0.524 |
| ns | 7864 |  | 131 | Autotag entry points: `tag_album` + `tag_item` signatures | 6.1 |  | 0.520 |
| walker |  | 7882 | 53 | python decl at beets/util/config.py:29 |  |  | 0.520 |
| walker |  | 7915 | 33 | python decl names surface in beets/importer/state.py |  |  | 0.520 |
| walker |  | 7921 | 6 | python decl at beets/importer/state.py:34 |  |  | 0.520 |
| walker |  | 7936 | 15 | python decl doc at beets/importer/state.py:34 |  |  | 0.520 |
| ns | 7963 |  | 99 | `Recommendation` + `Proposal` + `Match`/`AlbumMatch`/`TrackMatch` + `Distance` + `Info`/`AlbumInfo`/`TrackInfo` | 6.2 |  | 0.515 |
| walker |  | 8002 | 66 | python decl names surface #2 in beets/ui/__init__.py |  |  | 0.517 |
| walker |  | 8002 | 0 | python decl at beets/ui/__init__.py:905 |  |  | 0.517 |
| walker |  | 8002 | 0 | python decl at beets/ui/__init__.py:996 |  |  | 0.517 |
| walker |  | 8031 | 29 | python decl at beets/ui/__init__.py:65 |  |  | 0.517 |
| walker |  | 8068 | 37 | python decl doc at beets/ui/__init__.py:996 |  |  | 0.518 |
| walker |  | 8097 | 29 | python decl doc at beets/ui/__init__.py:905 |  |  | 0.518 |
| walker |  | 8152 | 55 | python decl doc at beets/ui/__init__.py:90 |  |  | 0.518 |
| ns | 8174 |  | 211 | `Action` enum + `ImportTask` class header | 7.1 |  | 0.511 |
| walker |  | 8191 | 39 | listing of '.github' |  |  | 0.511 |
| walker |  | 8222 | 31 | listing of '.github/workflows' |  |  | 0.511 |
| walker |  | 8233 | 11 | python imports #15 in beets/util/__init__.py |  |  | 0.511 |
| walker |  | 8398 | 165 | python decl doc at beets/util/__init__.py:74 |  |  | 0.511 |
| walker |  | 8408 | 10 | python imports #16 in beets/util/__init__.py |  |  | 0.511 |
| walker |  | 8417 | 9 | python imports #17 in beets/util/__init__.py |  |  | 0.511 |
| walker |  | 8455 | 38 | python decl names surface in beets/library/exceptions.py |  |  | 0.511 |
| walker |  | 8455 | 0 | python decl at beets/library/exceptions.py:4 |  |  | 0.511 |
| walker |  | 8455 | 0 | python decl at beets/library/exceptions.py:27 |  |  | 0.511 |
| walker |  | 8455 | 0 | python decl at beets/library/exceptions.py:34 |  |  | 0.511 |
| walker |  | 8475 | 20 | python decl doc at beets/library/exceptions.py:27 |  |  | 0.511 |
| ns | 8488 |  | 314 | `ImportSession.run()` — pipeline assembly | 7.2 |  | 0.502 |
| walker |  | 8495 | 20 | python decl doc at beets/library/exceptions.py:34 |  |  | 0.502 |
| walker |  | 8551 | 56 | python method sigs in beets/library/exceptions.py |  |  | 0.502 |
| walker |  | 8551 | 0 | python method at beets/library/exceptions.py:11 |  |  | 0.502 |
| walker |  | 8551 | 0 | python method at beets/library/exceptions.py:19 |  |  | 0.502 |
| walker |  | 8551 | 0 | python method at beets/library/exceptions.py:30 |  |  | 0.502 |
| walker |  | 8551 | 0 | python method at beets/library/exceptions.py:37 |  |  | 0.502 |
| walker |  | 8563 | 12 | python method body at beets/library/exceptions.py:30 body 31 |  |  | 0.502 |
| walker |  | 8575 | 12 | python method body at beets/library/exceptions.py:37 body 38 |  |  | 0.502 |
| walker |  | 8653 | 78 | python decl body at beets/ui/commands/__init__.py:36 body 38 |  |  | 0.515 |
| ns | 8655 |  | 167 | Importer stage functions (locations) | 7.3 |  | 0.511 |
| walker |  | 8695 | 42 | python method body at beets/util/__init__.py:90 body 91 |  |  | 0.511 |
| walker |  | 8824 | 129 | python decl names surface in beets/util/deprecation.py |  |  | 0.511 |
| walker |  | 8824 | 0 | python decl at beets/util/deprecation.py:30 |  |  | 0.511 |
| walker |  | 8840 | 16 | python decl at beets/util/deprecation.py:16 |  |  | 0.511 |
| walker |  | 8866 | 26 | python decl at beets/util/deprecation.py:78 |  |  | 0.511 |
| walker |  | 8893 | 27 | python decl at beets/util/deprecation.py:39 |  |  | 0.511 |
| ns | 8907 |  | 252 | `BeetsPlugin` class header + registration-method locations | 8.1 |  | 0.506 |
| walker |  | 8923 | 30 | python decl at beets/util/deprecation.py:59 |  |  | 0.506 |
| walker |  | 8954 | 31 | python decl at beets/util/deprecation.py:45 |  |  | 0.506 |
| walker |  | 8968 | 14 | python decl body at beets/util/deprecation.py:39 body 42 |  |  | 0.506 |
| walker |  | 9039 | 71 | python decl at beets/util/deprecation.py:19 |  |  | 0.506 |
| walker |  | 9079 | 40 | python decl body at beets/util/deprecation.py:45 body 54 |  |  | 0.506 |
| walker |  | 9088 | 9 | python imports #18 in beets/util/__init__.py |  |  | 0.506 |
| walker |  | 9156 | 68 | python class body at beets/importer/state.py:34 |  |  | 0.506 |
| ns | 9216 |  | 309 | `EventType` literal — every event a listener can hook | 8.2 |  | 0.495 |
| walker |  | 9244 | 88 | README.rst section #0 |  |  | 0.495 |
| walker |  | 9326 | 82 | python decl doc at beets/util/deprecation.py:45 |  |  | 0.495 |
| walker |  | 9345 | 19 | python decl body at beets/ui/__init__.py:111 body 118 |  |  | 0.495 |
| walker |  | 9403 | 58 | python decl doc at beets/library/exceptions.py:4 |  |  | 0.495 |
| ns | 9425 |  | 209 | `load_plugins`, `find_plugins`, dispatch entry points (signatures) | 8.3 |  | 0.491 |
| walker |  | 9494 | 91 | python decl body at beets/util/__init__.py:280 body 284 |  |  | 0.491 |
| walker |  | 9535 | 41 | listing of 'docs/dev/plugins/other' |  |  | 0.491 |
| ns | 9580 |  | 155 | `MetadataSourcePlugin` + entry-fn locations | 8.4 |  | 0.488 |
| walker |  | 9626 | 91 | python decl doc at beets/util/deprecation.py:59 |  |  | 0.488 |
| walker |  | 9787 | 161 | python decl names surface #1 in beets/ui/__init__.py |  |  | 0.502 |
| walker |  | 9787 | 0 | python decl at beets/ui/__init__.py:395 |  |  | 0.502 |
| walker |  | 9787 | 0 | python decl at beets/ui/__init__.py:433 |  |  | 0.502 |
| walker |  | 9787 | 0 | python decl at beets/ui/__init__.py:445 |  |  | 0.502 |
| walker |  | 9787 | 0 | python decl at beets/ui/__init__.py:498 |  |  | 0.502 |
| walker |  | 9787 | 0 | python decl at beets/ui/__init__.py:637 |  |  | 0.502 |
| walker |  | 9787 | 0 | python decl at beets/ui/__init__.py:676 |  |  | 0.502 |
| walker |  | 9787 | 0 | python decl at beets/ui/__init__.py:830 |  |  | 0.502 |
| walker |  | 9787 | 0 | python decl at beets/ui/__init__.py:867 |  |  | 0.502 |
| walker |  | 9787 | 0 | python decl at beets/ui/__init__.py:879 |  |  | 0.502 |
| walker |  | 9796 | 9 | python decl at beets/ui/__init__.py:459 |  |  | 0.503 |
| walker |  | 9813 | 17 | python class body at beets/ui/__init__.py:676 |  |  | 0.503 |
| walker |  | 9834 | 21 | python class body at beets/ui/__init__.py:637 |  |  | 0.505 |
| walker |  | 9848 | 14 | python decl doc at beets/ui/__init__.py:445 |  |  | 0.505 |
| walker |  | 9864 | 16 | python decl doc at beets/ui/__init__.py:459 |  |  | 0.505 |
| walker |  | 9893 | 29 | python decl doc at beets/ui/__init__.py:676 |  |  | 0.505 |
| walker |  | 9923 | 30 | python decl doc at beets/ui/__init__.py:433 |  |  | 0.505 |
| walker |  | 9959 | 36 | python decl doc at beets/ui/__init__.py:637 |  |  | 0.508 |
| ns | 9963 |  | 383 | `beetsplug/` listing — every shipped plugin | 9.1 |  | 0.483 |
| walker |  | 9972 | 13 | python decl doc at beets/ui/__init__.py:879 |  |  | 0.483 |
| walker |  | 9996 | 24 | python decl at beets/ui/__init__.py:807 |  |  | 0.484 |
