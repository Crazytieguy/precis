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
| walker |  | 3683 | 23 | python decl names surface in beets/importer/state.py |  |  | 0.615 |
| walker |  | 3689 | 6 | python decl at beets/importer/state.py:34 |  |  | 0.615 |
| walker |  | 3713 | 24 | python decl names surface in beets/util/m3u.py |  |  | 0.615 |
| walker |  | 3713 | 0 | python decl at beets/util/m3u.py:22 |  |  | 0.615 |
| walker |  | 3713 | 0 | python decl at beets/util/m3u.py:28 |  |  | 0.615 |
| walker |  | 3720 | 7 | python class body at beets/util/m3u.py:22 |  |  | 0.615 |
| ns | 3728 |  | 258 | `_raw_main()` parse + dispatch tail | 3.5 | 3.4 | 0.595 |
| walker |  | 3737 | 17 | python decl doc at beets/util/m3u.py:22 |  |  | 0.595 |
| walker |  | 3755 | 18 | python decl doc at beets/util/m3u.py:28 |  |  | 0.595 |
| walker |  | 3764 | 9 | python imports #14 in beets/util/__init__.py |  |  | 0.595 |
| walker |  | 3792 | 28 | python method sigs in beets/ui/commands/help.py |  |  | 0.595 |
| walker |  | 3792 | 0 | python method at beets/ui/commands/help.py:7 |  |  | 0.595 |
| walker |  | 3792 | 0 | python method at beets/ui/commands/help.py:14 |  |  | 0.595 |
| ns | 3939 |  | 211 | `_setup()` body: load plugins, build commands, open library | 3.6 | 3.2 | 0.579 |
| walker |  | 4268 | 476 | python imports in beets/ui/__init__.py |  |  | 0.579 |
| ns | 4347 |  | 408 | `default_commands` list (built-in subcommands) | 3.7 |  | 0.559 |
| walker |  | 4420 | 152 | python decl names surface in beets/util/__init__.py |  |  | 0.559 |
| walker |  | 4420 | 0 | python decl at beets/util/__init__.py:74 |  |  | 0.559 |
| walker |  | 4420 | 0 | python decl at beets/util/__init__.py:130 |  |  | 0.559 |
| walker |  | 4420 | 0 | python decl at beets/util/__init__.py:158 |  |  | 0.559 |
| walker |  | 4420 | 0 | python decl at beets/util/__init__.py:169 |  |  | 0.559 |
| walker |  | 4420 | 0 | python decl at beets/util/__init__.py:175 |  |  | 0.559 |
| walker |  | 4440 | 20 | python class body at beets/util/__init__.py:74 |  |  | 0.559 |
| walker |  | 4458 | 18 | python decl doc at beets/util/__init__.py:158 |  |  | 0.559 |
| walker |  | 4483 | 25 | python class body at beets/util/__init__.py:169 |  |  | 0.559 |
| walker |  | 4513 | 30 | python decl doc at beets/util/__init__.py:175 |  |  | 0.559 |
| ns | 4535 |  | 188 | Built-in command registrations (one `Subcommand` line per cmd) | 3.8 |  | 0.551 |
| walker |  | 4579 | 66 | python class body at beets/util/__init__.py:158 |  |  | 0.551 |
| walker |  | 4633 | 54 | python decl doc at beets/util/__init__.py:130 |  |  | 0.551 |
| walker |  | 4738 | 105 | python method sigs in beets/util/__init__.py |  |  | 0.551 |
| walker |  | 4738 | 0 | python method at beets/util/__init__.py:90 |  |  | 0.551 |
| walker |  | 4738 | 0 | python method at beets/util/__init__.py:96 |  |  | 0.551 |
| walker |  | 4738 | 0 | python method at beets/util/__init__.py:104 |  |  | 0.551 |
| walker |  | 4738 | 0 | python method at beets/util/__init__.py:115 |  |  | 0.551 |
| walker |  | 4738 | 0 | python method at beets/util/__init__.py:121 |  |  | 0.551 |
| walker |  | 4738 | 0 | python method at beets/util/__init__.py:136 |  |  | 0.551 |
| walker |  | 4753 | 15 | python method at beets/util/__init__.py:140 |  |  | 0.551 |
| walker |  | 4763 | 10 | python method body at beets/util/__init__.py:115 body 119 |  |  | 0.551 |
| walker |  | 4774 | 11 | python method doc at beets/util/__init__.py:104 |  |  | 0.551 |
| ns | 4796 |  | 261 | Smallest command as concrete template (`list`) | 3.9 | 3.8 | 0.535 |
| walker |  | 4802 | 28 | python method doc at beets/util/__init__.py:115 |  |  | 0.535 |
| walker |  | 4819 | 17 | python method doc at beets/util/__init__.py:96 |  |  | 0.535 |
| walker |  | 4858 | 39 | python method doc at beets/util/__init__.py:121 |  |  | 0.535 |
| walker |  | 4881 | 23 | python method body at beets/util/__init__.py:136 body 137 |  |  | 0.535 |
| walker |  | 4893 | 12 | python imports in beets/library/fields.py |  |  | 0.535 |
| ns | 4982 |  | 186 | `Library` class header + `_models` + `_migrations` | 4.1 |  | 0.528 |
| ns | 5125 |  | 143 | `Library` query/fetch method headers | 4.2 | 4.1 | 0.522 |
| ns | 5328 |  | 203 | `LibModel` + `Item` + `Album` class headers | 4.3 |  | 0.511 |
| walker |  | 5384 | 491 | python imports in beets/dbcore/__init__.py |  |  | 0.558 |
| walker |  | 5406 | 22 | python imports in beets/context.py |  |  | 0.558 |
| ns | 5412 |  | 84 | `_search_fields` for Item + Album (default query targets) | 4.4 | 4.3 | 0.553 |
| walker |  | 5472 | 66 | python decl names surface in beets/context.py |  |  | 0.553 |
| walker |  | 5472 | 0 | python decl at beets/context.py:8 |  |  | 0.553 |
| walker |  | 5472 | 0 | python decl at beets/context.py:13 |  |  | 0.553 |
| walker |  | 5481 | 9 | python decl at beets/context.py:18 |  |  | 0.553 |
| walker |  | 5492 | 11 | python decl doc at beets/context.py:8 |  |  | 0.553 |
| walker |  | 5503 | 11 | python decl doc at beets/context.py:13 |  |  | 0.553 |
| walker |  | 5515 | 12 | python decl body at beets/context.py:8 body 10 |  |  | 0.553 |
| walker |  | 5527 | 12 | python decl body at beets/context.py:13 body 15 |  |  | 0.553 |
| walker |  | 5545 | 18 | python decl doc at beets/context.py:18 |  |  | 0.553 |
| ns | 5614 |  | 202 | `TYPE_BY_FIELD` lede + first dozen fields | 4.5 |  | 0.545 |
| walker |  | 5709 | 164 | python decl names surface in beets/ui/__init__.py |  |  | 0.547 |
| walker |  | 5709 | 0 | python decl at beets/ui/__init__.py:71 |  |  | 0.547 |
| walker |  | 5709 | 0 | python decl at beets/ui/__init__.py:80 |  |  | 0.547 |
| walker |  | 5709 | 0 | python decl at beets/ui/__init__.py:85 |  |  | 0.547 |
| walker |  | 5709 | 0 | python decl at beets/ui/__init__.py:90 |  |  | 0.547 |
| walker |  | 5709 | 0 | python decl at beets/ui/__init__.py:111 |  |  | 0.547 |
| walker |  | 5709 | 0 | python decl at beets/ui/__init__.py:122 |  |  | 0.547 |
| walker |  | 5709 | 0 | python decl at beets/ui/__init__.py:150 |  |  | 0.547 |
| walker |  | 5709 | 0 | python decl at beets/ui/__init__.py:160 |  |  | 0.547 |
| walker |  | 5709 | 0 | python decl at beets/ui/__init__.py:167 |  |  | 0.547 |
| walker |  | 5709 | 0 | python decl at beets/ui/__init__.py:187 |  |  | 0.547 |
| walker |  | 5738 | 29 | python decl at beets/ui/__init__.py:65 |  |  | 0.547 |
| walker |  | 5773 | 35 | python decl doc at beets/ui/__init__.py:71 |  |  | 0.548 |
| walker |  | 5795 | 22 | python decl body at beets/ui/__init__.py:160 body 164 |  |  | 0.548 |
| walker |  | 5832 | 37 | python decl doc at beets/ui/__init__.py:160 |  |  | 0.548 |
| walker |  | 5844 | 12 | python decl body at beets/ui/__init__.py:80 body 82 |  |  | 0.548 |
| walker |  | 5856 | 12 | python decl body at beets/ui/__init__.py:85 body 87 |  |  | 0.548 |
| walker |  | 5874 | 18 | python decl doc at beets/ui/__init__.py:150 |  |  | 0.548 |
| walker |  | 5893 | 19 | python decl doc at beets/ui/__init__.py:80 |  |  | 0.548 |
| walker |  | 5912 | 19 | python decl doc at beets/ui/__init__.py:85 |  |  | 0.548 |
| walker |  | 5921 | 9 | python decl body at beets/ui/__init__.py:111 body 119 |  |  | 0.548 |
| ns | 5986 |  | 372 | `parse_query_parts` + `parse_query_string` (library-level query entry) | 4.6 |  | 0.532 |
| walker |  | 5992 | 71 | python decl doc at beets/ui/__init__.py:187 |  |  | 0.532 |
| walker |  | 6066 | 74 | python decl doc at beets/ui/__init__.py:111 |  |  | 0.532 |
| walker |  | 6148 | 82 | python decl doc at beets/ui/__init__.py:122 |  |  | 0.532 |
| walker |  | 6202 | 54 | python decl body at beets/ui/__init__.py:167 body 177 |  |  | 0.532 |
| walker |  | 6261 | 59 | python decl body at beets/__init__.py:26 body 28 |  |  | 0.532 |
| ns | 6325 |  | 339 | `LibModel` + `Item` + `Album` method names (locations) | 4.7 | 4.3 | 0.521 |
| walker |  | 6370 | 109 | python decl at beets/ui/commands/__init__.py:50 |  |  | 0.536 |
| walker |  | 6483 | 113 | python decl doc at beets/ui/__init__.py:167 |  |  | 0.536 |
| walker |  | 6502 | 19 | python decl names surface in beets/ui/commands/utils.py |  |  | 0.536 |
| walker |  | 6502 | 0 | python decl at beets/ui/commands/utils.py:6 |  |  | 0.536 |
| walker |  | 6553 | 51 | python decl doc at beets/ui/commands/import_/__init__.py:134 |  |  | 0.536 |
| ns | 6560 |  | 235 | `DefaultTemplateFunctions` header + `tmpl_*` method locations | 4.8 |  | 0.529 |
| walker |  | 6611 | 58 | python method sigs in beets/util/m3u.py |  |  | 0.529 |
| walker |  | 6611 | 0 | python method at beets/util/m3u.py:31 |  |  | 0.529 |
| walker |  | 6611 | 0 | python method at beets/util/m3u.py:43 |  |  | 0.529 |
| walker |  | 6611 | 0 | python method at beets/util/m3u.py:63 |  |  | 0.529 |
| walker |  | 6611 | 0 | python method at beets/util/m3u.py:77 |  |  | 0.529 |
| walker |  | 6629 | 18 | python method doc at beets/util/m3u.py:43 |  |  | 0.529 |
| walker |  | 6661 | 32 | python decl doc at beets/util/hidden.py:25 |  |  | 0.529 |
| ns | 6673 |  | 113 | Library migration class headers (full chain) | 4.9 |  | 0.525 |
| walker |  | 6693 | 32 | python decl names surface in beets/util/config.py |  |  | 0.525 |
| walker |  | 6693 | 0 | python decl at beets/util/config.py:78 |  |  | 0.525 |
| walker |  | 6708 | 15 | python method sigs in beets/util/config.py |  |  | 0.525 |
| walker |  | 6708 | 0 | python method at beets/util/config.py:79 |  |  | 0.525 |
| walker |  | 6731 | 23 | python decl at beets/util/config.py:9 |  |  | 0.525 |
| walker |  | 6784 | 53 | python decl at beets/util/config.py:29 |  |  | 0.525 |
| walker |  | 6839 | 55 | python decl doc at beets/ui/__init__.py:90 |  |  | 0.525 |
| ns | 6903 |  | 230 | `Model` + `Database` class headers | 5.1 |  | 0.518 |
| walker |  | 6994 | 155 | python decl doc at beets/util/__init__.py:74 |  |  | 0.518 |
| ns | 6999 |  | 96 | Top-level classes in `beets/dbcore/db.py` (locations) | 5.2 |  | 0.514 |
| walker |  | 7033 | 39 | listing of '.github' |  |  | 0.514 |
| walker |  | 7064 | 31 | listing of '.github/workflows' |  |  | 0.514 |
| walker |  | 7075 | 11 | python imports #15 in beets/util/__init__.py |  |  | 0.514 |
| walker |  | 7085 | 10 | python imports #16 in beets/util/__init__.py |  |  | 0.514 |
| walker |  | 7094 | 9 | python imports #17 in beets/util/__init__.py |  |  | 0.514 |
| walker |  | 7132 | 38 | python decl names surface in beets/library/exceptions.py |  |  | 0.514 |
| walker |  | 7132 | 0 | python decl at beets/library/exceptions.py:4 |  |  | 0.514 |
| walker |  | 7132 | 0 | python decl at beets/library/exceptions.py:27 |  |  | 0.514 |
| walker |  | 7132 | 0 | python decl at beets/library/exceptions.py:34 |  |  | 0.514 |
| walker |  | 7152 | 20 | python decl doc at beets/library/exceptions.py:27 |  |  | 0.514 |
| walker |  | 7172 | 20 | python decl doc at beets/library/exceptions.py:34 |  |  | 0.514 |
| walker |  | 7228 | 56 | python method sigs in beets/library/exceptions.py |  |  | 0.514 |
| walker |  | 7228 | 0 | python method at beets/library/exceptions.py:11 |  |  | 0.514 |
| walker |  | 7228 | 0 | python method at beets/library/exceptions.py:19 |  |  | 0.514 |
| walker |  | 7228 | 0 | python method at beets/library/exceptions.py:30 |  |  | 0.514 |
| walker |  | 7228 | 0 | python method at beets/library/exceptions.py:37 |  |  | 0.514 |
| walker |  | 7240 | 12 | python method body at beets/library/exceptions.py:30 body 31 |  |  | 0.514 |
| walker |  | 7252 | 12 | python method body at beets/library/exceptions.py:37 body 38 |  |  | 0.514 |
| ns | 7271 |  | 272 | `Query` hierarchy class headers (full chain) | 5.3 |  | 0.504 |
| walker |  | 7329 | 77 | python decl names surface #2 in beets/ui/__init__.py |  |  | 0.507 |
| walker |  | 7329 | 0 | python decl at beets/ui/__init__.py:867 |  |  | 0.507 |
| walker |  | 7329 | 0 | python decl at beets/ui/__init__.py:879 |  |  | 0.507 |
| walker |  | 7329 | 0 | python decl at beets/ui/__init__.py:905 |  |  | 0.507 |
| walker |  | 7329 | 0 | python decl at beets/ui/__init__.py:996 |  |  | 0.507 |
| walker |  | 7342 | 13 | python decl doc at beets/ui/__init__.py:879 |  |  | 0.507 |
| ns | 7346 |  | 75 | `Sort` class hierarchy (locations) | 5.4 |  | 0.504 |
| walker |  | 7379 | 37 | python decl doc at beets/ui/__init__.py:996 |  |  | 0.505 |
| walker |  | 7408 | 29 | python decl doc at beets/ui/__init__.py:905 |  |  | 0.505 |
| walker |  | 7486 | 78 | python decl body at beets/ui/commands/__init__.py:36 body 38 |  |  | 0.519 |
| walker |  | 7528 | 42 | python method body at beets/util/__init__.py:90 body 91 |  |  | 0.519 |
| ns | 7547 |  | 201 | `Type` hierarchy class headers | 5.5 |  | 0.512 |
| ns | 7601 |  | 54 | `queryparse` top-level fn locations | 5.6 |  | 0.509 |
| walker |  | 7611 | 83 | README.rst section #0 |  |  | 0.509 |
| walker |  | 7620 | 9 | python imports #18 in beets/util/__init__.py |  |  | 0.509 |
| walker |  | 7688 | 68 | python class body at beets/importer/state.py:34 |  |  | 0.509 |
| ns | 7733 |  | 132 | `PARSE_QUERY_PART_REGEX` — the query-string syntax | 5.7 | 5.6 | 0.505 |
| walker |  | 7741 | 53 | python decl doc at beets/library/exceptions.py:4 |  |  | 0.505 |
| walker |  | 7759 | 18 | python decl names surface #1 in beets/util/color.py |  |  | 0.505 |
| walker |  | 7759 | 0 | python decl at beets/util/color.py:208 |  |  | 0.505 |
| ns | 7864 |  | 131 | Autotag entry points: `tag_album` + `tag_item` signatures | 6.1 |  | 0.501 |
| walker |  | 7938 | 179 | python decl names surface in beets/logging.py |  |  | 0.501 |
| walker |  | 7938 | 0 | python decl at beets/logging.py:81 |  |  | 0.501 |
| walker |  | 7938 | 0 | python decl at beets/logging.py:105 |  |  | 0.501 |
| walker |  | 7938 | 0 | python decl at beets/logging.py:160 |  |  | 0.501 |
| walker |  | 7938 | 0 | python decl at beets/logging.py:188 |  |  | 0.501 |
| walker |  | 7938 | 0 | python decl at beets/logging.py:208 |  |  | 0.501 |
| walker |  | 7938 | 0 | python decl at beets/logging.py:210 |  |  | 0.501 |
| walker |  | 7938 | 0 | python decl at beets/logging.py:212 |  |  | 0.501 |
| walker |  | 7950 | 12 | python decl doc at beets/logging.py:188 |  |  | 0.501 |
| ns | 7963 |  | 99 | `Recommendation` + `Proposal` + `Match`/`AlbumMatch`/`TrackMatch` + `Distance` + `Info`/`AlbumInfo`/`TrackInfo` | 6.2 |  | 0.497 |
| walker |  | 7969 | 19 | python decl doc at beets/logging.py:160 |  |  | 0.497 |
| walker |  | 7984 | 15 | python decl body at beets/logging.py:208 body 209 |  |  | 0.497 |
| walker |  | 7999 | 15 | python decl body at beets/logging.py:210 body 211 |  |  | 0.497 |
| walker |  | 8165 | 166 | python class body at beets/logging.py:105 |  |  | 0.497 |
| ns | 8174 |  | 211 | `Action` enum + `ImportTask` class header | 7.1 |  | 0.491 |
| walker |  | 8261 | 96 | python method sigs in beets/logging.py |  |  | 0.491 |
| walker |  | 8261 | 0 | python method at beets/logging.py:163 |  |  | 0.491 |
| walker |  | 8261 | 0 | python method at beets/logging.py:180 |  |  | 0.491 |
| walker |  | 8261 | 0 | python method at beets/logging.py:191 |  |  | 0.491 |
| walker |  | 8268 | 7 | python method at beets/logging.py:168 |  |  | 0.491 |
| walker |  | 8277 | 9 | python method at beets/logging.py:176 |  |  | 0.491 |
| walker |  | 8290 | 13 | python method body at beets/logging.py:176 body 178 |  |  | 0.491 |
| walker |  | 8321 | 31 | python method doc at beets/logging.py:180 |  |  | 0.491 |
| walker |  | 8364 | 43 | python decl body at beets/logging.py:212 body 213 |  |  | 0.491 |
| walker |  | 8409 | 45 | python decl names surface in beets/importer/session.py |  |  | 0.491 |
| walker |  | 8409 | 0 | python decl at beets/importer/session.py:42 |  |  | 0.491 |
| walker |  | 8409 | 0 | python decl at beets/importer/session.py:48 |  |  | 0.491 |
| walker |  | 8416 | 7 | python class body at beets/importer/session.py:42 |  |  | 0.491 |
| walker |  | 8430 | 14 | python decl doc at beets/importer/session.py:42 |  |  | 0.491 |
| walker |  | 8466 | 36 | python decl doc at beets/importer/session.py:48 |  |  | 0.491 |
| walker |  | 8485 | 19 | python decl body at beets/ui/__init__.py:111 body 118 |  |  | 0.491 |
| ns | 8488 |  | 314 | `ImportSession.run()` — pipeline assembly | 7.2 |  | 0.481 |
| walker |  | 8507 | 22 | python imports in beets/util/config.py |  |  | 0.481 |
| walker |  | 8606 | 99 | python class body at beets/importer/session.py:48 |  |  | 0.481 |
| walker |  | 8652 | 46 | python decl names surface in beets/util/id_extractors.py |  |  | 0.481 |
| walker |  | 8652 | 0 | python decl at beets/util/id_extractors.py:50 |  |  | 0.481 |
| ns | 8655 |  | 167 | Importer stage functions (locations) | 7.3 |  | 0.477 |
| walker |  | 8803 | 151 | python decl names surface #1 in beets/ui/__init__.py |  |  | 0.492 |
| walker |  | 8803 | 0 | python decl at beets/ui/__init__.py:383 |  |  | 0.492 |
| walker |  | 8803 | 0 | python decl at beets/ui/__init__.py:395 |  |  | 0.492 |
| walker |  | 8803 | 0 | python decl at beets/ui/__init__.py:433 |  |  | 0.492 |
| walker |  | 8803 | 0 | python decl at beets/ui/__init__.py:445 |  |  | 0.492 |
| walker |  | 8803 | 0 | python decl at beets/ui/__init__.py:498 |  |  | 0.492 |
| walker |  | 8803 | 0 | python decl at beets/ui/__init__.py:637 |  |  | 0.492 |
| walker |  | 8803 | 0 | python decl at beets/ui/__init__.py:676 |  |  | 0.492 |
| walker |  | 8803 | 0 | python decl at beets/ui/__init__.py:830 |  |  | 0.492 |
| walker |  | 8812 | 9 | python decl at beets/ui/__init__.py:459 |  |  | 0.493 |
| walker |  | 8829 | 17 | python class body at beets/ui/__init__.py:676 |  |  | 0.493 |
| walker |  | 8850 | 21 | python class body at beets/ui/__init__.py:637 |  |  | 0.494 |
| walker |  | 8864 | 14 | python decl doc at beets/ui/__init__.py:445 |  |  | 0.494 |
| walker |  | 8880 | 16 | python decl doc at beets/ui/__init__.py:459 |  |  | 0.494 |
| ns | 8907 |  | 252 | `BeetsPlugin` class header + registration-method locations | 8.1 |  | 0.489 |
| walker |  | 8909 | 29 | python decl doc at beets/ui/__init__.py:676 |  |  | 0.489 |
| walker |  | 8939 | 30 | python decl doc at beets/ui/__init__.py:433 |  |  | 0.489 |
| walker |  | 8975 | 36 | python decl doc at beets/ui/__init__.py:637 |  |  | 0.493 |
| walker |  | 8999 | 24 | python decl at beets/ui/__init__.py:807 |  |  | 0.493 |
| walker |  | 9014 | 15 | python decl doc at beets/ui/__init__.py:830 |  |  | 0.493 |
| walker |  | 9076 | 62 | python decl at beets/ui/__init__.py:207 |  |  | 0.493 |
| walker |  | 9146 | 70 | python decl at beets/ui/__init__.py:466 |  |  | 0.493 |
| walker |  | 9199 | 53 | python decl doc at beets/ui/__init__.py:383 |  |  | 0.493 |
| ns | 9216 |  | 309 | `EventType` literal — every event a listener can hook | 8.2 |  | 0.482 |
| walker |  | 9245 | 46 | python decl doc at beets/ui/__init__.py:807 |  |  | 0.484 |
| walker |  | 9391 | 146 | python decl doc at beets/ui/__init__.py:498 |  |  | 0.484 |
| ns | 9425 |  | 209 | `load_plugins`, `find_plugins`, dispatch entry points (signatures) | 8.3 |  | 0.480 |
| walker |  | 9521 | 130 | python decl doc at beets/ui/__init__.py:466 |  |  | 0.480 |
| ns | 9580 |  | 155 | `MetadataSourcePlugin` + entry-fn locations | 8.4 |  | 0.478 |
| walker |  | 9657 | 136 | python decl doc at beets/ui/__init__.py:395 |  |  | 0.478 |
| walker |  | 9942 | 285 | python method sigs #1 in beets/ui/__init__.py |  |  | 0.480 |
| walker |  | 9942 | 0 | python method at beets/ui/__init__.py:514 |  |  | 0.480 |
| walker |  | 9942 | 0 | python method at beets/ui/__init__.py:521 |  |  | 0.480 |
| walker |  | 9942 | 0 | python method at beets/ui/__init__.py:571 |  |  | 0.480 |
| walker |  | 9942 | 0 | python method at beets/ui/__init__.py:591 |  |  | 0.480 |
| walker |  | 9942 | 0 | python method at beets/ui/__init__.py:621 |  |  | 0.480 |
| walker |  | 9942 | 0 | python method at beets/ui/__init__.py:644 |  |  | 0.480 |
| walker |  | 9942 | 0 | python method at beets/ui/__init__.py:658 |  |  | 0.480 |
| walker |  | 9942 | 0 | python method at beets/ui/__init__.py:661 |  |  | 0.480 |
| walker |  | 9942 | 0 | python method at beets/ui/__init__.py:681 |  |  | 0.480 |
| walker |  | 9942 | 0 | python method at beets/ui/__init__.py:701 |  |  | 0.480 |
| walker |  | 9942 | 0 | python method at beets/ui/__init__.py:760 |  |  | 0.480 |
| walker |  | 9942 | 0 | python method at beets/ui/__init__.py:770 |  |  | 0.480 |
| walker |  | 9942 | 0 | python method at beets/ui/__init__.py:783 |  |  | 0.480 |
| walker |  | 9950 | 8 | python method at beets/ui/__init__.py:664 |  |  | 0.480 |
| walker |  | 9961 | 11 | python method at beets/ui/__init__.py:668 |  |  | 0.480 |
| ns | 9963 |  | 383 | `beetsplug/` listing — every shipped plugin | 9.1 |  | 0.456 |
| walker |  | 9977 | 16 | python method at beets/ui/__init__.py:708 |  |  | 0.456 |
| walker |  | 9985 | 8 | python method body at beets/ui/__init__.py:658 body 659 |  |  | 0.456 |
| walker |  | 9997 | 12 | python method doc at beets/ui/__init__.py:621 |  |  | 0.456 |
