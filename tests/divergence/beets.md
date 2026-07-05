Score(3000)=0.639 I=0.793 C=0.515 ns_rows≤3K=17/50 (reached=8 partial=1 missing=8)

| source | ns_cum | walker_cum | marginal | descriptor | id | predecessor | Score(B=cum) |
|:-------|-------:|-----------:|---------:|:-----------|:---|:------------|-------------:|
| walker |  | 83 | 83 | listing of '.' |  |  | 0.000 |
| ns | 102 |  | 102 | README lede + tagline | 1.1 |  | 0.000 |
| walker |  | 112 | 29 | entry-point scripts in pyproject.toml |  |  | 0.000 |
| walker |  | 131 | 19 | listing of 'extra' |  |  | 0.000 |
| ns | 185 |  | 83 | Top-level repo listing | 1.2 |  | 0.482 |
| walker |  | 198 | 67 | listing of 'beets' |  |  | 0.516 |
| walker |  | 207 | 9 | listing of 'beets/ui' |  |  | 0.516 |
| ns | 219 |  | 34 | Entry-point scripts (`beet = beets.ui:main`) | 1.3 |  | 0.508 |
| ns | 313 |  | 94 | `python -m beets` shim | 1.4 |  | 0.427 |
| ns | 406 |  | 93 | Package version + global config singleton | 1.5 |  | 0.398 |
| walker |  | 494 | 287 | python imports in beets/__init__.py |  |  | 0.406 |
| walker |  | 512 | 18 | listing of 'beets/autotag' |  |  | 0.407 |
| walker |  | 534 | 22 | listing of 'beets/importer' |  |  | 0.410 |
| walker |  | 551 | 17 | python decl names surface in beets/autotag/__init__.py |  |  | 0.410 |
| walker |  | 551 | 0 | python decl at beets/autotag/__init__.py:28 |  |  | 0.410 |
| walker |  | 579 | 28 | listing of 'beets/dbcore' |  |  | 0.419 |
| ns | 589 |  | 183 | Project metadata + Python version constraint | 1.6 |  | 0.356 |
| walker |  | 609 | 30 | listing of 'beets/library' |  |  | 0.369 |
| walker |  | 641 | 32 | python decl names surface in beets/library/__init__.py |  |  | 0.369 |
| walker |  | 641 | 0 | python decl at beets/library/__init__.py:15 |  |  | 0.369 |
| walker |  | 708 | 67 | listing of 'beets/ui/commands' |  |  | 0.377 |
| walker |  | 722 | 14 | listing of 'beets/ui/commands/import_' |  |  | 0.379 |
| walker |  | 749 | 27 | python decl names surface in beets/ui/commands/__init__.py |  |  | 0.379 |
| walker |  | 749 | 0 | python decl at beets/ui/commands/__init__.py:36 |  |  | 0.379 |
| walker |  | 759 | 10 | python decl doc at beets/ui/commands/__init__.py:36 |  |  | 0.379 |
| ns | 820 |  | 231 | Core (non-optional) runtime deps | 1.7 |  | 0.329 |
| walker |  | 848 | 89 | python imports in beets/ui/commands/import_/__init__.py |  |  | 0.329 |
| ns | 887 |  | 67 | `beets/` core package listing | 2.1 |  | 0.384 |
| walker |  | 893 | 45 | python decl names surface in beets/__init__.py |  |  | 0.400 |
| walker |  | 893 | 0 | python decl at beets/__init__.py:26 |  |  | 0.400 |
| walker |  | 893 | 0 | python decl at beets/__init__.py:35 |  |  | 0.400 |
| walker |  | 903 | 10 | python decl doc at beets/__init__.py:26 |  |  | 0.400 |
| walker |  | 929 | 26 | python method sigs in beets/__init__.py |  |  | 0.400 |
| walker |  | 929 | 0 | python method at beets/__init__.py:40 |  |  | 0.400 |
| ns | 985 |  | 98 | Core subpackage listings: dbcore, library, autotag, importer | 2.2 |  | 0.437 |
| walker |  | 1004 | 75 | listing of 'beets/util' |  |  | 0.458 |
| walker |  | 1018 | 14 | python imports in beets/util/__init__.py |  |  | 0.458 |
| walker |  | 1029 | 11 | python imports #1 in beets/util/__init__.py |  |  | 0.458 |
| walker |  | 1037 | 8 | python imports #2 in beets/util/__init__.py |  |  | 0.458 |
| walker |  | 1044 | 7 | python imports #3 in beets/util/__init__.py |  |  | 0.458 |
| walker |  | 1052 | 8 | python imports #4 in beets/util/__init__.py |  |  | 0.458 |
| walker |  | 1059 | 7 | python imports #5 in beets/util/__init__.py |  |  | 0.458 |
| walker |  | 1066 | 7 | python imports #6 in beets/util/__init__.py |  |  | 0.458 |
| walker |  | 1073 | 7 | python imports #7 in beets/util/__init__.py |  |  | 0.458 |
| walker |  | 1081 | 8 | python imports #8 in beets/util/__init__.py |  |  | 0.458 |
| walker |  | 1102 | 21 | python decl body at beets/library/__init__.py:15 body 16 |  |  | 0.458 |
| walker |  | 1109 | 7 | python imports #9 in beets/util/__init__.py |  |  | 0.458 |
| walker |  | 1149 | 40 | python decl doc at beets/__init__.py:35 |  |  | 0.472 |
| ns | 1150 |  | 165 | UI + util + commands listings | 2.3 |  | 0.500 |
| walker |  | 1321 | 172 | python imports in beets/library/__init__.py |  |  | 0.509 |
| walker |  | 1328 | 7 | python imports #10 in beets/util/__init__.py |  |  | 0.509 |
| walker |  | 1335 | 7 | python imports #11 in beets/util/__init__.py |  |  | 0.509 |
| walker |  | 1404 | 69 | [package] in pyproject.toml |  |  | 0.519 |
| walker |  | 1418 | 14 | python decl names surface in beets/library/library.py |  |  | 0.519 |
| walker |  | 1418 | 0 | python decl at beets/library/library.py:20 |  |  | 0.519 |
| ns | 1425 |  | 275 | `beets.dbcore` public re-exports | 2.4 |  | 0.464 |
| walker |  | 1433 | 15 | python decl doc at beets/library/library.py:20 |  |  | 0.464 |
| walker |  | 1440 | 7 | python imports #12 in beets/util/__init__.py |  |  | 0.464 |
| walker |  | 1558 | 118 | python decl names surface in beets/ui/commands/import_/__init__.py |  |  | 0.464 |
| walker |  | 1558 | 0 | python decl at beets/ui/commands/import_/__init__.py:14 |  |  | 0.464 |
| walker |  | 1558 | 0 | python decl at beets/ui/commands/import_/__init__.py:34 |  |  | 0.464 |
| walker |  | 1558 | 0 | python decl at beets/ui/commands/import_/__init__.py:49 |  |  | 0.464 |
| walker |  | 1558 | 0 | python decl at beets/ui/commands/import_/__init__.py:81 |  |  | 0.464 |
| walker |  | 1558 | 0 | python decl at beets/ui/commands/import_/__init__.py:134 |  |  | 0.464 |
| walker |  | 1584 | 26 | python decl at beets/ui/commands/import_/__init__.py:160 |  |  | 0.464 |
| walker |  | 1602 | 18 | python decl doc at beets/ui/commands/import_/__init__.py:34 |  |  | 0.464 |
| walker |  | 1634 | 32 | python decl doc at beets/ui/commands/import_/__init__.py:49 |  |  | 0.464 |
| walker |  | 1667 | 33 | python decl doc at beets/ui/commands/import_/__init__.py:14 |  |  | 0.464 |
| ns | 1742 |  | 317 | `beets.library` public re-exports | 2.5 |  | 0.455 |
| walker |  | 1744 | 77 | listing of 'docs' |  |  | 0.458 |
| walker |  | 1766 | 22 | listing of 'docs/api' |  |  | 0.459 |
| walker |  | 1792 | 26 | listing of 'docs/guides' |  |  | 0.461 |
| walker |  | 1818 | 26 | listing of 'docs/reference' |  |  | 0.464 |
| walker |  | 1846 | 28 | listing of 'docs/dev' |  |  | 0.467 |
| walker |  | 1913 | 67 | python decl at beets/library/__init__.py:8 |  |  | 0.488 |
| walker |  | 1920 | 7 | python imports #13 in beets/util/__init__.py |  |  | 0.488 |
| walker |  | 1924 | 4 | listing of 'docs/_templates' |  |  | 0.488 |
| walker |  | 1945 | 21 | listing of 'docs/_templates/autosummary' |  |  | 0.488 |
| walker |  | 1949 | 4 | listing of 'docs/extensions' |  |  | 0.488 |
| walker |  | 1964 | 15 | listing of 'beets/test' |  |  | 0.488 |
| walker |  | 1988 | 24 | listing of 'docs/dev/plugins' |  |  | 0.492 |
| walker |  | 1997 | 9 | python imports in beets/util/units.py |  |  | 0.492 |
| walker |  | 2017 | 20 | python decl names surface in beets/library/fields.py |  |  | 0.492 |
| walker |  | 2037 | 20 | python decl names surface in beets/util/hidden.py |  |  | 0.492 |
| walker |  | 2037 | 0 | python decl at beets/util/hidden.py:25 |  |  | 0.492 |
| ns | 2110 |  | 368 | `beets.autotag` public re-exports | 2.6 |  | 0.450 |
| ns | 2330 |  | 220 | `beets.importer` public re-exports | 2.7 |  | 0.426 |
| ns | 2533 |  | 203 | Docs tree + reference + dev-doc table-of-contents | 2.8 |  | 0.453 |
| walker |  | 2631 | 594 | README headline in README.rst |  |  | 0.677 |
| ns | 2711 |  | 178 | `UserError` + `Subcommand` class header | 3.1 |  | 0.661 |
| ns | 2996 |  | 285 | Top-level fns in `beets/ui/__init__.py` (signature heads only) | 3.2 |  | 0.639 |
| walker |  | 3021 | 390 | python imports in beets/importer/__init__.py |  |  | 0.677 |
| walker |  | 3416 | 395 | python imports in beets/autotag/__init__.py |  |  | 0.701 |
| ns | 3444 |  | 448 | `main()` top-level exception handlers | 3.3 | 3.2 | 0.665 |
| ns | 3540 |  | 96 | `_raw_main()` global-option setup | 3.4 | 3.2 | 0.659 |
| ns | 3798 |  | 258 | `_raw_main()` parse + dispatch tail | 3.5 | 3.4 | 0.638 |
| walker |  | 3823 | 407 | python imports in beets/ui/commands/__init__.py |  |  | 0.639 |
| walker |  | 3833 | 10 | python imports in beets/library/fields.py |  |  | 0.639 |
| walker |  | 3848 | 15 | python decl names surface in beets/ui/commands/help.py |  |  | 0.639 |
| walker |  | 3848 | 0 | python decl at beets/ui/commands/help.py:6 |  |  | 0.639 |
| walker |  | 3857 | 9 | python imports #14 in beets/util/__init__.py |  |  | 0.639 |
| walker |  | 3885 | 28 | python method sigs in beets/ui/commands/help.py |  |  | 0.639 |
| walker |  | 3885 | 0 | python method at beets/ui/commands/help.py:7 |  |  | 0.639 |
| walker |  | 3885 | 0 | python method at beets/ui/commands/help.py:14 |  |  | 0.639 |
| walker |  | 3897 | 12 | python imports in beets/library/exceptions.py |  |  | 0.639 |
| walker |  | 3923 | 26 | python decl names surface in beets/util/m3u.py |  |  | 0.639 |
| walker |  | 3923 | 0 | python decl at beets/util/m3u.py:22 |  |  | 0.639 |
| walker |  | 3923 | 0 | python decl at beets/util/m3u.py:28 |  |  | 0.639 |
| walker |  | 3930 | 7 | python class body at beets/util/m3u.py:22 |  |  | 0.639 |
| walker |  | 3947 | 17 | python decl doc at beets/util/m3u.py:22 |  |  | 0.639 |
| walker |  | 3967 | 20 | python decl doc at beets/util/m3u.py:28 |  |  | 0.639 |
| ns | 4007 |  | 209 | `_setup()` body: load plugins, build commands, open library | 3.6 | 3.2 | 0.622 |
| ns | 4419 |  | 412 | `default_commands` list (built-in subcommands) | 3.7 |  | 0.600 |
| walker |  | 4470 | 503 | python imports in beets/ui/__init__.py |  |  | 0.600 |
| ns | 4655 |  | 236 | Built-in command registrations (one `Subcommand` line per cmd) | 3.8 |  | 0.591 |
| ns | 4912 |  | 257 | Smallest command as concrete template (`list`) | 3.9 | 3.8 | 0.574 |
| walker |  | 4976 | 506 | python imports in beets/dbcore/__init__.py |  |  | 0.626 |
| walker |  | 5033 | 57 | python decl body at beets/__init__.py:26 body 28 |  |  | 0.626 |
| ns | 5102 |  | 190 | `Library` class header + `_models` + `_migrations` | 4.1 |  | 0.618 |
| walker |  | 5198 | 165 | python decl names surface in beets/ui/__init__.py |  |  | 0.621 |
| walker |  | 5198 | 0 | python decl at beets/ui/__init__.py:71 |  |  | 0.621 |
| walker |  | 5198 | 0 | python decl at beets/ui/__init__.py:80 |  |  | 0.621 |
| walker |  | 5198 | 0 | python decl at beets/ui/__init__.py:85 |  |  | 0.621 |
| walker |  | 5198 | 0 | python decl at beets/ui/__init__.py:90 |  |  | 0.621 |
| walker |  | 5198 | 0 | python decl at beets/ui/__init__.py:111 |  |  | 0.621 |
| walker |  | 5198 | 0 | python decl at beets/ui/__init__.py:122 |  |  | 0.621 |
| walker |  | 5198 | 0 | python decl at beets/ui/__init__.py:150 |  |  | 0.621 |
| walker |  | 5198 | 0 | python decl at beets/ui/__init__.py:160 |  |  | 0.621 |
| walker |  | 5198 | 0 | python decl at beets/ui/__init__.py:167 |  |  | 0.621 |
| walker |  | 5198 | 0 | python decl at beets/ui/__init__.py:187 |  |  | 0.621 |
| walker |  | 5198 | 0 | python decl at beets/ui/__init__.py:383 |  |  | 0.621 |
| walker |  | 5220 | 22 | python decl body at beets/ui/__init__.py:160 body 164 |  |  | 0.621 |
| walker |  | 5257 | 37 | python decl doc at beets/ui/__init__.py:71 |  |  | 0.623 |
| ns | 5261 |  | 159 | `Library` query/fetch method headers | 4.2 | 4.1 | 0.616 |
| walker |  | 5294 | 37 | python decl doc at beets/ui/__init__.py:160 |  |  | 0.616 |
| walker |  | 5306 | 12 | python decl body at beets/ui/__init__.py:80 body 82 |  |  | 0.616 |
| walker |  | 5318 | 12 | python decl body at beets/ui/__init__.py:85 body 87 |  |  | 0.616 |
| walker |  | 5382 | 64 | python decl at beets/ui/__init__.py:207 |  |  | 0.616 |
| walker |  | 5401 | 19 | python decl doc at beets/ui/__init__.py:80 |  |  | 0.616 |
| walker |  | 5420 | 19 | python decl doc at beets/ui/__init__.py:85 |  |  | 0.616 |
| walker |  | 5440 | 20 | python decl doc at beets/ui/__init__.py:150 |  |  | 0.616 |
| ns | 5472 |  | 211 | `LibModel` + `Item` + `Album` class headers | 4.3 |  | 0.603 |
| walker |  | 5495 | 55 | python decl doc at beets/ui/__init__.py:383 |  |  | 0.603 |
| walker |  | 5504 | 9 | python decl body at beets/ui/__init__.py:111 body 119 |  |  | 0.603 |
| ns | 5560 |  | 88 | `_search_fields` for Item + Album (default query targets) | 4.4 | 4.3 | 0.597 |
| walker |  | 5577 | 73 | python decl doc at beets/ui/__init__.py:187 |  |  | 0.597 |
| walker |  | 5658 | 81 | python decl doc at beets/ui/__init__.py:111 |  |  | 0.597 |
| walker |  | 5747 | 89 | python decl doc at beets/ui/__init__.py:122 |  |  | 0.597 |
| ns | 5764 |  | 204 | `TYPE_BY_FIELD` lede + first dozen fields | 4.5 |  | 0.589 |
| walker |  | 5803 | 56 | python decl body at beets/ui/__init__.py:167 body 177 |  |  | 0.589 |
| walker |  | 5910 | 107 | python decl at beets/ui/commands/__init__.py:50 |  |  | 0.605 |
| walker |  | 5934 | 24 | python imports in beets/context.py |  |  | 0.605 |
| walker |  | 6114 | 180 | python decl names surface in beets/util/__init__.py |  |  | 0.605 |
| walker |  | 6114 | 0 | python decl at beets/util/__init__.py:74 |  |  | 0.605 |
| walker |  | 6114 | 0 | python decl at beets/util/__init__.py:130 |  |  | 0.605 |
| walker |  | 6114 | 0 | python decl at beets/util/__init__.py:158 |  |  | 0.605 |
| walker |  | 6114 | 0 | python decl at beets/util/__init__.py:169 |  |  | 0.605 |
| walker |  | 6114 | 0 | python decl at beets/util/__init__.py:175 |  |  | 0.605 |
| walker |  | 6114 | 0 | python decl at beets/util/__init__.py:184 |  |  | 0.605 |
| walker |  | 6114 | 0 | python decl at beets/util/__init__.py:273 |  |  | 0.605 |
| walker |  | 6114 | 0 | python decl at beets/util/__init__.py:280 |  |  | 0.605 |
| walker |  | 6114 | 0 | python decl at beets/util/__init__.py:294 |  |  | 0.605 |
| walker |  | 6114 | 0 | python decl at beets/util/__init__.py:356 |  |  | 0.605 |
| walker |  | 6136 | 22 | python class body at beets/util/__init__.py:74 |  |  | 0.605 |
| ns | 6140 |  | 376 | `parse_query_parts` + `parse_query_string` (library-level query entry) | 4.6 |  | 0.587 |
| walker |  | 6161 | 25 | python class body at beets/util/__init__.py:169 |  |  | 0.587 |
| walker |  | 6181 | 20 | python decl doc at beets/util/__init__.py:158 |  |  | 0.587 |
| walker |  | 6196 | 15 | python decl body at beets/util/__init__.py:273 body 277 |  |  | 0.587 |
| walker |  | 6247 | 51 | python decl at beets/util/__init__.py:309 |  |  | 0.587 |
| walker |  | 6276 | 29 | python decl doc at beets/util/__init__.py:273 |  |  | 0.587 |
| walker |  | 6308 | 32 | python decl doc at beets/util/__init__.py:175 |  |  | 0.587 |
| walker |  | 6342 | 34 | python decl doc at beets/util/__init__.py:280 |  |  | 0.587 |
| walker |  | 6406 | 64 | python class body at beets/util/__init__.py:158 |  |  | 0.587 |
| walker |  | 6448 | 42 | python decl doc at beets/util/__init__.py:294 |  |  | 0.587 |
| walker |  | 6518 | 70 | python decl at beets/util/__init__.py:208 |  |  | 0.587 |
| ns | 6521 |  | 381 | `LibModel` + `Item` + `Album` method names (locations) | 4.7 | 4.3 | 0.575 |
| walker |  | 6574 | 56 | python decl doc at beets/util/__init__.py:130 |  |  | 0.575 |
| walker |  | 6582 | 8 | python decl body at beets/util/__init__.py:294 body 306 |  |  | 0.575 |
| walker |  | 6665 | 83 | python decl doc at beets/util/__init__.py:208 |  |  | 0.575 |
| walker |  | 6749 | 84 | python decl doc at beets/util/__init__.py:356 |  |  | 0.575 |
| ns | 6786 |  | 265 | `DefaultTemplateFunctions` header + `tmpl_*` method locations | 4.8 |  | 0.567 |
| walker |  | 6847 | 98 | python decl doc at beets/util/__init__.py:184 |  |  | 0.567 |
| ns | 6917 |  | 131 | Library migration class headers (full chain) | 4.9 |  | 0.563 |
| walker |  | 7052 | 205 | python method sigs in beets/util/__init__.py |  |  | 0.563 |
| walker |  | 7052 | 0 | python method at beets/util/__init__.py:90 |  |  | 0.563 |
| walker |  | 7052 | 0 | python method at beets/util/__init__.py:96 |  |  | 0.563 |
| walker |  | 7052 | 0 | python method at beets/util/__init__.py:104 |  |  | 0.563 |
| walker |  | 7052 | 0 | python method at beets/util/__init__.py:115 |  |  | 0.563 |
| walker |  | 7052 | 0 | python method at beets/util/__init__.py:121 |  |  | 0.563 |
| walker |  | 7052 | 0 | python method at beets/util/__init__.py:136 |  |  | 0.563 |
| walker |  | 7052 | 0 | python method at beets/util/__init__.py:1094 |  |  | 0.563 |
| walker |  | 7052 | 0 | python method at beets/util/__init__.py:1098 |  |  | 0.563 |
| walker |  | 7052 | 0 | python method at beets/util/__init__.py:1102 |  |  | 0.563 |
| walker |  | 7052 | 0 | python method at beets/util/__init__.py:1145 |  |  | 0.563 |
| walker |  | 7069 | 17 | python method at beets/util/__init__.py:140 |  |  | 0.563 |
| walker |  | 7079 | 10 | python method body at beets/util/__init__.py:115 body 119 |  |  | 0.563 |
| walker |  | 7090 | 11 | python method body at beets/util/__init__.py:1098 body 1100 |  |  | 0.563 |
| walker |  | 7105 | 15 | python method doc at beets/util/__init__.py:1098 |  |  | 0.563 |
| walker |  | 7121 | 16 | python method doc at beets/util/__init__.py:1094 |  |  | 0.563 |
| walker |  | 7140 | 19 | python method doc at beets/util/__init__.py:1102 |  |  | 0.563 |
| ns | 7153 |  | 236 | `Model` + `Database` class headers | 5.1 |  | 0.556 |
| walker |  | 7155 | 15 | python method body at beets/util/__init__.py:1094 body 1096 |  |  | 0.556 |
| walker |  | 7183 | 28 | python method doc at beets/util/__init__.py:115 |  |  | 0.556 |
| walker |  | 7196 | 13 | python method doc at beets/util/__init__.py:104 |  |  | 0.556 |
| walker |  | 7237 | 41 | python method doc at beets/util/__init__.py:121 |  |  | 0.556 |
| walker |  | 7260 | 23 | python method body at beets/util/__init__.py:136 body 137 |  |  | 0.556 |
| ns | 7267 |  | 114 | Top-level classes in `beets/dbcore/db.py` (locations) | 5.2 |  | 0.551 |
| walker |  | 7279 | 19 | python method doc at beets/util/__init__.py:96 |  |  | 0.551 |
| walker |  | 7335 | 56 | python method sigs in beets/util/m3u.py |  |  | 0.551 |
| walker |  | 7335 | 0 | python method at beets/util/m3u.py:31 |  |  | 0.551 |
| walker |  | 7335 | 0 | python method at beets/util/m3u.py:43 |  |  | 0.551 |
| walker |  | 7335 | 0 | python method at beets/util/m3u.py:63 |  |  | 0.551 |
| walker |  | 7335 | 0 | python method at beets/util/m3u.py:77 |  |  | 0.551 |
| walker |  | 7453 | 118 | python decl doc at beets/ui/__init__.py:167 |  |  | 0.551 |
| walker |  | 7571 | 118 | python decl doc at beets/util/__init__.py:309 |  |  | 0.551 |
| ns | 7587 |  | 320 | `Query` hierarchy class headers (full chain) | 5.3 |  | 0.541 |
| walker |  | 7647 | 76 | python decl names surface in beets/context.py |  |  | 0.541 |
| walker |  | 7647 | 0 | python decl at beets/context.py:8 |  |  | 0.541 |
| walker |  | 7647 | 0 | python decl at beets/context.py:13 |  |  | 0.541 |
| walker |  | 7658 | 11 | python decl at beets/context.py:18 |  |  | 0.541 |
| walker |  | 7671 | 13 | python decl doc at beets/context.py:8 |  |  | 0.541 |
| ns | 7676 |  | 89 | `Sort` class hierarchy (locations) | 5.4 |  | 0.538 |
| walker |  | 7681 | 10 | python decl body at beets/context.py:8 body 10 |  |  | 0.538 |
| walker |  | 7694 | 13 | python decl doc at beets/context.py:13 |  |  | 0.538 |
| walker |  | 7704 | 10 | python decl body at beets/context.py:13 body 15 |  |  | 0.538 |
| walker |  | 7722 | 18 | python decl doc at beets/context.py:18 |  |  | 0.538 |
| walker |  | 7735 | 13 | python decl names surface #1 in beets/util/color.py |  |  | 0.538 |
| walker |  | 7788 | 53 | python decl doc at beets/ui/commands/import_/__init__.py:134 |  |  | 0.538 |
| walker |  | 7822 | 34 | python decl doc at beets/util/hidden.py:25 |  |  | 0.538 |
| walker |  | 7843 | 21 | python decl names surface in beets/ui/commands/utils.py |  |  | 0.538 |
| walker |  | 7843 | 0 | python decl at beets/ui/commands/utils.py:6 |  |  | 0.538 |
| walker |  | 7882 | 39 | listing of '.github' |  |  | 0.538 |
| walker |  | 7913 | 31 | listing of '.github/workflows' |  |  | 0.531 |
| ns | 7913 |  | 237 | `Type` hierarchy class headers | 5.5 |  | 0.531 |
| walker |  | 7947 | 34 | python decl names surface in beets/util/config.py |  |  | 0.531 |
| walker |  | 7947 | 0 | python decl at beets/util/config.py:78 |  |  | 0.531 |
| walker |  | 7962 | 15 | python method sigs in beets/util/config.py |  |  | 0.531 |
| walker |  | 7962 | 0 | python method at beets/util/config.py:79 |  |  | 0.531 |
| ns | 7981 |  | 68 | `queryparse` top-level fn locations | 5.6 |  | 0.528 |
| walker |  | 7987 | 25 | python decl at beets/util/config.py:9 |  |  | 0.528 |
| walker |  | 8042 | 55 | python decl at beets/util/config.py:29 |  |  | 0.528 |
| walker |  | 8062 | 20 | python method doc at beets/util/m3u.py:43 |  |  | 0.528 |
| walker |  | 8073 | 11 | python imports #15 in beets/util/__init__.py |  |  | 0.528 |
| ns | 8113 |  | 132 | `PARSE_QUERY_PART_REGEX` — the query-string syntax | 5.7 | 5.6 | 0.524 |
| walker |  | 8130 | 57 | python decl doc at beets/ui/__init__.py:90 |  |  | 0.524 |
| walker |  | 8165 | 35 | python decl names surface in beets/importer/state.py |  |  | 0.524 |
| walker |  | 8173 | 8 | python decl at beets/importer/state.py:34 |  |  | 0.524 |
| walker |  | 8188 | 15 | python decl doc at beets/importer/state.py:34 |  |  | 0.524 |
| ns | 8250 |  | 137 | Autotag entry points: `tag_album` + `tag_item` signatures | 6.1 |  | 0.520 |
| walker |  | 8258 | 70 | python decl names surface #2 in beets/ui/__init__.py |  |  | 0.522 |
| walker |  | 8258 | 0 | python decl at beets/ui/__init__.py:905 |  |  | 0.522 |
| walker |  | 8258 | 0 | python decl at beets/ui/__init__.py:996 |  |  | 0.522 |
| walker |  | 8285 | 27 | python decl at beets/ui/__init__.py:65 |  |  | 0.522 |
| walker |  | 8324 | 39 | python decl doc at beets/ui/__init__.py:996 |  |  | 0.522 |
| walker |  | 8355 | 31 | python decl doc at beets/ui/__init__.py:905 |  |  | 0.522 |
| ns | 8375 |  | 125 | `Recommendation` + `Proposal` + `Match`/`AlbumMatch`/`TrackMatch` + `Distance` + `Info`/`AlbumInfo`/`TrackInfo` | 6.2 |  | 0.518 |
| walker |  | 8520 | 165 | python decl doc at beets/util/__init__.py:74 |  |  | 0.518 |
| walker |  | 8530 | 10 | python imports #16 in beets/util/__init__.py |  |  | 0.518 |
| walker |  | 8566 | 36 | python decl names surface in beets/library/exceptions.py |  |  | 0.518 |
| walker |  | 8566 | 0 | python decl at beets/library/exceptions.py:4 |  |  | 0.518 |
| walker |  | 8566 | 0 | python decl at beets/library/exceptions.py:27 |  |  | 0.518 |
| walker |  | 8566 | 0 | python decl at beets/library/exceptions.py:34 |  |  | 0.518 |
| walker |  | 8588 | 22 | python decl doc at beets/library/exceptions.py:27 |  |  | 0.518 |
| ns | 8592 |  | 217 | `Action` enum + `ImportTask` class header | 7.1 |  | 0.511 |
| walker |  | 8610 | 22 | python decl doc at beets/library/exceptions.py:34 |  |  | 0.511 |
| walker |  | 8662 | 52 | python method sigs in beets/library/exceptions.py |  |  | 0.511 |
| walker |  | 8662 | 0 | python method at beets/library/exceptions.py:11 |  |  | 0.511 |
| walker |  | 8662 | 0 | python method at beets/library/exceptions.py:19 |  |  | 0.511 |
| walker |  | 8662 | 0 | python method at beets/library/exceptions.py:30 |  |  | 0.511 |
| walker |  | 8662 | 0 | python method at beets/library/exceptions.py:37 |  |  | 0.511 |
| walker |  | 8674 | 12 | python method body at beets/library/exceptions.py:30 body 31 |  |  | 0.511 |
| walker |  | 8686 | 12 | python method body at beets/library/exceptions.py:37 body 38 |  |  | 0.511 |
| walker |  | 8695 | 9 | python imports #17 in beets/util/__init__.py |  |  | 0.511 |
| walker |  | 8773 | 78 | python decl body at beets/ui/commands/__init__.py:36 body 38 |  |  | 0.525 |
| walker |  | 8815 | 42 | python method body at beets/util/__init__.py:90 body 91 |  |  | 0.525 |
| walker |  | 8832 | 17 | python decl body at beets/ui/__init__.py:111 body 118 |  |  | 0.525 |
| walker |  | 8841 | 9 | python imports #18 in beets/util/__init__.py |  |  | 0.525 |
| ns | 8914 |  | 322 | `ImportSession.run()` — pipeline assembly | 7.2 |  | 0.515 |
| walker |  | 8976 | 135 | python decl names surface in beets/util/deprecation.py |  |  | 0.515 |
| walker |  | 8976 | 0 | python decl at beets/util/deprecation.py:30 |  |  | 0.515 |
| walker |  | 8990 | 14 | python decl at beets/util/deprecation.py:16 |  |  | 0.515 |
| walker |  | 9018 | 28 | python decl at beets/util/deprecation.py:78 |  |  | 0.515 |
| walker |  | 9047 | 29 | python decl at beets/util/deprecation.py:39 |  |  | 0.515 |
| walker |  | 9059 | 12 | python decl body at beets/util/deprecation.py:39 body 42 |  |  | 0.515 |
| walker |  | 9091 | 32 | python decl at beets/util/deprecation.py:59 |  |  | 0.515 |
| ns | 9105 |  | 191 | Importer stage functions (locations) | 7.3 |  | 0.511 |
| walker |  | 9124 | 33 | python decl at beets/util/deprecation.py:45 |  |  | 0.511 |
| walker |  | 9193 | 69 | python decl at beets/util/deprecation.py:19 |  |  | 0.511 |
| walker |  | 9233 | 40 | python decl body at beets/util/deprecation.py:45 body 54 |  |  | 0.511 |
| walker |  | 9313 | 80 | python decl doc at beets/util/deprecation.py:45 |  |  | 0.511 |
| ns | 9377 |  | 272 | `BeetsPlugin` class header + registration-method locations | 8.1 |  | 0.506 |
| walker |  | 9403 | 90 | README.rst section #0 |  |  | 0.506 |
| walker |  | 9477 | 74 | python class body at beets/importer/state.py:34 |  |  | 0.506 |
| walker |  | 9566 | 89 | python decl body at beets/util/__init__.py:280 body 284 |  |  | 0.506 |
| walker |  | 9624 | 58 | python decl doc at beets/library/exceptions.py:4 |  |  | 0.506 |
| walker |  | 9665 | 41 | listing of 'docs/dev/plugins/other' |  |  | 0.506 |
| ns | 9690 |  | 313 | `EventType` literal — every event a listener can hook | 8.2 |  | 0.495 |
| walker |  | 9756 | 91 | python decl doc at beets/util/deprecation.py:59 |  |  | 0.495 |
| walker |  | 9805 | 49 | python decl names surface in beets/util/units.py |  |  | 0.495 |
| walker |  | 9805 | 0 | python decl at beets/util/units.py:4 |  |  | 0.495 |
| walker |  | 9805 | 0 | python decl at beets/util/units.py:17 |  |  | 0.495 |
| walker |  | 9805 | 0 | python decl at beets/util/units.py:25 |  |  | 0.495 |
| walker |  | 9805 | 0 | python decl at beets/util/units.py:37 |  |  | 0.495 |
| walker |  | 9825 | 20 | python decl doc at beets/util/units.py:25 |  |  | 0.495 |
| walker |  | 9858 | 33 | python decl doc at beets/util/units.py:17 |  |  | 0.495 |
| walker |  | 9894 | 36 | python decl doc at beets/util/units.py:37 |  |  | 0.495 |
| ns | 9925 |  | 235 | `load_plugins`, `find_plugins`, dispatch entry points (signatures) | 8.3 |  | 0.491 |
| ns | 10098 |  | 173 | `MetadataSourcePlugin` + entry-fn locations | 8.4 |  | 0.488 |
| ns | 10481 |  | 383 | `beetsplug/` listing — every shipped plugin | 9.1 |  | 0.465 |
