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
| walker |  | 3679 | 21 | python decl at beets/importer/state.py:34 |  |  | 0.607 |
| walker |  | 3703 | 24 | python decl names surface in beets/util/m3u.py |  |  | 0.607 |
| walker |  | 3703 | 0 | python decl at beets/util/m3u.py:22 |  |  | 0.607 |
| walker |  | 3703 | 0 | python decl at beets/util/m3u.py:28 |  |  | 0.607 |
| walker |  | 3710 | 7 | python class body at beets/util/m3u.py:22 |  |  | 0.607 |
| walker |  | 3727 | 17 | python decl doc at beets/util/m3u.py:22 |  |  | 0.607 |
| ns | 3728 |  | 258 | `_raw_main()` parse + dispatch tail | 3.5 | 3.4 | 0.588 |
| walker |  | 3745 | 18 | python decl doc at beets/util/m3u.py:28 |  |  | 0.588 |
| walker |  | 3754 | 9 | python imports #14 in beets/util/__init__.py |  |  | 0.588 |
| walker |  | 3782 | 28 | python method sigs in beets/ui/commands/help.py |  |  | 0.588 |
| walker |  | 3782 | 0 | python method at beets/ui/commands/help.py:7 |  |  | 0.588 |
| walker |  | 3782 | 0 | python method at beets/ui/commands/help.py:14 |  |  | 0.588 |
| walker |  | 3934 | 152 | python decl names surface in beets/util/__init__.py |  |  | 0.588 |
| walker |  | 3934 | 0 | python decl at beets/util/__init__.py:74 |  |  | 0.588 |
| walker |  | 3934 | 0 | python decl at beets/util/__init__.py:130 |  |  | 0.588 |
| walker |  | 3934 | 0 | python decl at beets/util/__init__.py:158 |  |  | 0.588 |
| walker |  | 3934 | 0 | python decl at beets/util/__init__.py:169 |  |  | 0.588 |
| walker |  | 3934 | 0 | python decl at beets/util/__init__.py:175 |  |  | 0.588 |
| ns | 3939 |  | 211 | `_setup()` body: load plugins, build commands, open library | 3.6 | 3.2 | 0.572 |
| walker |  | 3954 | 20 | python class body at beets/util/__init__.py:74 |  |  | 0.572 |
| walker |  | 3972 | 18 | python decl doc at beets/util/__init__.py:158 |  |  | 0.572 |
| walker |  | 3997 | 25 | python class body at beets/util/__init__.py:169 |  |  | 0.572 |
| walker |  | 4027 | 30 | python decl doc at beets/util/__init__.py:175 |  |  | 0.572 |
| walker |  | 4093 | 66 | python class body at beets/util/__init__.py:158 |  |  | 0.572 |
| walker |  | 4147 | 54 | python decl doc at beets/util/__init__.py:130 |  |  | 0.572 |
| walker |  | 4252 | 105 | python method sigs in beets/util/__init__.py |  |  | 0.572 |
| walker |  | 4252 | 0 | python method at beets/util/__init__.py:90 |  |  | 0.572 |
| walker |  | 4252 | 0 | python method at beets/util/__init__.py:96 |  |  | 0.572 |
| walker |  | 4252 | 0 | python method at beets/util/__init__.py:104 |  |  | 0.572 |
| walker |  | 4252 | 0 | python method at beets/util/__init__.py:115 |  |  | 0.572 |
| walker |  | 4252 | 0 | python method at beets/util/__init__.py:121 |  |  | 0.572 |
| walker |  | 4252 | 0 | python method at beets/util/__init__.py:136 |  |  | 0.572 |
| walker |  | 4267 | 15 | python method at beets/util/__init__.py:140 |  |  | 0.572 |
| walker |  | 4277 | 10 | python method body at beets/util/__init__.py:115 body 119 |  |  | 0.572 |
| walker |  | 4288 | 11 | python method doc at beets/util/__init__.py:104 |  |  | 0.572 |
| walker |  | 4316 | 28 | python method doc at beets/util/__init__.py:115 |  |  | 0.572 |
| walker |  | 4333 | 17 | python method doc at beets/util/__init__.py:96 |  |  | 0.572 |
| ns | 4347 |  | 408 | `default_commands` list (built-in subcommands) | 3.7 |  | 0.552 |
| walker |  | 4372 | 39 | python method doc at beets/util/__init__.py:121 |  |  | 0.552 |
| walker |  | 4395 | 23 | python method body at beets/util/__init__.py:136 body 137 |  |  | 0.552 |
| walker |  | 4407 | 12 | python imports in beets/library/fields.py |  |  | 0.552 |
| ns | 4535 |  | 188 | Built-in command registrations (one `Subcommand` line per cmd) | 3.8 |  | 0.545 |
| ns | 4796 |  | 261 | Smallest command as concrete template (`list`) | 3.9 | 3.8 | 0.529 |
| walker |  | 4898 | 491 | python imports in beets/dbcore/__init__.py |  |  | 0.578 |
| walker |  | 4920 | 22 | python imports in beets/context.py |  |  | 0.578 |
| ns | 4982 |  | 186 | `Library` class header + `_models` + `_migrations` | 4.1 |  | 0.570 |
| ns | 5125 |  | 143 | `Library` query/fetch method headers | 4.2 | 4.1 | 0.564 |
| ns | 5328 |  | 203 | `LibModel` + `Item` + `Album` class headers | 4.3 |  | 0.552 |
| ns | 5412 |  | 84 | `_search_fields` for Item + Album (default query targets) | 4.4 | 4.3 | 0.547 |
| walker |  | 5419 | 499 | python imports in beets/ui/__init__.py |  |  | 0.547 |
| walker |  | 5485 | 66 | python decl names surface in beets/context.py |  |  | 0.547 |
| walker |  | 5485 | 0 | python decl at beets/context.py:8 |  |  | 0.547 |
| walker |  | 5485 | 0 | python decl at beets/context.py:13 |  |  | 0.547 |
| walker |  | 5494 | 9 | python decl at beets/context.py:18 |  |  | 0.547 |
| walker |  | 5505 | 11 | python decl doc at beets/context.py:8 |  |  | 0.547 |
| walker |  | 5516 | 11 | python decl doc at beets/context.py:13 |  |  | 0.547 |
| walker |  | 5528 | 12 | python decl body at beets/context.py:8 body 10 |  |  | 0.547 |
| walker |  | 5540 | 12 | python decl body at beets/context.py:13 body 15 |  |  | 0.547 |
| walker |  | 5558 | 18 | python decl doc at beets/context.py:18 |  |  | 0.547 |
| ns | 5614 |  | 202 | `TYPE_BY_FIELD` lede + first dozen fields | 4.5 |  | 0.539 |
| walker |  | 5722 | 164 | python decl names surface in beets/ui/__init__.py |  |  | 0.541 |
| walker |  | 5722 | 0 | python decl at beets/ui/__init__.py:71 |  |  | 0.541 |
| walker |  | 5722 | 0 | python decl at beets/ui/__init__.py:80 |  |  | 0.541 |
| walker |  | 5722 | 0 | python decl at beets/ui/__init__.py:85 |  |  | 0.541 |
| walker |  | 5722 | 0 | python decl at beets/ui/__init__.py:90 |  |  | 0.541 |
| walker |  | 5722 | 0 | python decl at beets/ui/__init__.py:111 |  |  | 0.541 |
| walker |  | 5722 | 0 | python decl at beets/ui/__init__.py:122 |  |  | 0.541 |
| walker |  | 5722 | 0 | python decl at beets/ui/__init__.py:150 |  |  | 0.541 |
| walker |  | 5722 | 0 | python decl at beets/ui/__init__.py:160 |  |  | 0.541 |
| walker |  | 5722 | 0 | python decl at beets/ui/__init__.py:167 |  |  | 0.541 |
| walker |  | 5722 | 0 | python decl at beets/ui/__init__.py:187 |  |  | 0.541 |
| walker |  | 5751 | 29 | python decl at beets/ui/__init__.py:65 |  |  | 0.541 |
| walker |  | 5786 | 35 | python decl doc at beets/ui/__init__.py:71 |  |  | 0.542 |
| walker |  | 5808 | 22 | python decl body at beets/ui/__init__.py:160 body 164 |  |  | 0.542 |
| walker |  | 5845 | 37 | python decl doc at beets/ui/__init__.py:160 |  |  | 0.542 |
| walker |  | 5857 | 12 | python decl body at beets/ui/__init__.py:80 body 82 |  |  | 0.542 |
| walker |  | 5869 | 12 | python decl body at beets/ui/__init__.py:85 body 87 |  |  | 0.542 |
| walker |  | 5887 | 18 | python decl doc at beets/ui/__init__.py:150 |  |  | 0.542 |
| walker |  | 5906 | 19 | python decl doc at beets/ui/__init__.py:80 |  |  | 0.542 |
| walker |  | 5925 | 19 | python decl doc at beets/ui/__init__.py:85 |  |  | 0.542 |
| walker |  | 5934 | 9 | python decl body at beets/ui/__init__.py:111 body 119 |  |  | 0.542 |
| ns | 5986 |  | 372 | `parse_query_parts` + `parse_query_string` (library-level query entry) | 4.6 |  | 0.526 |
| walker |  | 6005 | 71 | python decl doc at beets/ui/__init__.py:187 |  |  | 0.526 |
| walker |  | 6079 | 74 | python decl doc at beets/ui/__init__.py:111 |  |  | 0.526 |
| walker |  | 6161 | 82 | python decl doc at beets/ui/__init__.py:122 |  |  | 0.526 |
| walker |  | 6215 | 54 | python decl body at beets/ui/__init__.py:167 body 177 |  |  | 0.526 |
| walker |  | 6274 | 59 | python decl body at beets/__init__.py:26 body 28 |  |  | 0.526 |
| ns | 6325 |  | 339 | `LibModel` + `Item` + `Album` method names (locations) | 4.7 | 4.3 | 0.515 |
| walker |  | 6383 | 109 | python decl at beets/ui/commands/__init__.py:50 |  |  | 0.530 |
| walker |  | 6496 | 113 | python decl doc at beets/ui/__init__.py:167 |  |  | 0.530 |
| walker |  | 6515 | 19 | python decl names surface in beets/ui/commands/utils.py |  |  | 0.530 |
| walker |  | 6515 | 0 | python decl at beets/ui/commands/utils.py:6 |  |  | 0.530 |
| ns | 6560 |  | 235 | `DefaultTemplateFunctions` header + `tmpl_*` method locations | 4.8 |  | 0.523 |
| walker |  | 6566 | 51 | python decl doc at beets/ui/commands/import_/__init__.py:134 |  |  | 0.523 |
| walker |  | 6624 | 58 | python method sigs in beets/util/m3u.py |  |  | 0.523 |
| walker |  | 6624 | 0 | python method at beets/util/m3u.py:31 |  |  | 0.523 |
| walker |  | 6624 | 0 | python method at beets/util/m3u.py:43 |  |  | 0.523 |
| walker |  | 6624 | 0 | python method at beets/util/m3u.py:63 |  |  | 0.523 |
| walker |  | 6624 | 0 | python method at beets/util/m3u.py:77 |  |  | 0.523 |
| walker |  | 6642 | 18 | python method doc at beets/util/m3u.py:43 |  |  | 0.523 |
| ns | 6673 |  | 113 | Library migration class headers (full chain) | 4.9 |  | 0.519 |
| walker |  | 6674 | 32 | python decl doc at beets/util/hidden.py:25 |  |  | 0.519 |
| walker |  | 6706 | 32 | python decl names surface in beets/util/config.py |  |  | 0.519 |
| walker |  | 6706 | 0 | python decl at beets/util/config.py:78 |  |  | 0.519 |
| walker |  | 6721 | 15 | python method sigs in beets/util/config.py |  |  | 0.519 |
| walker |  | 6721 | 0 | python method at beets/util/config.py:79 |  |  | 0.519 |
| walker |  | 6744 | 23 | python decl at beets/util/config.py:9 |  |  | 0.519 |
| walker |  | 6799 | 55 | python decl doc at beets/ui/__init__.py:90 |  |  | 0.519 |
| ns | 6903 |  | 230 | `Model` + `Database` class headers | 5.1 |  | 0.512 |
| walker |  | 6954 | 155 | python decl doc at beets/util/__init__.py:74 |  |  | 0.512 |
| walker |  | 6993 | 39 | listing of '.github' |  |  | 0.512 |
| ns | 6999 |  | 96 | Top-level classes in `beets/dbcore/db.py` (locations) | 5.2 |  | 0.508 |
| walker |  | 7024 | 31 | listing of '.github/workflows' |  |  | 0.508 |
| walker |  | 7035 | 11 | python imports #15 in beets/util/__init__.py |  |  | 0.508 |
| walker |  | 7045 | 10 | python imports #16 in beets/util/__init__.py |  |  | 0.508 |
| walker |  | 7054 | 9 | python imports #17 in beets/util/__init__.py |  |  | 0.508 |
| walker |  | 7092 | 38 | python decl names surface in beets/library/exceptions.py |  |  | 0.508 |
| walker |  | 7092 | 0 | python decl at beets/library/exceptions.py:4 |  |  | 0.508 |
| walker |  | 7092 | 0 | python decl at beets/library/exceptions.py:27 |  |  | 0.508 |
| walker |  | 7092 | 0 | python decl at beets/library/exceptions.py:34 |  |  | 0.508 |
| walker |  | 7112 | 20 | python decl doc at beets/library/exceptions.py:27 |  |  | 0.508 |
| walker |  | 7132 | 20 | python decl doc at beets/library/exceptions.py:34 |  |  | 0.508 |
| walker |  | 7188 | 56 | python method sigs in beets/library/exceptions.py |  |  | 0.508 |
| walker |  | 7188 | 0 | python method at beets/library/exceptions.py:11 |  |  | 0.508 |
| walker |  | 7188 | 0 | python method at beets/library/exceptions.py:19 |  |  | 0.508 |
| walker |  | 7188 | 0 | python method at beets/library/exceptions.py:30 |  |  | 0.508 |
| walker |  | 7188 | 0 | python method at beets/library/exceptions.py:37 |  |  | 0.508 |
| walker |  | 7200 | 12 | python method body at beets/library/exceptions.py:30 body 31 |  |  | 0.508 |
| walker |  | 7212 | 12 | python method body at beets/library/exceptions.py:37 body 38 |  |  | 0.508 |
| ns | 7271 |  | 272 | `Query` hierarchy class headers (full chain) | 5.3 |  | 0.499 |
| walker |  | 7289 | 77 | python decl names surface #2 in beets/ui/__init__.py |  |  | 0.502 |
| walker |  | 7289 | 0 | python decl at beets/ui/__init__.py:867 |  |  | 0.502 |
| walker |  | 7289 | 0 | python decl at beets/ui/__init__.py:879 |  |  | 0.502 |
| walker |  | 7289 | 0 | python decl at beets/ui/__init__.py:905 |  |  | 0.502 |
| walker |  | 7289 | 0 | python decl at beets/ui/__init__.py:996 |  |  | 0.502 |
| walker |  | 7302 | 13 | python decl doc at beets/ui/__init__.py:879 |  |  | 0.502 |
| walker |  | 7339 | 37 | python decl doc at beets/ui/__init__.py:996 |  |  | 0.502 |
| ns | 7346 |  | 75 | `Sort` class hierarchy (locations) | 5.4 |  | 0.499 |
| walker |  | 7368 | 29 | python decl doc at beets/ui/__init__.py:905 |  |  | 0.499 |
| walker |  | 7446 | 78 | python decl body at beets/ui/commands/__init__.py:36 body 38 |  |  | 0.513 |
| walker |  | 7488 | 42 | python method body at beets/util/__init__.py:90 body 91 |  |  | 0.513 |
| walker |  | 7497 | 9 | python imports #18 in beets/util/__init__.py |  |  | 0.513 |
| ns | 7547 |  | 201 | `Type` hierarchy class headers | 5.5 |  | 0.506 |
| ns | 7601 |  | 54 | `queryparse` top-level fn locations | 5.6 |  | 0.504 |
| ns | 7733 |  | 132 | `PARSE_QUERY_PART_REGEX` — the query-string syntax | 5.7 | 5.6 | 0.500 |
| walker |  | 7743 | 246 | python imports in beets/__main__.py |  |  | 0.508 |
| walker |  | 7811 | 68 | python class body at beets/importer/state.py:34 |  |  | 0.508 |
| walker |  | 7864 | 53 | python decl doc at beets/library/exceptions.py:4 |  |  | 0.504 |
| ns | 7864 |  | 131 | Autotag entry points: `tag_album` + `tag_item` signatures | 6.1 |  | 0.504 |
| walker |  | 7882 | 18 | python decl names surface #1 in beets/util/color.py |  |  | 0.504 |
| walker |  | 7882 | 0 | python decl at beets/util/color.py:208 |  |  | 0.504 |
| walker |  | 7927 | 45 | python decl names surface in beets/importer/session.py |  |  | 0.504 |
| walker |  | 7927 | 0 | python decl at beets/importer/session.py:42 |  |  | 0.504 |
| walker |  | 7927 | 0 | python decl at beets/importer/session.py:48 |  |  | 0.504 |
| walker |  | 7934 | 7 | python class body at beets/importer/session.py:42 |  |  | 0.504 |
| walker |  | 7948 | 14 | python decl doc at beets/importer/session.py:42 |  |  | 0.504 |
| ns | 7963 |  | 99 | `Recommendation` + `Proposal` + `Match`/`AlbumMatch`/`TrackMatch` + `Distance` + `Info`/`AlbumInfo`/`TrackInfo` | 6.2 |  | 0.500 |
| walker |  | 7984 | 36 | python decl doc at beets/importer/session.py:48 |  |  | 0.500 |
| walker |  | 8003 | 19 | python decl body at beets/ui/__init__.py:111 body 118 |  |  | 0.500 |
| walker |  | 8049 | 46 | python decl names surface in beets/util/id_extractors.py |  |  | 0.500 |
| walker |  | 8049 | 0 | python decl at beets/util/id_extractors.py:50 |  |  | 0.500 |
| ns | 8174 |  | 211 | `Action` enum + `ImportTask` class header | 7.1 |  | 0.494 |
| walker |  | 8200 | 151 | python decl names surface #1 in beets/ui/__init__.py |  |  | 0.508 |
| walker |  | 8200 | 0 | python decl at beets/ui/__init__.py:383 |  |  | 0.508 |
| walker |  | 8200 | 0 | python decl at beets/ui/__init__.py:395 |  |  | 0.508 |
| walker |  | 8200 | 0 | python decl at beets/ui/__init__.py:433 |  |  | 0.508 |
| walker |  | 8200 | 0 | python decl at beets/ui/__init__.py:445 |  |  | 0.508 |
| walker |  | 8200 | 0 | python decl at beets/ui/__init__.py:637 |  |  | 0.508 |
| walker |  | 8200 | 0 | python decl at beets/ui/__init__.py:676 |  |  | 0.508 |
| walker |  | 8200 | 0 | python decl at beets/ui/__init__.py:830 |  |  | 0.508 |
| walker |  | 8209 | 9 | python decl at beets/ui/__init__.py:459 |  |  | 0.510 |
| walker |  | 8223 | 14 | python decl at beets/ui/__init__.py:498 |  |  | 0.510 |
| walker |  | 8240 | 17 | python class body at beets/ui/__init__.py:676 |  |  | 0.510 |
| walker |  | 8261 | 21 | python class body at beets/ui/__init__.py:637 |  |  | 0.511 |
| walker |  | 8275 | 14 | python decl doc at beets/ui/__init__.py:445 |  |  | 0.511 |
| walker |  | 8291 | 16 | python decl doc at beets/ui/__init__.py:459 |  |  | 0.511 |
| walker |  | 8320 | 29 | python decl doc at beets/ui/__init__.py:676 |  |  | 0.511 |
| walker |  | 8350 | 30 | python decl doc at beets/ui/__init__.py:433 |  |  | 0.511 |
| walker |  | 8386 | 36 | python decl doc at beets/ui/__init__.py:637 |  |  | 0.515 |
| walker |  | 8410 | 24 | python decl at beets/ui/__init__.py:807 |  |  | 0.515 |
| walker |  | 8425 | 15 | python decl doc at beets/ui/__init__.py:830 |  |  | 0.515 |
| ns | 8488 |  | 314 | `ImportSession.run()` — pipeline assembly | 7.2 |  | 0.505 |
| walker |  | 8495 | 70 | python decl at beets/ui/__init__.py:466 |  |  | 0.505 |
| walker |  | 8548 | 53 | python decl doc at beets/ui/__init__.py:383 |  |  | 0.505 |
| walker |  | 8651 | 103 | python decl at beets/ui/__init__.py:207 |  |  | 0.505 |
| ns | 8655 |  | 167 | Importer stage functions (locations) | 7.3 |  | 0.501 |
| walker |  | 8697 | 46 | python decl doc at beets/ui/__init__.py:807 |  |  | 0.503 |
| walker |  | 8829 | 132 | python decl doc at beets/ui/__init__.py:498 |  |  | 0.503 |
| ns | 8907 |  | 252 | `BeetsPlugin` class header + registration-method locations | 8.1 |  | 0.498 |
| walker |  | 8959 | 130 | python decl doc at beets/ui/__init__.py:466 |  |  | 0.498 |
| walker |  | 9095 | 136 | python decl doc at beets/ui/__init__.py:395 |  |  | 0.498 |
| ns | 9216 |  | 309 | `EventType` literal — every event a listener can hook | 8.2 |  | 0.487 |
| walker |  | 9380 | 285 | python method sigs #1 in beets/ui/__init__.py |  |  | 0.489 |
| walker |  | 9380 | 0 | python method at beets/ui/__init__.py:514 |  |  | 0.489 |
| walker |  | 9380 | 0 | python method at beets/ui/__init__.py:521 |  |  | 0.489 |
| walker |  | 9380 | 0 | python method at beets/ui/__init__.py:571 |  |  | 0.489 |
| walker |  | 9380 | 0 | python method at beets/ui/__init__.py:591 |  |  | 0.489 |
| walker |  | 9380 | 0 | python method at beets/ui/__init__.py:621 |  |  | 0.489 |
| walker |  | 9380 | 0 | python method at beets/ui/__init__.py:644 |  |  | 0.489 |
| walker |  | 9380 | 0 | python method at beets/ui/__init__.py:658 |  |  | 0.489 |
| walker |  | 9380 | 0 | python method at beets/ui/__init__.py:661 |  |  | 0.489 |
| walker |  | 9380 | 0 | python method at beets/ui/__init__.py:681 |  |  | 0.489 |
| walker |  | 9380 | 0 | python method at beets/ui/__init__.py:701 |  |  | 0.489 |
| walker |  | 9380 | 0 | python method at beets/ui/__init__.py:760 |  |  | 0.489 |
| walker |  | 9380 | 0 | python method at beets/ui/__init__.py:770 |  |  | 0.489 |
| walker |  | 9380 | 0 | python method at beets/ui/__init__.py:783 |  |  | 0.489 |
| walker |  | 9388 | 8 | python method at beets/ui/__init__.py:664 |  |  | 0.489 |
| walker |  | 9399 | 11 | python method at beets/ui/__init__.py:668 |  |  | 0.489 |
| walker |  | 9415 | 16 | python method at beets/ui/__init__.py:708 |  |  | 0.489 |
| walker |  | 9423 | 8 | python method body at beets/ui/__init__.py:658 body 659 |  |  | 0.489 |
| ns | 9425 |  | 209 | `load_plugins`, `find_plugins`, dispatch entry points (signatures) | 8.3 |  | 0.485 |
| walker |  | 9435 | 12 | python method doc at beets/ui/__init__.py:621 |  |  | 0.485 |
| walker |  | 9445 | 10 | python method body at beets/ui/__init__.py:661 body 662 |  |  | 0.485 |
| walker |  | 9456 | 11 | python method body at beets/ui/__init__.py:664 body 666 |  |  | 0.485 |
| walker |  | 9473 | 17 | python method doc at beets/ui/__init__.py:701 |  |  | 0.485 |
| walker |  | 9510 | 37 | python method doc at beets/ui/__init__.py:770 |  |  | 0.485 |
| walker |  | 9562 | 52 | python method doc at beets/ui/__init__.py:783 |  |  | 0.485 |
| ns | 9580 |  | 155 | `MetadataSourcePlugin` + entry-fn locations | 8.4 |  | 0.483 |
| walker |  | 9618 | 56 | python method doc at beets/ui/__init__.py:681 |  |  | 0.483 |
| walker |  | 9650 | 32 | python method body at beets/ui/__init__.py:701 body 703 |  |  | 0.483 |
| walker |  | 9723 | 73 | python method doc at beets/ui/__init__.py:521 |  |  | 0.483 |
| walker |  | 9818 | 95 | python method doc at beets/ui/__init__.py:571 |  |  | 0.483 |
| walker |  | 9913 | 95 | python method doc at beets/ui/__init__.py:644 |  |  | 0.485 |
| walker |  | 9923 | 10 | python method body at beets/ui/__init__.py:571 body 589 |  |  | 0.485 |
| walker |  | 9931 | 8 | python decl body at beets/ui/__init__.py:445 body 447 |  |  | 0.485 |
| walker |  | 9939 | 8 | python decl body at beets/ui/__init__.py:445 body 456 |  |  | 0.485 |
| ns | 9963 |  | 383 | `beetsplug/` listing — every shipped plugin | 9.1 |  | 0.461 |
| walker |  | 9980 | 41 | listing of 'docs/dev/plugins/other' |  |  | 0.461 |
| walker |  | 10000 | 20 | python decl body at beets/ui/__init__.py:459 body 463 |  |  | 0.461 |
