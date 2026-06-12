Score(3000)=0.619 I=0.714 C=0.536 ns_rows≤3K=17/50 (reached=6 partial=3 missing=8)

| source | ns_cum | walker_cum | marginal | descriptor | id | predecessor | Score(B=cum) |
|:-------|-------:|-----------:|---------:|:-----------|:---|:------------|-------------:|
| walker |  | 83 | 83 | listing of '.' |  |  | 0.000 |
| ns | 98 |  | 98 | README lede + tagline | 1.1 |  | 0.000 |
| walker |  | 108 | 25 | entry-point scripts in pyproject.toml |  |  | 0.000 |
| walker |  | 127 | 19 | listing of 'extra' |  |  | 0.000 |
| ns | 181 |  | 83 | Top-level repo listing | 1.2 |  | 0.482 |
| walker |  | 194 | 67 | listing of 'beets' |  |  | 0.516 |
| ns | 211 |  | 30 | Entry-point scripts (`beet = beets.ui:main`) | 1.3 |  | 0.508 |
| ns | 303 |  | 92 | `python -m beets` shim | 1.4 |  | 0.427 |
| ns | 390 |  | 87 | Package version + global config singleton | 1.5 |  | 0.398 |
| walker |  | 454 | 260 | python imports in beets/__init__.py |  |  | 0.406 |
| walker |  | 463 | 9 | listing of 'beets/ui' |  |  | 0.406 |
| walker |  | 481 | 18 | listing of 'beets/autotag' |  |  | 0.407 |
| walker |  | 503 | 22 | listing of 'beets/importer' |  |  | 0.410 |
| walker |  | 518 | 15 | python decl names surface in beets/autotag/__init__.py |  |  | 0.410 |
| walker |  | 518 | 0 | python decl at beets/autotag/__init__.py:28 |  |  | 0.410 |
| walker |  | 546 | 28 | listing of 'beets/dbcore' |  |  | 0.419 |
| ns | 571 |  | 181 | Project metadata + Python version constraint | 1.6 |  | 0.356 |
| walker |  | 576 | 30 | listing of 'beets/library' |  |  | 0.369 |
| walker |  | 604 | 28 | python decl names surface in beets/library/__init__.py |  |  | 0.369 |
| walker |  | 604 | 0 | python decl at beets/library/__init__.py:15 |  |  | 0.369 |
| walker |  | 671 | 67 | listing of 'beets/ui/commands' |  |  | 0.377 |
| walker |  | 685 | 14 | listing of 'beets/ui/commands/import_' |  |  | 0.379 |
| walker |  | 708 | 23 | python decl names surface in beets/ui/commands/__init__.py |  |  | 0.379 |
| walker |  | 708 | 0 | python decl at beets/ui/commands/__init__.py:36 |  |  | 0.379 |
| walker |  | 716 | 8 | python decl doc at beets/ui/commands/__init__.py:36 |  |  | 0.379 |
| walker |  | 788 | 72 | python imports in beets/ui/commands/import_/__init__.py |  |  | 0.379 |
| ns | 802 |  | 231 | Core (non-optional) runtime deps | 1.7 |  | 0.329 |
| walker |  | 863 | 75 | listing of 'beets/util' |  |  | 0.345 |
| ns | 869 |  | 67 | `beets/` core package listing | 2.1 |  | 0.403 |
| walker |  | 873 | 10 | python imports in beets/util/__init__.py |  |  | 0.403 |
| walker |  | 884 | 11 | python imports #1 in beets/util/__init__.py |  |  | 0.403 |
| walker |  | 892 | 8 | python imports #2 in beets/util/__init__.py |  |  | 0.403 |
| walker |  | 899 | 7 | python imports #3 in beets/util/__init__.py |  |  | 0.403 |
| walker |  | 907 | 8 | python imports #4 in beets/util/__init__.py |  |  | 0.403 |
| walker |  | 914 | 7 | python imports #5 in beets/util/__init__.py |  |  | 0.403 |
| walker |  | 921 | 7 | python imports #6 in beets/util/__init__.py |  |  | 0.403 |
| walker |  | 928 | 7 | python imports #7 in beets/util/__init__.py |  |  | 0.403 |
| ns | 967 |  | 98 | Core subpackage listings: dbcore, library, autotag, importer | 2.2 |  | 0.444 |
| walker |  | 975 | 47 | python decl names surface in beets/__init__.py |  |  | 0.458 |
| walker |  | 975 | 0 | python decl at beets/__init__.py:26 |  |  | 0.458 |
| walker |  | 975 | 0 | python decl at beets/__init__.py:35 |  |  | 0.458 |
| walker |  | 983 | 8 | python decl doc at beets/__init__.py:26 |  |  | 0.458 |
| walker |  | 1009 | 26 | python method sigs in beets/__init__.py |  |  | 0.458 |
| walker |  | 1009 | 0 | python method at beets/__init__.py:40 |  |  | 0.458 |
| walker |  | 1028 | 19 | python decl body at beets/library/__init__.py:15 body 16 |  |  | 0.458 |
| walker |  | 1036 | 8 | python imports #8 in beets/util/__init__.py |  |  | 0.458 |
| walker |  | 1043 | 7 | python imports #9 in beets/util/__init__.py |  |  | 0.458 |
| walker |  | 1083 | 40 | python decl doc at beets/__init__.py:35 |  |  | 0.472 |
| ns | 1132 |  | 165 | UI + util + commands listings | 2.3 |  | 0.500 |
| walker |  | 1254 | 171 | python imports in beets/library/__init__.py |  |  | 0.509 |
| walker |  | 1261 | 7 | python imports #10 in beets/util/__init__.py |  |  | 0.509 |
| walker |  | 1268 | 7 | python imports #11 in beets/util/__init__.py |  |  | 0.509 |
| walker |  | 1280 | 12 | python decl names surface in beets/library/library.py |  |  | 0.509 |
| walker |  | 1280 | 0 | python decl at beets/library/library.py:20 |  |  | 0.509 |
| walker |  | 1293 | 13 | python decl doc at beets/library/library.py:20 |  |  | 0.509 |
| walker |  | 1360 | 67 | [package] in pyproject.toml |  |  | 0.519 |
| ns | 1405 |  | 273 | `beets.dbcore` public re-exports | 2.4 |  | 0.463 |
| walker |  | 1466 | 106 | python decl names surface in beets/ui/commands/import_/__init__.py |  |  | 0.463 |
| walker |  | 1466 | 0 | python decl at beets/ui/commands/import_/__init__.py:14 |  |  | 0.463 |
| walker |  | 1466 | 0 | python decl at beets/ui/commands/import_/__init__.py:34 |  |  | 0.463 |
| walker |  | 1466 | 0 | python decl at beets/ui/commands/import_/__init__.py:49 |  |  | 0.463 |
| walker |  | 1466 | 0 | python decl at beets/ui/commands/import_/__init__.py:81 |  |  | 0.463 |
| walker |  | 1466 | 0 | python decl at beets/ui/commands/import_/__init__.py:134 |  |  | 0.463 |
| walker |  | 1482 | 16 | python decl doc at beets/ui/commands/import_/__init__.py:34 |  |  | 0.463 |
| walker |  | 1508 | 26 | python decl at beets/ui/commands/import_/__init__.py:160 |  |  | 0.463 |
| walker |  | 1538 | 30 | python decl doc at beets/ui/commands/import_/__init__.py:49 |  |  | 0.463 |
| walker |  | 1569 | 31 | python decl doc at beets/ui/commands/import_/__init__.py:14 |  |  | 0.463 |
| walker |  | 1576 | 7 | python imports #12 in beets/util/__init__.py |  |  | 0.463 |
| walker |  | 1653 | 77 | listing of 'docs' |  |  | 0.466 |
| walker |  | 1675 | 22 | listing of 'docs/api' |  |  | 0.467 |
| walker |  | 1701 | 26 | listing of 'docs/guides' |  |  | 0.469 |
| ns | 1722 |  | 317 | `beets.library` public re-exports | 2.5 |  | 0.459 |
| walker |  | 1727 | 26 | listing of 'docs/reference' |  |  | 0.462 |
| walker |  | 1755 | 28 | listing of 'docs/dev' |  |  | 0.465 |
| walker |  | 1762 | 7 | python imports in beets/util/units.py |  |  | 0.465 |
| walker |  | 1778 | 16 | python decl names surface in beets/library/fields.py |  |  | 0.465 |
| walker |  | 1785 | 7 | python imports #13 in beets/util/__init__.py |  |  | 0.465 |
| walker |  | 1789 | 4 | listing of 'docs/_templates' |  |  | 0.465 |
| walker |  | 1810 | 21 | listing of 'docs/_templates/autosummary' |  |  | 0.465 |
| walker |  | 1814 | 4 | listing of 'docs/extensions' |  |  | 0.465 |
| walker |  | 1883 | 69 | python decl at beets/library/__init__.py:8 |  |  | 0.486 |
| walker |  | 1898 | 15 | listing of 'beets/test' |  |  | 0.486 |
| walker |  | 1916 | 18 | python decl names surface in beets/util/hidden.py |  |  | 0.486 |
| walker |  | 1916 | 0 | python decl at beets/util/hidden.py:25 |  |  | 0.486 |
| walker |  | 1940 | 24 | listing of 'docs/dev/plugins' |  |  | 0.489 |
| ns | 2088 |  | 366 | `beets.autotag` public re-exports | 2.6 |  | 0.448 |
| ns | 2306 |  | 218 | `beets.importer` public re-exports | 2.7 |  | 0.424 |
| walker |  | 2490 | 550 | README headline in README.rst |  |  | 0.600 |
| ns | 2509 |  | 203 | Docs tree + reference + dev-doc table-of-contents | 2.8 |  | 0.634 |
| ns | 2681 |  | 172 | `UserError` + `Subcommand` class header | 3.1 |  | 0.618 |
| walker |  | 2865 | 375 | python imports in beets/autotag/__init__.py |  |  | 0.639 |
| ns | 2928 |  | 247 | Top-level fns in `beets/ui/__init__.py` (signature heads only) | 3.2 |  | 0.619 |
| walker |  | 3243 | 378 | python imports in beets/importer/__init__.py |  |  | 0.652 |
| walker |  | 3256 | 13 | python decl names surface in beets/ui/commands/help.py |  |  | 0.652 |
| walker |  | 3256 | 0 | python decl at beets/ui/commands/help.py:6 |  |  | 0.652 |
| ns | 3376 |  | 448 | `main()` top-level exception handlers | 3.3 | 3.2 | 0.619 |
| ns | 3470 |  | 94 | `_raw_main()` global-option setup | 3.4 | 3.2 | 0.614 |
| walker |  | 3650 | 394 | python imports in beets/ui/commands/__init__.py |  |  | 0.615 |
| walker |  | 3660 | 10 | python imports in beets/library/exceptions.py |  |  | 0.615 |
| walker |  | 3669 | 9 | python decl names surface #1 in beets/util/color.py |  |  | 0.615 |
| walker |  | 3692 | 23 | python decl names surface in beets/importer/state.py |  |  | 0.615 |
| walker |  | 3698 | 6 | python decl at beets/importer/state.py:34 |  |  | 0.615 |
| walker |  | 3713 | 15 | python decl doc at beets/importer/state.py:34 |  |  | 0.615 |
| ns | 3728 |  | 258 | `_raw_main()` parse + dispatch tail | 3.5 | 3.4 | 0.595 |
| walker |  | 3737 | 24 | python decl names surface in beets/util/m3u.py |  |  | 0.595 |
| walker |  | 3737 | 0 | python decl at beets/util/m3u.py:22 |  |  | 0.595 |
| walker |  | 3737 | 0 | python decl at beets/util/m3u.py:28 |  |  | 0.595 |
| walker |  | 3744 | 7 | python class body at beets/util/m3u.py:22 |  |  | 0.595 |
| walker |  | 3761 | 17 | python decl doc at beets/util/m3u.py:22 |  |  | 0.595 |
| walker |  | 3779 | 18 | python decl doc at beets/util/m3u.py:28 |  |  | 0.595 |
| walker |  | 3788 | 9 | python imports #14 in beets/util/__init__.py |  |  | 0.595 |
| walker |  | 3816 | 28 | python method sigs in beets/ui/commands/help.py |  |  | 0.595 |
| walker |  | 3816 | 0 | python method at beets/ui/commands/help.py:7 |  |  | 0.595 |
| walker |  | 3816 | 0 | python method at beets/ui/commands/help.py:14 |  |  | 0.595 |
| ns | 3939 |  | 211 | `_setup()` body: load plugins, build commands, open library | 3.6 | 3.2 | 0.579 |
| walker |  | 4292 | 476 | python imports in beets/ui/__init__.py |  |  | 0.579 |
| walker |  | 4304 | 12 | python imports in beets/library/fields.py |  |  | 0.579 |
| ns | 4347 |  | 408 | `default_commands` list (built-in subcommands) | 3.7 |  | 0.559 |
| ns | 4535 |  | 188 | Built-in command registrations (one `Subcommand` line per cmd) | 3.8 |  | 0.551 |
| walker |  | 4795 | 491 | python imports in beets/dbcore/__init__.py |  |  | 0.602 |
| ns | 4796 |  | 261 | Smallest command as concrete template (`list`) | 3.9 | 3.8 | 0.584 |
| walker |  | 4817 | 22 | python imports in beets/context.py |  |  | 0.584 |
| walker |  | 4883 | 66 | python decl names surface in beets/context.py |  |  | 0.584 |
| walker |  | 4883 | 0 | python decl at beets/context.py:8 |  |  | 0.584 |
| walker |  | 4883 | 0 | python decl at beets/context.py:13 |  |  | 0.584 |
| walker |  | 4892 | 9 | python decl at beets/context.py:18 |  |  | 0.584 |
| walker |  | 4903 | 11 | python decl doc at beets/context.py:8 |  |  | 0.584 |
| walker |  | 4914 | 11 | python decl doc at beets/context.py:13 |  |  | 0.584 |
| walker |  | 4926 | 12 | python decl body at beets/context.py:8 body 10 |  |  | 0.584 |
| walker |  | 4938 | 12 | python decl body at beets/context.py:13 body 15 |  |  | 0.584 |
| walker |  | 4956 | 18 | python decl doc at beets/context.py:18 |  |  | 0.584 |
| ns | 4982 |  | 186 | `Library` class header + `_models` + `_migrations` | 4.1 |  | 0.576 |
| walker |  | 5121 | 165 | python decl names surface in beets/ui/__init__.py |  |  | 0.579 |
| walker |  | 5121 | 0 | python decl at beets/ui/__init__.py:71 |  |  | 0.579 |
| walker |  | 5121 | 0 | python decl at beets/ui/__init__.py:80 |  |  | 0.579 |
| walker |  | 5121 | 0 | python decl at beets/ui/__init__.py:85 |  |  | 0.579 |
| walker |  | 5121 | 0 | python decl at beets/ui/__init__.py:90 |  |  | 0.579 |
| walker |  | 5121 | 0 | python decl at beets/ui/__init__.py:111 |  |  | 0.579 |
| walker |  | 5121 | 0 | python decl at beets/ui/__init__.py:122 |  |  | 0.579 |
| walker |  | 5121 | 0 | python decl at beets/ui/__init__.py:150 |  |  | 0.579 |
| walker |  | 5121 | 0 | python decl at beets/ui/__init__.py:160 |  |  | 0.579 |
| walker |  | 5121 | 0 | python decl at beets/ui/__init__.py:167 |  |  | 0.579 |
| walker |  | 5121 | 0 | python decl at beets/ui/__init__.py:187 |  |  | 0.579 |
| walker |  | 5121 | 0 | python decl at beets/ui/__init__.py:383 |  |  | 0.579 |
| ns | 5125 |  | 143 | `Library` query/fetch method headers | 4.2 | 4.1 | 0.573 |
| walker |  | 5156 | 35 | python decl doc at beets/ui/__init__.py:71 |  |  | 0.575 |
| walker |  | 5178 | 22 | python decl body at beets/ui/__init__.py:160 body 164 |  |  | 0.575 |
| walker |  | 5240 | 62 | python decl at beets/ui/__init__.py:207 |  |  | 0.575 |
| walker |  | 5277 | 37 | python decl doc at beets/ui/__init__.py:160 |  |  | 0.575 |
| walker |  | 5289 | 12 | python decl body at beets/ui/__init__.py:80 body 82 |  |  | 0.575 |
| walker |  | 5301 | 12 | python decl body at beets/ui/__init__.py:85 body 87 |  |  | 0.575 |
| walker |  | 5319 | 18 | python decl doc at beets/ui/__init__.py:150 |  |  | 0.575 |
| ns | 5328 |  | 203 | `LibModel` + `Item` + `Album` class headers | 4.3 |  | 0.563 |
| walker |  | 5338 | 19 | python decl doc at beets/ui/__init__.py:80 |  |  | 0.563 |
| walker |  | 5357 | 19 | python decl doc at beets/ui/__init__.py:85 |  |  | 0.563 |
| walker |  | 5410 | 53 | python decl doc at beets/ui/__init__.py:383 |  |  | 0.563 |
| ns | 5412 |  | 84 | `_search_fields` for Item + Album (default query targets) | 4.4 | 4.3 | 0.557 |
| walker |  | 5419 | 9 | python decl body at beets/ui/__init__.py:111 body 119 |  |  | 0.557 |
| walker |  | 5490 | 71 | python decl doc at beets/ui/__init__.py:187 |  |  | 0.557 |
| walker |  | 5564 | 74 | python decl doc at beets/ui/__init__.py:111 |  |  | 0.557 |
| ns | 5614 |  | 202 | `TYPE_BY_FIELD` lede + first dozen fields | 4.5 |  | 0.549 |
| walker |  | 5646 | 82 | python decl doc at beets/ui/__init__.py:122 |  |  | 0.549 |
| walker |  | 5700 | 54 | python decl body at beets/ui/__init__.py:167 body 177 |  |  | 0.549 |
| walker |  | 5759 | 59 | python decl body at beets/__init__.py:26 body 28 |  |  | 0.549 |
| walker |  | 5868 | 109 | python decl at beets/ui/commands/__init__.py:50 |  |  | 0.565 |
| walker |  | 5981 | 113 | python decl doc at beets/ui/__init__.py:167 |  |  | 0.565 |
| ns | 5986 |  | 372 | `parse_query_parts` + `parse_query_string` (library-level query entry) | 4.6 |  | 0.549 |
| walker |  | 6161 | 180 | python decl names surface in beets/util/__init__.py |  |  | 0.549 |
| walker |  | 6161 | 0 | python decl at beets/util/__init__.py:74 |  |  | 0.549 |
| walker |  | 6161 | 0 | python decl at beets/util/__init__.py:130 |  |  | 0.549 |
| walker |  | 6161 | 0 | python decl at beets/util/__init__.py:158 |  |  | 0.549 |
| walker |  | 6161 | 0 | python decl at beets/util/__init__.py:169 |  |  | 0.549 |
| walker |  | 6161 | 0 | python decl at beets/util/__init__.py:175 |  |  | 0.549 |
| walker |  | 6161 | 0 | python decl at beets/util/__init__.py:184 |  |  | 0.549 |
| walker |  | 6161 | 0 | python decl at beets/util/__init__.py:273 |  |  | 0.549 |
| walker |  | 6161 | 0 | python decl at beets/util/__init__.py:280 |  |  | 0.549 |
| walker |  | 6161 | 0 | python decl at beets/util/__init__.py:294 |  |  | 0.549 |
| walker |  | 6161 | 0 | python decl at beets/util/__init__.py:356 |  |  | 0.549 |
| walker |  | 6181 | 20 | python class body at beets/util/__init__.py:74 |  |  | 0.549 |
| walker |  | 6199 | 18 | python decl doc at beets/util/__init__.py:158 |  |  | 0.549 |
| walker |  | 6224 | 25 | python class body at beets/util/__init__.py:169 |  |  | 0.549 |
| walker |  | 6239 | 15 | python decl body at beets/util/__init__.py:273 body 277 |  |  | 0.549 |
| walker |  | 6288 | 49 | python decl at beets/util/__init__.py:309 |  |  | 0.549 |
| walker |  | 6317 | 29 | python decl doc at beets/util/__init__.py:273 |  |  | 0.549 |
| ns | 6325 |  | 339 | `LibModel` + `Item` + `Album` method names (locations) | 4.7 | 4.3 | 0.537 |
| walker |  | 6347 | 30 | python decl doc at beets/util/__init__.py:175 |  |  | 0.537 |
| walker |  | 6379 | 32 | python decl doc at beets/util/__init__.py:280 |  |  | 0.537 |
| walker |  | 6419 | 40 | python decl doc at beets/util/__init__.py:294 |  |  | 0.537 |
| walker |  | 6487 | 68 | python decl at beets/util/__init__.py:208 |  |  | 0.537 |
| walker |  | 6553 | 66 | python class body at beets/util/__init__.py:158 |  |  | 0.537 |
| ns | 6560 |  | 235 | `DefaultTemplateFunctions` header + `tmpl_*` method locations | 4.8 |  | 0.530 |
| walker |  | 6607 | 54 | python decl doc at beets/util/__init__.py:130 |  |  | 0.530 |
| walker |  | 6615 | 8 | python decl body at beets/util/__init__.py:294 body 306 |  |  | 0.530 |
| ns | 6673 |  | 113 | Library migration class headers (full chain) | 4.9 |  | 0.526 |
| walker |  | 6687 | 72 | python decl doc at beets/util/__init__.py:356 |  |  | 0.526 |
| walker |  | 6770 | 83 | python decl doc at beets/util/__init__.py:208 |  |  | 0.526 |
| walker |  | 6856 | 86 | python decl doc at beets/util/__init__.py:184 |  |  | 0.526 |
| ns | 6903 |  | 230 | `Model` + `Database` class headers | 5.1 |  | 0.519 |
| ns | 6999 |  | 96 | Top-level classes in `beets/dbcore/db.py` (locations) | 5.2 |  | 0.515 |
| walker |  | 7065 | 209 | python method sigs in beets/util/__init__.py |  |  | 0.515 |
| walker |  | 7065 | 0 | python method at beets/util/__init__.py:90 |  |  | 0.515 |
| walker |  | 7065 | 0 | python method at beets/util/__init__.py:96 |  |  | 0.515 |
| walker |  | 7065 | 0 | python method at beets/util/__init__.py:104 |  |  | 0.515 |
| walker |  | 7065 | 0 | python method at beets/util/__init__.py:115 |  |  | 0.515 |
| walker |  | 7065 | 0 | python method at beets/util/__init__.py:121 |  |  | 0.515 |
| walker |  | 7065 | 0 | python method at beets/util/__init__.py:136 |  |  | 0.515 |
| walker |  | 7065 | 0 | python method at beets/util/__init__.py:1094 |  |  | 0.515 |
| walker |  | 7065 | 0 | python method at beets/util/__init__.py:1098 |  |  | 0.515 |
| walker |  | 7065 | 0 | python method at beets/util/__init__.py:1102 |  |  | 0.515 |
| walker |  | 7065 | 0 | python method at beets/util/__init__.py:1145 |  |  | 0.515 |
| walker |  | 7080 | 15 | python method at beets/util/__init__.py:140 |  |  | 0.515 |
| walker |  | 7094 | 14 | python method doc at beets/util/__init__.py:1094 |  |  | 0.515 |
| walker |  | 7104 | 10 | python method body at beets/util/__init__.py:115 body 119 |  |  | 0.515 |
| walker |  | 7119 | 15 | python method doc at beets/util/__init__.py:1098 |  |  | 0.515 |
| walker |  | 7130 | 11 | python method body at beets/util/__init__.py:1098 body 1100 |  |  | 0.515 |
| walker |  | 7147 | 17 | python method doc at beets/util/__init__.py:1102 |  |  | 0.515 |
| walker |  | 7158 | 11 | python method doc at beets/util/__init__.py:104 |  |  | 0.515 |
| walker |  | 7175 | 17 | python method body at beets/util/__init__.py:1094 body 1096 |  |  | 0.515 |
| walker |  | 7203 | 28 | python method doc at beets/util/__init__.py:115 |  |  | 0.515 |
| walker |  | 7220 | 17 | python method doc at beets/util/__init__.py:96 |  |  | 0.515 |
| walker |  | 7259 | 39 | python method doc at beets/util/__init__.py:121 |  |  | 0.515 |
| ns | 7271 |  | 272 | `Query` hierarchy class headers (full chain) | 5.3 |  | 0.505 |
| walker |  | 7282 | 23 | python method body at beets/util/__init__.py:136 body 137 |  |  | 0.505 |
| walker |  | 7301 | 19 | python decl names surface in beets/ui/commands/utils.py |  |  | 0.505 |
| walker |  | 7301 | 0 | python decl at beets/ui/commands/utils.py:6 |  |  | 0.505 |
| ns | 7346 |  | 75 | `Sort` class hierarchy (locations) | 5.4 |  | 0.502 |
| walker |  | 7419 | 118 | python decl doc at beets/util/__init__.py:309 |  |  | 0.502 |
| walker |  | 7470 | 51 | python decl doc at beets/ui/commands/import_/__init__.py:134 |  |  | 0.502 |
| walker |  | 7528 | 58 | python method sigs in beets/util/m3u.py |  |  | 0.502 |
| walker |  | 7528 | 0 | python method at beets/util/m3u.py:31 |  |  | 0.502 |
| walker |  | 7528 | 0 | python method at beets/util/m3u.py:43 |  |  | 0.502 |
| walker |  | 7528 | 0 | python method at beets/util/m3u.py:63 |  |  | 0.502 |
| walker |  | 7528 | 0 | python method at beets/util/m3u.py:77 |  |  | 0.502 |
| walker |  | 7546 | 18 | python method doc at beets/util/m3u.py:43 |  |  | 0.502 |
| ns | 7547 |  | 201 | `Type` hierarchy class headers | 5.5 |  | 0.495 |
| walker |  | 7578 | 32 | python decl doc at beets/util/hidden.py:25 |  |  | 0.495 |
| ns | 7601 |  | 54 | `queryparse` top-level fn locations | 5.6 |  | 0.493 |
| walker |  | 7610 | 32 | python decl names surface in beets/util/config.py |  |  | 0.493 |
| walker |  | 7610 | 0 | python decl at beets/util/config.py:78 |  |  | 0.493 |
| walker |  | 7625 | 15 | python method sigs in beets/util/config.py |  |  | 0.493 |
| walker |  | 7625 | 0 | python method at beets/util/config.py:79 |  |  | 0.493 |
| walker |  | 7648 | 23 | python decl at beets/util/config.py:9 |  |  | 0.493 |
| walker |  | 7701 | 53 | python decl at beets/util/config.py:29 |  |  | 0.493 |
| ns | 7733 |  | 132 | `PARSE_QUERY_PART_REGEX` — the query-string syntax | 5.7 | 5.6 | 0.489 |
| walker |  | 7767 | 66 | python decl names surface #2 in beets/ui/__init__.py |  |  | 0.491 |
| walker |  | 7767 | 0 | python decl at beets/ui/__init__.py:905 |  |  | 0.491 |
| walker |  | 7767 | 0 | python decl at beets/ui/__init__.py:996 |  |  | 0.491 |
| walker |  | 7796 | 29 | python decl at beets/ui/__init__.py:65 |  |  | 0.491 |
| walker |  | 7833 | 37 | python decl doc at beets/ui/__init__.py:996 |  |  | 0.492 |
| walker |  | 7862 | 29 | python decl doc at beets/ui/__init__.py:905 |  |  | 0.492 |
| ns | 7864 |  | 131 | Autotag entry points: `tag_album` + `tag_item` signatures | 6.1 |  | 0.488 |
| walker |  | 7917 | 55 | python decl doc at beets/ui/__init__.py:90 |  |  | 0.488 |
| ns | 7963 |  | 99 | `Recommendation` + `Proposal` + `Match`/`AlbumMatch`/`TrackMatch` + `Distance` + `Info`/`AlbumInfo`/`TrackInfo` | 6.2 |  | 0.484 |
| walker |  | 8072 | 155 | python decl doc at beets/util/__init__.py:74 |  |  | 0.484 |
| walker |  | 8111 | 39 | listing of '.github' |  |  | 0.484 |
| walker |  | 8142 | 31 | listing of '.github/workflows' |  |  | 0.484 |
| walker |  | 8153 | 11 | python imports #15 in beets/util/__init__.py |  |  | 0.484 |
| walker |  | 8163 | 10 | python imports #16 in beets/util/__init__.py |  |  | 0.484 |
| walker |  | 8172 | 9 | python imports #17 in beets/util/__init__.py |  |  | 0.484 |
| ns | 8174 |  | 211 | `Action` enum + `ImportTask` class header | 7.1 |  | 0.478 |
| walker |  | 8291 | 119 | python decl names surface in beets/util/deprecation.py |  |  | 0.478 |
| walker |  | 8291 | 0 | python decl at beets/util/deprecation.py:30 |  |  | 0.478 |
| walker |  | 8307 | 16 | python decl at beets/util/deprecation.py:16 |  |  | 0.478 |
| walker |  | 8333 | 26 | python decl at beets/util/deprecation.py:78 |  |  | 0.478 |
| walker |  | 8360 | 27 | python decl at beets/util/deprecation.py:39 |  |  | 0.478 |
| walker |  | 8390 | 30 | python decl at beets/util/deprecation.py:59 |  |  | 0.478 |
| walker |  | 8421 | 31 | python decl at beets/util/deprecation.py:45 |  |  | 0.478 |
| walker |  | 8435 | 14 | python decl body at beets/util/deprecation.py:39 body 42 |  |  | 0.478 |
| ns | 8488 |  | 314 | `ImportSession.run()` — pipeline assembly | 7.2 |  | 0.469 |
| walker |  | 8506 | 71 | python decl at beets/util/deprecation.py:19 |  |  | 0.469 |
| walker |  | 8546 | 40 | python decl body at beets/util/deprecation.py:45 body 54 |  |  | 0.469 |
| walker |  | 8584 | 38 | python decl names surface in beets/library/exceptions.py |  |  | 0.469 |
| walker |  | 8584 | 0 | python decl at beets/library/exceptions.py:4 |  |  | 0.469 |
| walker |  | 8584 | 0 | python decl at beets/library/exceptions.py:27 |  |  | 0.469 |
| walker |  | 8584 | 0 | python decl at beets/library/exceptions.py:34 |  |  | 0.469 |
| walker |  | 8604 | 20 | python decl doc at beets/library/exceptions.py:27 |  |  | 0.469 |
| walker |  | 8624 | 20 | python decl doc at beets/library/exceptions.py:34 |  |  | 0.469 |
| ns | 8655 |  | 167 | Importer stage functions (locations) | 7.3 |  | 0.465 |
| walker |  | 8680 | 56 | python method sigs in beets/library/exceptions.py |  |  | 0.465 |
| walker |  | 8680 | 0 | python method at beets/library/exceptions.py:11 |  |  | 0.465 |
| walker |  | 8680 | 0 | python method at beets/library/exceptions.py:19 |  |  | 0.465 |
| walker |  | 8680 | 0 | python method at beets/library/exceptions.py:30 |  |  | 0.465 |
| walker |  | 8680 | 0 | python method at beets/library/exceptions.py:37 |  |  | 0.465 |
| walker |  | 8692 | 12 | python method body at beets/library/exceptions.py:30 body 31 |  |  | 0.465 |
| walker |  | 8704 | 12 | python method body at beets/library/exceptions.py:37 body 38 |  |  | 0.465 |
| walker |  | 8782 | 78 | python decl body at beets/ui/commands/__init__.py:36 body 38 |  |  | 0.478 |
| walker |  | 8824 | 42 | python method body at beets/util/__init__.py:90 body 91 |  |  | 0.478 |
| walker |  | 8907 | 83 | README.rst section #0 |  |  | 0.473 |
| ns | 8907 |  | 252 | `BeetsPlugin` class header + registration-method locations | 8.1 |  | 0.473 |
| walker |  | 8916 | 9 | python imports #18 in beets/util/__init__.py |  |  | 0.473 |
| walker |  | 8993 | 77 | python decl doc at beets/util/deprecation.py:45 |  |  | 0.473 |
| walker |  | 9061 | 68 | python class body at beets/importer/state.py:34 |  |  | 0.473 |
| walker |  | 9114 | 53 | python decl doc at beets/library/exceptions.py:4 |  |  | 0.473 |
| walker |  | 9159 | 45 | python decl names surface in beets/importer/session.py |  |  | 0.473 |
| walker |  | 9159 | 0 | python decl at beets/importer/session.py:42 |  |  | 0.473 |
| walker |  | 9159 | 0 | python decl at beets/importer/session.py:48 |  |  | 0.473 |
| walker |  | 9166 | 7 | python class body at beets/importer/session.py:42 |  |  | 0.473 |
| walker |  | 9180 | 14 | python decl doc at beets/importer/session.py:42 |  |  | 0.473 |
| walker |  | 9216 | 36 | python decl doc at beets/importer/session.py:48 |  |  | 0.463 |
| ns | 9216 |  | 309 | `EventType` literal — every event a listener can hook | 8.2 |  | 0.463 |
| walker |  | 9235 | 19 | python decl body at beets/ui/__init__.py:111 body 118 |  |  | 0.463 |
| walker |  | 9257 | 22 | python imports in beets/util/config.py |  |  | 0.463 |
| walker |  | 9356 | 99 | python class body at beets/importer/session.py:48 |  |  | 0.463 |
| ns | 9425 |  | 209 | `load_plugins`, `find_plugins`, dispatch entry points (signatures) | 8.3 |  | 0.459 |
| walker |  | 9442 | 86 | python decl doc at beets/util/deprecation.py:59 |  |  | 0.459 |
| walker |  | 9488 | 46 | python decl names surface in beets/util/id_extractors.py |  |  | 0.459 |
| walker |  | 9488 | 0 | python decl at beets/util/id_extractors.py:50 |  |  | 0.459 |
| walker |  | 9579 | 91 | python decl body at beets/util/__init__.py:280 body 284 |  |  | 0.459 |
| ns | 9580 |  | 155 | `MetadataSourcePlugin` + entry-fn locations | 8.4 |  | 0.457 |
| walker |  | 9620 | 41 | listing of 'docs/dev/plugins/other' |  |  | 0.457 |
| walker |  | 9779 | 159 | python decl names surface in beets/logging.py |  |  | 0.457 |
| walker |  | 9779 | 0 | python decl at beets/logging.py:81 |  |  | 0.457 |
| walker |  | 9779 | 0 | python decl at beets/logging.py:105 |  |  | 0.457 |
| walker |  | 9779 | 0 | python decl at beets/logging.py:160 |  |  | 0.457 |
| walker |  | 9779 | 0 | python decl at beets/logging.py:188 |  |  | 0.457 |
| walker |  | 9779 | 0 | python decl at beets/logging.py:212 |  |  | 0.457 |
| walker |  | 9787 | 8 | python decl at beets/logging.py:208 |  |  | 0.457 |
| walker |  | 9795 | 8 | python decl at beets/logging.py:210 |  |  | 0.457 |
| walker |  | 9807 | 12 | python decl doc at beets/logging.py:188 |  |  | 0.457 |
| walker |  | 9826 | 19 | python decl doc at beets/logging.py:160 |  |  | 0.457 |
| walker |  | 9843 | 17 | python decl body at beets/logging.py:208 body 209 |  |  | 0.457 |
| walker |  | 9860 | 17 | python decl body at beets/logging.py:210 body 211 |  |  | 0.457 |
| ns | 9963 |  | 383 | `beetsplug/` listing — every shipped plugin | 9.1 |  | 0.434 |
