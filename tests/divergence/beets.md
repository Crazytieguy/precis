Score(3000)=0.611 I=0.704 C=0.530 ns_rows≤3K=17/50 (reached=6 partial=2 missing=9)

| source | ns_cum | walker_cum | marginal | descriptor | id | predecessor | Score(B=cum) |
|:-------|-------:|-----------:|---------:|:-----------|:---|:------------|-------------:|
| walker |  | 83 | 83 | listing of '.' |  |  | 0.000 |
| ns | 98 |  | 98 | README lede + tagline | 1.1 |  | 0.000 |
| walker |  | 102 | 19 | listing of 'extra' |  |  | 0.000 |
| walker |  | 169 | 67 | listing of 'beets' |  |  | 0.000 |
| ns | 181 |  | 83 | Top-level repo listing | 1.2 |  | 0.504 |
| ns | 211 |  | 30 | Entry-point scripts (`beet = beets.ui:main`) | 1.3 |  | 0.473 |
| ns | 303 |  | 92 | `python -m beets` shim | 1.4 |  | 0.398 |
| ns | 390 |  | 87 | Package version + global config singleton | 1.5 |  | 0.371 |
| walker |  | 429 | 260 | python imports in beets/__init__.py |  |  | 0.379 |
| walker |  | 438 | 9 | listing of 'beets/ui' |  |  | 0.379 |
| walker |  | 456 | 18 | listing of 'beets/autotag' |  |  | 0.380 |
| walker |  | 478 | 22 | listing of 'beets/importer' |  |  | 0.383 |
| walker |  | 493 | 15 | python decl names surface in beets/autotag/__init__.py |  |  | 0.383 |
| walker |  | 493 | 0 | python decl at beets/autotag/__init__.py:28 |  |  | 0.383 |
| walker |  | 521 | 28 | listing of 'beets/dbcore' |  |  | 0.392 |
| walker |  | 551 | 30 | listing of 'beets/library' |  |  | 0.406 |
| ns | 571 |  | 181 | Project metadata + Python version constraint | 1.6 |  | 0.346 |
| walker |  | 579 | 28 | python decl names surface in beets/library/__init__.py |  |  | 0.346 |
| walker |  | 579 | 0 | python decl at beets/library/__init__.py:15 |  |  | 0.346 |
| walker |  | 646 | 67 | listing of 'beets/ui/commands' |  |  | 0.353 |
| walker |  | 660 | 14 | listing of 'beets/ui/commands/import_' |  |  | 0.356 |
| walker |  | 683 | 23 | python decl names surface in beets/ui/commands/__init__.py |  |  | 0.356 |
| walker |  | 683 | 0 | python decl at beets/ui/commands/__init__.py:36 |  |  | 0.356 |
| walker |  | 691 | 8 | python decl doc at beets/ui/commands/__init__.py:36 |  |  | 0.356 |
| walker |  | 763 | 72 | python imports in beets/ui/commands/import_/__init__.py |  |  | 0.356 |
| ns | 802 |  | 231 | Core (non-optional) runtime deps | 1.7 |  | 0.309 |
| walker |  | 838 | 75 | listing of 'beets/util' |  |  | 0.325 |
| walker |  | 848 | 10 | python imports in beets/util/__init__.py |  |  | 0.325 |
| walker |  | 859 | 11 | python imports #1 in beets/util/__init__.py |  |  | 0.325 |
| walker |  | 867 | 8 | python imports #2 in beets/util/__init__.py |  |  | 0.325 |
| ns | 869 |  | 67 | `beets/` core package listing | 2.1 |  | 0.386 |
| walker |  | 874 | 7 | python imports #3 in beets/util/__init__.py |  |  | 0.386 |
| walker |  | 882 | 8 | python imports #4 in beets/util/__init__.py |  |  | 0.386 |
| walker |  | 889 | 7 | python imports #5 in beets/util/__init__.py |  |  | 0.386 |
| walker |  | 896 | 7 | python imports #6 in beets/util/__init__.py |  |  | 0.386 |
| walker |  | 903 | 7 | python imports #7 in beets/util/__init__.py |  |  | 0.386 |
| walker |  | 950 | 47 | python decl names surface in beets/__init__.py |  |  | 0.402 |
| walker |  | 950 | 0 | python decl at beets/__init__.py:26 |  |  | 0.402 |
| walker |  | 950 | 0 | python decl at beets/__init__.py:35 |  |  | 0.402 |
| walker |  | 958 | 8 | python decl doc at beets/__init__.py:26 |  |  | 0.402 |
| ns | 967 |  | 98 | Core subpackage listings: dbcore, library, autotag, importer | 2.2 |  | 0.443 |
| walker |  | 984 | 26 | python method sigs in beets/__init__.py |  |  | 0.443 |
| walker |  | 984 | 0 | python method at beets/__init__.py:40 |  |  | 0.443 |
| walker |  | 1003 | 19 | python decl body at beets/library/__init__.py:15 body 16 |  |  | 0.443 |
| walker |  | 1011 | 8 | python imports #8 in beets/util/__init__.py |  |  | 0.443 |
| walker |  | 1018 | 7 | python imports #9 in beets/util/__init__.py |  |  | 0.443 |
| walker |  | 1058 | 40 | python decl doc at beets/__init__.py:35 |  |  | 0.457 |
| ns | 1132 |  | 165 | UI + util + commands listings | 2.3 |  | 0.488 |
| walker |  | 1229 | 171 | python imports in beets/library/__init__.py |  |  | 0.496 |
| walker |  | 1236 | 7 | python imports #10 in beets/util/__init__.py |  |  | 0.496 |
| walker |  | 1243 | 7 | python imports #11 in beets/util/__init__.py |  |  | 0.496 |
| walker |  | 1255 | 12 | python decl names surface in beets/library/library.py |  |  | 0.496 |
| walker |  | 1255 | 0 | python decl at beets/library/library.py:20 |  |  | 0.496 |
| walker |  | 1268 | 13 | python decl doc at beets/library/library.py:20 |  |  | 0.496 |
| walker |  | 1335 | 67 | [package] in pyproject.toml |  |  | 0.506 |
| ns | 1405 |  | 273 | `beets.dbcore` public re-exports | 2.4 |  | 0.452 |
| walker |  | 1441 | 106 | python decl names surface in beets/ui/commands/import_/__init__.py |  |  | 0.452 |
| walker |  | 1441 | 0 | python decl at beets/ui/commands/import_/__init__.py:14 |  |  | 0.452 |
| walker |  | 1441 | 0 | python decl at beets/ui/commands/import_/__init__.py:34 |  |  | 0.452 |
| walker |  | 1441 | 0 | python decl at beets/ui/commands/import_/__init__.py:49 |  |  | 0.452 |
| walker |  | 1441 | 0 | python decl at beets/ui/commands/import_/__init__.py:81 |  |  | 0.452 |
| walker |  | 1441 | 0 | python decl at beets/ui/commands/import_/__init__.py:134 |  |  | 0.452 |
| walker |  | 1457 | 16 | python decl doc at beets/ui/commands/import_/__init__.py:34 |  |  | 0.452 |
| walker |  | 1483 | 26 | python decl at beets/ui/commands/import_/__init__.py:160 |  |  | 0.452 |
| walker |  | 1513 | 30 | python decl doc at beets/ui/commands/import_/__init__.py:49 |  |  | 0.452 |
| walker |  | 1544 | 31 | python decl doc at beets/ui/commands/import_/__init__.py:14 |  |  | 0.452 |
| walker |  | 1551 | 7 | python imports #12 in beets/util/__init__.py |  |  | 0.452 |
| walker |  | 1628 | 77 | listing of 'docs' |  |  | 0.455 |
| walker |  | 1650 | 22 | listing of 'docs/api' |  |  | 0.456 |
| walker |  | 1676 | 26 | listing of 'docs/guides' |  |  | 0.458 |
| walker |  | 1702 | 26 | listing of 'docs/reference' |  |  | 0.460 |
| ns | 1722 |  | 317 | `beets.library` public re-exports | 2.5 |  | 0.451 |
| walker |  | 1730 | 28 | listing of 'docs/dev' |  |  | 0.455 |
| walker |  | 1737 | 7 | python imports in beets/util/units.py |  |  | 0.455 |
| walker |  | 1753 | 16 | python decl names surface in beets/library/fields.py |  |  | 0.455 |
| walker |  | 1760 | 7 | python imports #13 in beets/util/__init__.py |  |  | 0.455 |
| walker |  | 1764 | 4 | listing of 'docs/_templates' |  |  | 0.455 |
| walker |  | 1785 | 21 | listing of 'docs/_templates/autosummary' |  |  | 0.455 |
| walker |  | 1789 | 4 | listing of 'docs/extensions' |  |  | 0.455 |
| walker |  | 1858 | 69 | python decl at beets/library/__init__.py:8 |  |  | 0.475 |
| walker |  | 1873 | 15 | listing of 'beets/test' |  |  | 0.475 |
| walker |  | 1891 | 18 | python decl names surface in beets/util/hidden.py |  |  | 0.475 |
| walker |  | 1891 | 0 | python decl at beets/util/hidden.py:25 |  |  | 0.475 |
| walker |  | 1915 | 24 | listing of 'docs/dev/plugins' |  |  | 0.479 |
| ns | 2088 |  | 366 | `beets.autotag` public re-exports | 2.6 |  | 0.438 |
| ns | 2306 |  | 218 | `beets.importer` public re-exports | 2.7 |  | 0.415 |
| walker |  | 2465 | 550 | README headline in README.rst |  |  | 0.592 |
| ns | 2509 |  | 203 | Docs tree + reference + dev-doc table-of-contents | 2.8 |  | 0.626 |
| ns | 2681 |  | 172 | `UserError` + `Subcommand` class header | 3.1 |  | 0.610 |
| walker |  | 2840 | 375 | python imports in beets/autotag/__init__.py |  |  | 0.631 |
| ns | 2928 |  | 247 | Top-level fns in `beets/ui/__init__.py` (signature heads only) | 3.2 |  | 0.611 |
| walker |  | 3218 | 378 | python imports in beets/importer/__init__.py |  |  | 0.645 |
| walker |  | 3231 | 13 | python decl names surface in beets/ui/commands/help.py |  |  | 0.645 |
| walker |  | 3231 | 0 | python decl at beets/ui/commands/help.py:6 |  |  | 0.645 |
| ns | 3376 |  | 448 | `main()` top-level exception handlers | 3.3 | 3.2 | 0.612 |
| ns | 3470 |  | 94 | `_raw_main()` global-option setup | 3.4 | 3.2 | 0.606 |
| walker |  | 3625 | 394 | python imports in beets/ui/commands/__init__.py |  |  | 0.607 |
| walker |  | 3635 | 10 | python imports in beets/library/exceptions.py |  |  | 0.607 |
| walker |  | 3658 | 23 | python decl names surface in beets/importer/state.py |  |  | 0.607 |
| walker |  | 3664 | 6 | python decl at beets/importer/state.py:34 |  |  | 0.607 |
| walker |  | 3688 | 24 | python decl names surface in beets/util/m3u.py |  |  | 0.607 |
| walker |  | 3688 | 0 | python decl at beets/util/m3u.py:22 |  |  | 0.607 |
| walker |  | 3688 | 0 | python decl at beets/util/m3u.py:28 |  |  | 0.607 |
| walker |  | 3695 | 7 | python class body at beets/util/m3u.py:22 |  |  | 0.607 |
| walker |  | 3712 | 17 | python decl doc at beets/util/m3u.py:22 |  |  | 0.607 |
| ns | 3728 |  | 258 | `_raw_main()` parse + dispatch tail | 3.5 | 3.4 | 0.588 |
| walker |  | 3730 | 18 | python decl doc at beets/util/m3u.py:28 |  |  | 0.588 |
| walker |  | 3739 | 9 | python imports #14 in beets/util/__init__.py |  |  | 0.588 |
| walker |  | 3767 | 28 | python method sigs in beets/ui/commands/help.py |  |  | 0.588 |
| walker |  | 3767 | 0 | python method at beets/ui/commands/help.py:7 |  |  | 0.588 |
| walker |  | 3767 | 0 | python method at beets/ui/commands/help.py:14 |  |  | 0.588 |
| ns | 3939 |  | 211 | `_setup()` body: load plugins, build commands, open library | 3.6 | 3.2 | 0.572 |
| walker |  | 4243 | 476 | python imports in beets/ui/__init__.py |  |  | 0.572 |
| ns | 4347 |  | 408 | `default_commands` list (built-in subcommands) | 3.7 |  | 0.552 |
| walker |  | 4395 | 152 | python decl names surface in beets/util/__init__.py |  |  | 0.552 |
| walker |  | 4395 | 0 | python decl at beets/util/__init__.py:74 |  |  | 0.552 |
| walker |  | 4395 | 0 | python decl at beets/util/__init__.py:130 |  |  | 0.552 |
| walker |  | 4395 | 0 | python decl at beets/util/__init__.py:158 |  |  | 0.552 |
| walker |  | 4395 | 0 | python decl at beets/util/__init__.py:169 |  |  | 0.552 |
| walker |  | 4395 | 0 | python decl at beets/util/__init__.py:175 |  |  | 0.552 |
| walker |  | 4415 | 20 | python class body at beets/util/__init__.py:74 |  |  | 0.552 |
| walker |  | 4433 | 18 | python decl doc at beets/util/__init__.py:158 |  |  | 0.552 |
| walker |  | 4458 | 25 | python class body at beets/util/__init__.py:169 |  |  | 0.552 |
| walker |  | 4488 | 30 | python decl doc at beets/util/__init__.py:175 |  |  | 0.552 |
| ns | 4535 |  | 188 | Built-in command registrations (one `Subcommand` line per cmd) | 3.8 |  | 0.545 |
| walker |  | 4554 | 66 | python class body at beets/util/__init__.py:158 |  |  | 0.545 |
| walker |  | 4608 | 54 | python decl doc at beets/util/__init__.py:130 |  |  | 0.545 |
| walker |  | 4713 | 105 | python method sigs in beets/util/__init__.py |  |  | 0.545 |
| walker |  | 4713 | 0 | python method at beets/util/__init__.py:90 |  |  | 0.545 |
| walker |  | 4713 | 0 | python method at beets/util/__init__.py:96 |  |  | 0.545 |
| walker |  | 4713 | 0 | python method at beets/util/__init__.py:104 |  |  | 0.545 |
| walker |  | 4713 | 0 | python method at beets/util/__init__.py:115 |  |  | 0.545 |
| walker |  | 4713 | 0 | python method at beets/util/__init__.py:121 |  |  | 0.545 |
| walker |  | 4713 | 0 | python method at beets/util/__init__.py:136 |  |  | 0.545 |
| walker |  | 4728 | 15 | python method at beets/util/__init__.py:140 |  |  | 0.545 |
| walker |  | 4738 | 10 | python method body at beets/util/__init__.py:115 body 119 |  |  | 0.545 |
| walker |  | 4749 | 11 | python method doc at beets/util/__init__.py:104 |  |  | 0.545 |
| walker |  | 4777 | 28 | python method doc at beets/util/__init__.py:115 |  |  | 0.545 |
| walker |  | 4794 | 17 | python method doc at beets/util/__init__.py:96 |  |  | 0.545 |
| ns | 4796 |  | 261 | Smallest command as concrete template (`list`) | 3.9 | 3.8 | 0.529 |
| walker |  | 4833 | 39 | python method doc at beets/util/__init__.py:121 |  |  | 0.529 |
| walker |  | 4856 | 23 | python method body at beets/util/__init__.py:136 body 137 |  |  | 0.529 |
| walker |  | 4868 | 12 | python imports in beets/library/fields.py |  |  | 0.529 |
| ns | 4982 |  | 186 | `Library` class header + `_models` + `_migrations` | 4.1 |  | 0.522 |
| ns | 5125 |  | 143 | `Library` query/fetch method headers | 4.2 | 4.1 | 0.516 |
| ns | 5328 |  | 203 | `LibModel` + `Item` + `Album` class headers | 4.3 |  | 0.505 |
| walker |  | 5359 | 491 | python imports in beets/dbcore/__init__.py |  |  | 0.552 |
| walker |  | 5381 | 22 | python imports in beets/context.py |  |  | 0.552 |
| ns | 5412 |  | 84 | `_search_fields` for Item + Album (default query targets) | 4.4 | 4.3 | 0.547 |
| walker |  | 5447 | 66 | python decl names surface in beets/context.py |  |  | 0.547 |
| walker |  | 5447 | 0 | python decl at beets/context.py:8 |  |  | 0.547 |
| walker |  | 5447 | 0 | python decl at beets/context.py:13 |  |  | 0.547 |
| walker |  | 5456 | 9 | python decl at beets/context.py:18 |  |  | 0.547 |
| walker |  | 5467 | 11 | python decl doc at beets/context.py:8 |  |  | 0.547 |
| walker |  | 5478 | 11 | python decl doc at beets/context.py:13 |  |  | 0.547 |
| walker |  | 5490 | 12 | python decl body at beets/context.py:8 body 10 |  |  | 0.547 |
| walker |  | 5502 | 12 | python decl body at beets/context.py:13 body 15 |  |  | 0.547 |
| walker |  | 5520 | 18 | python decl doc at beets/context.py:18 |  |  | 0.547 |
| ns | 5614 |  | 202 | `TYPE_BY_FIELD` lede + first dozen fields | 4.5 |  | 0.539 |
| walker |  | 5684 | 164 | python decl names surface in beets/ui/__init__.py |  |  | 0.541 |
| walker |  | 5684 | 0 | python decl at beets/ui/__init__.py:71 |  |  | 0.541 |
| walker |  | 5684 | 0 | python decl at beets/ui/__init__.py:80 |  |  | 0.541 |
| walker |  | 5684 | 0 | python decl at beets/ui/__init__.py:85 |  |  | 0.541 |
| walker |  | 5684 | 0 | python decl at beets/ui/__init__.py:90 |  |  | 0.541 |
| walker |  | 5684 | 0 | python decl at beets/ui/__init__.py:111 |  |  | 0.541 |
| walker |  | 5684 | 0 | python decl at beets/ui/__init__.py:122 |  |  | 0.541 |
| walker |  | 5684 | 0 | python decl at beets/ui/__init__.py:150 |  |  | 0.541 |
| walker |  | 5684 | 0 | python decl at beets/ui/__init__.py:160 |  |  | 0.541 |
| walker |  | 5684 | 0 | python decl at beets/ui/__init__.py:167 |  |  | 0.541 |
| walker |  | 5684 | 0 | python decl at beets/ui/__init__.py:187 |  |  | 0.541 |
| walker |  | 5713 | 29 | python decl at beets/ui/__init__.py:65 |  |  | 0.541 |
| walker |  | 5748 | 35 | python decl doc at beets/ui/__init__.py:71 |  |  | 0.542 |
| walker |  | 5770 | 22 | python decl body at beets/ui/__init__.py:160 body 164 |  |  | 0.542 |
| walker |  | 5807 | 37 | python decl doc at beets/ui/__init__.py:160 |  |  | 0.542 |
| walker |  | 5819 | 12 | python decl body at beets/ui/__init__.py:80 body 82 |  |  | 0.542 |
| walker |  | 5831 | 12 | python decl body at beets/ui/__init__.py:85 body 87 |  |  | 0.542 |
| walker |  | 5849 | 18 | python decl doc at beets/ui/__init__.py:150 |  |  | 0.542 |
| walker |  | 5868 | 19 | python decl doc at beets/ui/__init__.py:80 |  |  | 0.542 |
| walker |  | 5887 | 19 | python decl doc at beets/ui/__init__.py:85 |  |  | 0.542 |
| walker |  | 5896 | 9 | python decl body at beets/ui/__init__.py:111 body 119 |  |  | 0.542 |
| walker |  | 5967 | 71 | python decl doc at beets/ui/__init__.py:187 |  |  | 0.542 |
| ns | 5986 |  | 372 | `parse_query_parts` + `parse_query_string` (library-level query entry) | 4.6 |  | 0.526 |
| walker |  | 6041 | 74 | python decl doc at beets/ui/__init__.py:111 |  |  | 0.526 |
| walker |  | 6123 | 82 | python decl doc at beets/ui/__init__.py:122 |  |  | 0.526 |
| walker |  | 6177 | 54 | python decl body at beets/ui/__init__.py:167 body 177 |  |  | 0.526 |
| walker |  | 6236 | 59 | python decl body at beets/__init__.py:26 body 28 |  |  | 0.526 |
| ns | 6325 |  | 339 | `LibModel` + `Item` + `Album` method names (locations) | 4.7 | 4.3 | 0.515 |
| walker |  | 6345 | 109 | python decl at beets/ui/commands/__init__.py:50 |  |  | 0.530 |
| walker |  | 6458 | 113 | python decl doc at beets/ui/__init__.py:167 |  |  | 0.530 |
| walker |  | 6477 | 19 | python decl names surface in beets/ui/commands/utils.py |  |  | 0.530 |
| walker |  | 6477 | 0 | python decl at beets/ui/commands/utils.py:6 |  |  | 0.530 |
| walker |  | 6528 | 51 | python decl doc at beets/ui/commands/import_/__init__.py:134 |  |  | 0.530 |
| ns | 6560 |  | 235 | `DefaultTemplateFunctions` header + `tmpl_*` method locations | 4.8 |  | 0.523 |
| walker |  | 6586 | 58 | python method sigs in beets/util/m3u.py |  |  | 0.523 |
| walker |  | 6586 | 0 | python method at beets/util/m3u.py:31 |  |  | 0.523 |
| walker |  | 6586 | 0 | python method at beets/util/m3u.py:43 |  |  | 0.523 |
| walker |  | 6586 | 0 | python method at beets/util/m3u.py:63 |  |  | 0.523 |
| walker |  | 6586 | 0 | python method at beets/util/m3u.py:77 |  |  | 0.523 |
| walker |  | 6604 | 18 | python method doc at beets/util/m3u.py:43 |  |  | 0.523 |
| walker |  | 6636 | 32 | python decl doc at beets/util/hidden.py:25 |  |  | 0.523 |
| walker |  | 6668 | 32 | python decl names surface in beets/util/config.py |  |  | 0.523 |
| walker |  | 6668 | 0 | python decl at beets/util/config.py:78 |  |  | 0.523 |
| ns | 6673 |  | 113 | Library migration class headers (full chain) | 4.9 |  | 0.519 |
| walker |  | 6683 | 15 | python method sigs in beets/util/config.py |  |  | 0.519 |
| walker |  | 6683 | 0 | python method at beets/util/config.py:79 |  |  | 0.519 |
| walker |  | 6706 | 23 | python decl at beets/util/config.py:9 |  |  | 0.519 |
| walker |  | 6759 | 53 | python decl at beets/util/config.py:29 |  |  | 0.519 |
| walker |  | 6814 | 55 | python decl doc at beets/ui/__init__.py:90 |  |  | 0.519 |
| ns | 6903 |  | 230 | `Model` + `Database` class headers | 5.1 |  | 0.512 |
| walker |  | 6969 | 155 | python decl doc at beets/util/__init__.py:74 |  |  | 0.512 |
| ns | 6999 |  | 96 | Top-level classes in `beets/dbcore/db.py` (locations) | 5.2 |  | 0.508 |
| walker |  | 7008 | 39 | listing of '.github' |  |  | 0.508 |
| walker |  | 7039 | 31 | listing of '.github/workflows' |  |  | 0.508 |
| walker |  | 7050 | 11 | python imports #15 in beets/util/__init__.py |  |  | 0.508 |
| walker |  | 7060 | 10 | python imports #16 in beets/util/__init__.py |  |  | 0.508 |
| walker |  | 7069 | 9 | python imports #17 in beets/util/__init__.py |  |  | 0.508 |
| walker |  | 7107 | 38 | python decl names surface in beets/library/exceptions.py |  |  | 0.508 |
| walker |  | 7107 | 0 | python decl at beets/library/exceptions.py:4 |  |  | 0.508 |
| walker |  | 7107 | 0 | python decl at beets/library/exceptions.py:27 |  |  | 0.508 |
| walker |  | 7107 | 0 | python decl at beets/library/exceptions.py:34 |  |  | 0.508 |
| walker |  | 7127 | 20 | python decl doc at beets/library/exceptions.py:27 |  |  | 0.508 |
| walker |  | 7147 | 20 | python decl doc at beets/library/exceptions.py:34 |  |  | 0.508 |
| walker |  | 7203 | 56 | python method sigs in beets/library/exceptions.py |  |  | 0.508 |
| walker |  | 7203 | 0 | python method at beets/library/exceptions.py:11 |  |  | 0.508 |
| walker |  | 7203 | 0 | python method at beets/library/exceptions.py:19 |  |  | 0.508 |
| walker |  | 7203 | 0 | python method at beets/library/exceptions.py:30 |  |  | 0.508 |
| walker |  | 7203 | 0 | python method at beets/library/exceptions.py:37 |  |  | 0.508 |
| walker |  | 7215 | 12 | python method body at beets/library/exceptions.py:30 body 31 |  |  | 0.508 |
| walker |  | 7227 | 12 | python method body at beets/library/exceptions.py:37 body 38 |  |  | 0.508 |
| ns | 7271 |  | 272 | `Query` hierarchy class headers (full chain) | 5.3 |  | 0.499 |
| walker |  | 7304 | 77 | python decl names surface #2 in beets/ui/__init__.py |  |  | 0.502 |
| walker |  | 7304 | 0 | python decl at beets/ui/__init__.py:867 |  |  | 0.502 |
| walker |  | 7304 | 0 | python decl at beets/ui/__init__.py:879 |  |  | 0.502 |
| walker |  | 7304 | 0 | python decl at beets/ui/__init__.py:905 |  |  | 0.502 |
| walker |  | 7304 | 0 | python decl at beets/ui/__init__.py:996 |  |  | 0.502 |
| walker |  | 7317 | 13 | python decl doc at beets/ui/__init__.py:879 |  |  | 0.502 |
| ns | 7346 |  | 75 | `Sort` class hierarchy (locations) | 5.4 |  | 0.499 |
| walker |  | 7354 | 37 | python decl doc at beets/ui/__init__.py:996 |  |  | 0.499 |
| walker |  | 7383 | 29 | python decl doc at beets/ui/__init__.py:905 |  |  | 0.499 |
| walker |  | 7461 | 78 | python decl body at beets/ui/commands/__init__.py:36 body 38 |  |  | 0.513 |
| walker |  | 7503 | 42 | python method body at beets/util/__init__.py:90 body 91 |  |  | 0.513 |
| walker |  | 7512 | 9 | python imports #18 in beets/util/__init__.py |  |  | 0.513 |
| ns | 7547 |  | 201 | `Type` hierarchy class headers | 5.5 |  | 0.506 |
| walker |  | 7580 | 68 | python class body at beets/importer/state.py:34 |  |  | 0.506 |
| ns | 7601 |  | 54 | `queryparse` top-level fn locations | 5.6 |  | 0.504 |
| walker |  | 7633 | 53 | python decl doc at beets/library/exceptions.py:4 |  |  | 0.504 |
| walker |  | 7651 | 18 | python decl names surface #1 in beets/util/color.py |  |  | 0.504 |
| walker |  | 7651 | 0 | python decl at beets/util/color.py:208 |  |  | 0.504 |
| walker |  | 7696 | 45 | python decl names surface in beets/importer/session.py |  |  | 0.504 |
| walker |  | 7696 | 0 | python decl at beets/importer/session.py:42 |  |  | 0.504 |
| walker |  | 7696 | 0 | python decl at beets/importer/session.py:48 |  |  | 0.504 |
| walker |  | 7703 | 7 | python class body at beets/importer/session.py:42 |  |  | 0.504 |
| walker |  | 7717 | 14 | python decl doc at beets/importer/session.py:42 |  |  | 0.504 |
| ns | 7733 |  | 132 | `PARSE_QUERY_PART_REGEX` — the query-string syntax | 5.7 | 5.6 | 0.500 |
| walker |  | 7753 | 36 | python decl doc at beets/importer/session.py:48 |  |  | 0.500 |
| walker |  | 7772 | 19 | python decl body at beets/ui/__init__.py:111 body 118 |  |  | 0.500 |
| walker |  | 7794 | 22 | python imports in beets/util/config.py |  |  | 0.500 |
| walker |  | 7840 | 46 | python decl names surface in beets/util/id_extractors.py |  |  | 0.500 |
| walker |  | 7840 | 0 | python decl at beets/util/id_extractors.py:50 |  |  | 0.500 |
| ns | 7864 |  | 131 | Autotag entry points: `tag_album` + `tag_item` signatures | 6.1 |  | 0.496 |
| ns | 7963 |  | 99 | `Recommendation` + `Proposal` + `Match`/`AlbumMatch`/`TrackMatch` + `Distance` + `Info`/`AlbumInfo`/`TrackInfo` | 6.2 |  | 0.492 |
| walker |  | 7991 | 151 | python decl names surface #1 in beets/ui/__init__.py |  |  | 0.506 |
| walker |  | 7991 | 0 | python decl at beets/ui/__init__.py:383 |  |  | 0.506 |
| walker |  | 7991 | 0 | python decl at beets/ui/__init__.py:395 |  |  | 0.506 |
| walker |  | 7991 | 0 | python decl at beets/ui/__init__.py:433 |  |  | 0.506 |
| walker |  | 7991 | 0 | python decl at beets/ui/__init__.py:445 |  |  | 0.506 |
| walker |  | 7991 | 0 | python decl at beets/ui/__init__.py:498 |  |  | 0.506 |
| walker |  | 7991 | 0 | python decl at beets/ui/__init__.py:637 |  |  | 0.506 |
| walker |  | 7991 | 0 | python decl at beets/ui/__init__.py:676 |  |  | 0.506 |
| walker |  | 7991 | 0 | python decl at beets/ui/__init__.py:830 |  |  | 0.506 |
| walker |  | 8000 | 9 | python decl at beets/ui/__init__.py:459 |  |  | 0.508 |
| walker |  | 8017 | 17 | python class body at beets/ui/__init__.py:676 |  |  | 0.508 |
| walker |  | 8038 | 21 | python class body at beets/ui/__init__.py:637 |  |  | 0.509 |
| walker |  | 8052 | 14 | python decl doc at beets/ui/__init__.py:445 |  |  | 0.509 |
| walker |  | 8068 | 16 | python decl doc at beets/ui/__init__.py:459 |  |  | 0.509 |
| walker |  | 8097 | 29 | python decl doc at beets/ui/__init__.py:676 |  |  | 0.509 |
| walker |  | 8127 | 30 | python decl doc at beets/ui/__init__.py:433 |  |  | 0.509 |
| walker |  | 8163 | 36 | python decl doc at beets/ui/__init__.py:637 |  |  | 0.513 |
| ns | 8174 |  | 211 | `Action` enum + `ImportTask` class header | 7.1 |  | 0.506 |
| walker |  | 8187 | 24 | python decl at beets/ui/__init__.py:807 |  |  | 0.507 |
| walker |  | 8202 | 15 | python decl doc at beets/ui/__init__.py:830 |  |  | 0.507 |
| walker |  | 8264 | 62 | python decl at beets/ui/__init__.py:207 |  |  | 0.507 |
| walker |  | 8334 | 70 | python decl at beets/ui/__init__.py:466 |  |  | 0.507 |
| walker |  | 8387 | 53 | python decl doc at beets/ui/__init__.py:383 |  |  | 0.507 |
| walker |  | 8433 | 46 | python decl doc at beets/ui/__init__.py:807 |  |  | 0.509 |
| ns | 8488 |  | 314 | `ImportSession.run()` — pipeline assembly | 7.2 |  | 0.499 |
| walker |  | 8579 | 146 | python decl doc at beets/ui/__init__.py:498 |  |  | 0.499 |
| ns | 8655 |  | 167 | Importer stage functions (locations) | 7.3 |  | 0.495 |
| walker |  | 8709 | 130 | python decl doc at beets/ui/__init__.py:466 |  |  | 0.495 |
| walker |  | 8845 | 136 | python decl doc at beets/ui/__init__.py:395 |  |  | 0.495 |
| ns | 8907 |  | 252 | `BeetsPlugin` class header + registration-method locations | 8.1 |  | 0.490 |
| walker |  | 9130 | 285 | python method sigs #1 in beets/ui/__init__.py |  |  | 0.492 |
| walker |  | 9130 | 0 | python method at beets/ui/__init__.py:514 |  |  | 0.492 |
| walker |  | 9130 | 0 | python method at beets/ui/__init__.py:521 |  |  | 0.492 |
| walker |  | 9130 | 0 | python method at beets/ui/__init__.py:571 |  |  | 0.492 |
| walker |  | 9130 | 0 | python method at beets/ui/__init__.py:591 |  |  | 0.492 |
| walker |  | 9130 | 0 | python method at beets/ui/__init__.py:621 |  |  | 0.492 |
| walker |  | 9130 | 0 | python method at beets/ui/__init__.py:644 |  |  | 0.492 |
| walker |  | 9130 | 0 | python method at beets/ui/__init__.py:658 |  |  | 0.492 |
| walker |  | 9130 | 0 | python method at beets/ui/__init__.py:661 |  |  | 0.492 |
| walker |  | 9130 | 0 | python method at beets/ui/__init__.py:681 |  |  | 0.492 |
| walker |  | 9130 | 0 | python method at beets/ui/__init__.py:701 |  |  | 0.492 |
| walker |  | 9130 | 0 | python method at beets/ui/__init__.py:760 |  |  | 0.492 |
| walker |  | 9130 | 0 | python method at beets/ui/__init__.py:770 |  |  | 0.492 |
| walker |  | 9130 | 0 | python method at beets/ui/__init__.py:783 |  |  | 0.492 |
| walker |  | 9138 | 8 | python method at beets/ui/__init__.py:664 |  |  | 0.492 |
| walker |  | 9149 | 11 | python method at beets/ui/__init__.py:668 |  |  | 0.492 |
| walker |  | 9165 | 16 | python method at beets/ui/__init__.py:708 |  |  | 0.492 |
| walker |  | 9173 | 8 | python method body at beets/ui/__init__.py:658 body 659 |  |  | 0.492 |
| walker |  | 9185 | 12 | python method doc at beets/ui/__init__.py:621 |  |  | 0.492 |
| walker |  | 9195 | 10 | python method body at beets/ui/__init__.py:661 body 662 |  |  | 0.492 |
| walker |  | 9206 | 11 | python method body at beets/ui/__init__.py:664 body 666 |  |  | 0.492 |
| ns | 9216 |  | 309 | `EventType` literal — every event a listener can hook | 8.2 |  | 0.482 |
| walker |  | 9223 | 17 | python method doc at beets/ui/__init__.py:701 |  |  | 0.482 |
| walker |  | 9260 | 37 | python method doc at beets/ui/__init__.py:770 |  |  | 0.482 |
| walker |  | 9312 | 52 | python method doc at beets/ui/__init__.py:783 |  |  | 0.482 |
| walker |  | 9368 | 56 | python method doc at beets/ui/__init__.py:681 |  |  | 0.482 |
| walker |  | 9400 | 32 | python method body at beets/ui/__init__.py:701 body 703 |  |  | 0.482 |
| ns | 9425 |  | 209 | `load_plugins`, `find_plugins`, dispatch entry points (signatures) | 8.3 |  | 0.477 |
| walker |  | 9473 | 73 | python method doc at beets/ui/__init__.py:521 |  |  | 0.477 |
| walker |  | 9568 | 95 | python method doc at beets/ui/__init__.py:571 |  |  | 0.477 |
| ns | 9580 |  | 155 | `MetadataSourcePlugin` + entry-fn locations | 8.4 |  | 0.475 |
| walker |  | 9663 | 95 | python method doc at beets/ui/__init__.py:644 |  |  | 0.477 |
| walker |  | 9673 | 10 | python method body at beets/ui/__init__.py:571 body 589 |  |  | 0.477 |
| walker |  | 9681 | 8 | python decl body at beets/ui/__init__.py:445 body 447 |  |  | 0.477 |
| walker |  | 9689 | 8 | python decl body at beets/ui/__init__.py:445 body 456 |  |  | 0.477 |
| walker |  | 9730 | 41 | listing of 'docs/dev/plugins/other' |  |  | 0.477 |
| walker |  | 9750 | 20 | python decl body at beets/ui/__init__.py:459 body 463 |  |  | 0.477 |
| walker |  | 9800 | 50 | python method body at beets/ui/__init__.py:668 body 670 |  |  | 0.477 |
| walker |  | 9831 | 31 | python decl names surface in beets/ui/commands/version.py |  |  | 0.478 |
| walker |  | 9831 | 0 | python decl at beets/ui/commands/version.py:9 |  |  | 0.478 |
| walker |  | 9882 | 51 | python decl names surface in beets/util/units.py |  |  | 0.478 |
| walker |  | 9882 | 0 | python decl at beets/util/units.py:4 |  |  | 0.478 |
| walker |  | 9882 | 0 | python decl at beets/util/units.py:17 |  |  | 0.478 |
| walker |  | 9882 | 0 | python decl at beets/util/units.py:25 |  |  | 0.478 |
| walker |  | 9882 | 0 | python decl at beets/util/units.py:37 |  |  | 0.478 |
| walker |  | 9900 | 18 | python decl doc at beets/util/units.py:25 |  |  | 0.478 |
| walker |  | 9931 | 31 | python decl doc at beets/util/units.py:17 |  |  | 0.478 |
| ns | 9963 |  | 383 | `beetsplug/` listing — every shipped plugin | 9.1 |  | 0.454 |
| walker |  | 9965 | 34 | python decl doc at beets/util/units.py:37 |  |  | 0.454 |
| walker |  | 9986 | 21 | python decl body at beets/ui/__init__.py:459 body 462 |  |  | 0.454 |
