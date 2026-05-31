Score(3000)=0.588 I=0.834 C=0.414 ns_rows≤3K=17/40 (reached=8 partial=2 missing=7)

| source | ns_cum | walker_cum | marginal | descriptor | id | predecessor | Score(B=cum) |
|:-------|-------:|-----------:|---------:|:-----------|:---|:------------|-------------:|
| ns | 54 |  | 54 | README headline + tagline | 1.1 |  | 0.000 |
| walker |  | 96 | 96 | listing of '.' |  |  | 0.000 |
| walker |  | 100 | 4 | listing of 'src' |  |  | 0.000 |
| walker |  | 143 | 43 | listing of 'src/pluggy' |  |  | 0.000 |
| ns | 150 |  | 96 | Top-level fs listing | 1.2 |  | 0.679 |
| ns | 193 |  | 43 | src/pluggy listing | 1.3 |  | 0.671 |
| ns | 343 |  | 150 | Public API surface (__all__) | 1.4 |  | 0.538 |
| walker |  | 459 | 316 | python imports in src/pluggy/__init__.py |  |  | 0.739 |
| walker |  | 477 | 18 | python decl names surface in src/pluggy/__init__.py |  |  | 0.739 |
| walker |  | 477 | 0 | python decl at src/pluggy/__init__.py:32 |  |  | 0.739 |
| ns | 509 |  | 166 | __init__ re-export map (which file each symbol comes from) | 2.1 |  | 0.728 |
| walker |  | 579 | 102 | README headline in README.rst |  |  | 1.000 |
| walker |  | 598 | 19 | listing of 'changelog' |  |  | 1.000 |
| ns | 624 |  | 115 | pyproject.toml build-system + license + authors | 2.2 |  | 0.911 |
| walker |  | 630 | 32 | listing of 'docs' |  |  | 0.912 |
| walker |  | 675 | 45 | [package] in pyproject.toml |  |  | 0.915 |
| walker |  | 718 | 43 | listing of 'downstream' |  |  | 0.916 |
| ns | 789 |  | 165 | pyproject.toml description + requires-python + dep groups + package layout | 2.3 | 2.2 | 0.831 |
| ns | 1108 |  | 319 | README example: spec + impls | 2.4 |  | 0.695 |
| walker |  | 1174 | 456 | README.rst section #0 |  |  | 0.819 |
| walker |  | 1177 | 3 | listing of 'docs/_static' |  |  | 0.819 |
| walker |  | 1186 | 9 | python imports in src/pluggy/_warnings.py |  |  | 0.819 |
| walker |  | 1208 | 22 | python decl names surface in src/pluggy/_warnings.py |  |  | 0.820 |
| walker |  | 1208 | 0 | python decl at src/pluggy/_warnings.py:4 |  |  | 0.820 |
| walker |  | 1222 | 14 | python decl at src/pluggy/_warnings.py:10 |  |  | 0.820 |
| walker |  | 1235 | 13 | python class body at src/pluggy/_warnings.py:4 |  |  | 0.820 |
| walker |  | 1248 | 13 | python class body at src/pluggy/_warnings.py:10 |  |  | 0.820 |
| walker |  | 1262 | 14 | python decl doc at src/pluggy/_warnings.py:4 |  |  | 0.820 |
| walker |  | 1318 | 56 | python decl body at src/pluggy/__init__.py:32 body 33 |  |  | 0.820 |
| ns | 1321 |  | 213 | README example: register + call + output | 2.5 | 2.4 | 0.804 |
| walker |  | 1322 | 4 | listing of 'docs/_static/img' |  |  | 0.804 |
| walker |  | 1384 | 62 | listing of 'testing' |  |  | 0.805 |
| walker |  | 1439 | 55 | python decl names surface in src/pluggy/_tracing.py |  |  | 0.805 |
| walker |  | 1439 | 0 | python decl at src/pluggy/_tracing.py:16 |  |  | 0.805 |
| walker |  | 1439 | 0 | python decl at src/pluggy/_tracing.py:59 |  |  | 0.805 |
| ns | 1493 |  | 172 | _hooks.py: marker + caller class/fn name surface | 3.1 |  | 0.751 |
| walker |  | 1507 | 68 | python decl names surface in src/pluggy/_callers.py |  |  | 0.752 |
| walker |  | 1536 | 29 | python decl at src/pluggy/_callers.py:27 |  |  | 0.752 |
| walker |  | 1567 | 31 | python decl doc at src/pluggy/_callers.py:27 |  |  | 0.752 |
| walker |  | 1594 | 27 | python decl at src/pluggy/_callers.py:70 |  |  | 0.752 |
| walker |  | 1626 | 32 | python decl at src/pluggy/_callers.py:60 |  |  | 0.752 |
| walker |  | 1695 | 69 | python decl names surface in src/pluggy/_result.py |  |  | 0.752 |
| walker |  | 1695 | 0 | python decl at src/pluggy/_result.py:20 |  |  | 0.752 |
| walker |  | 1704 | 9 | python decl at src/pluggy/_result.py:24 |  |  | 0.752 |
| walker |  | 1713 | 9 | python decl doc at src/pluggy/_result.py:20 |  |  | 0.752 |
| walker |  | 1733 | 20 | python class body at src/pluggy/_result.py:24 |  |  | 0.752 |
| ns | 1748 |  | 255 | _manager.py: PluginManager + helpers method-name surface | 3.2 |  | 0.690 |
| walker |  | 1767 | 34 | python decl doc at src/pluggy/_result.py:24 |  |  | 0.690 |
| walker |  | 1864 | 97 | python method sigs in src/pluggy/_result.py |  |  | 0.690 |
| walker |  | 1864 | 0 | python method at src/pluggy/_result.py:67 |  |  | 0.690 |
| walker |  | 1864 | 0 | python method at src/pluggy/_result.py:80 |  |  | 0.690 |
| walker |  | 1864 | 0 | python method at src/pluggy/_result.py:91 |  |  | 0.690 |
| walker |  | 1877 | 13 | python method at src/pluggy/_result.py:51 |  |  | 0.691 |
| walker |  | 1892 | 15 | python method at src/pluggy/_result.py:42 |  |  | 0.691 |
| walker |  | 1914 | 22 | python method at src/pluggy/_result.py:56 |  |  | 0.692 |
| walker |  | 1926 | 12 | python method doc at src/pluggy/_result.py:42 |  |  | 0.692 |
| walker |  | 1938 | 12 | python method doc at src/pluggy/_result.py:51 |  |  | 0.692 |
| walker |  | 1950 | 12 | python method doc at src/pluggy/_result.py:56 |  |  | 0.692 |
| walker |  | 1960 | 10 | python method body at src/pluggy/_result.py:51 body 54 |  |  | 0.692 |
| walker |  | 1998 | 38 | python method at src/pluggy/_result.py:31 |  |  | 0.692 |
| walker |  | 2010 | 12 | python method doc at src/pluggy/_result.py:31 |  |  | 0.692 |
| walker |  | 2014 | 4 | listing of '.claude' |  |  | 0.692 |
| walker |  | 2029 | 15 | listing of 'docs/examples' |  |  | 0.692 |
| walker |  | 2078 | 49 | python method doc at src/pluggy/_result.py:80 |  |  | 0.692 |
| walker |  | 2134 | 56 | python method doc at src/pluggy/_result.py:91 |  |  | 0.692 |
| ns | 2171 |  | 423 | Tutorial: toy-example.py in full | 3.3 |  | 0.615 |
| walker |  | 2191 | 57 | python decl at src/pluggy/_callers.py:82 |  |  | 0.615 |
| ns | 2213 |  | 42 | _callers.py top-level function names | 3.4 |  | 0.621 |
| ns | 2320 |  | 107 | _result.py class + method surface | 3.5 |  | 0.631 |
| ns | 2347 |  | 27 | _warnings.py classes | 3.6 |  | 0.634 |
| walker |  | 2401 | 210 | python method sigs in src/pluggy/_tracing.py |  |  | 0.634 |
| walker |  | 2401 | 0 | python method at src/pluggy/_tracing.py:17 |  |  | 0.634 |
| walker |  | 2401 | 0 | python method at src/pluggy/_tracing.py:22 |  |  | 0.634 |
| walker |  | 2401 | 0 | python method at src/pluggy/_tracing.py:25 |  |  | 0.634 |
| walker |  | 2401 | 0 | python method at src/pluggy/_tracing.py:42 |  |  | 0.634 |
| walker |  | 2401 | 0 | python method at src/pluggy/_tracing.py:48 |  |  | 0.634 |
| walker |  | 2401 | 0 | python method at src/pluggy/_tracing.py:51 |  |  | 0.634 |
| walker |  | 2401 | 0 | python method at src/pluggy/_tracing.py:60 |  |  | 0.634 |
| walker |  | 2401 | 0 | python method at src/pluggy/_tracing.py:64 |  |  | 0.634 |
| walker |  | 2401 | 0 | python method at src/pluggy/_tracing.py:67 |  |  | 0.634 |
| walker |  | 2410 | 9 | python method body at src/pluggy/_tracing.py:48 body 49 |  |  | 0.634 |
| walker |  | 2422 | 12 | python method body at src/pluggy/_tracing.py:22 body 23 |  |  | 0.634 |
| walker |  | 2435 | 13 | python method body at src/pluggy/_tracing.py:64 body 65 |  |  | 0.634 |
| walker |  | 2452 | 17 | python method body at src/pluggy/_tracing.py:67 body 68 |  |  | 0.634 |
| walker |  | 2470 | 18 | python method body at src/pluggy/_tracing.py:60 body 61 |  |  | 0.634 |
| walker |  | 2521 | 51 | python decl doc at src/pluggy/_callers.py:82 |  |  | 0.634 |
| ns | 2533 |  | 186 | HookspecOpts TypedDict fields | 4.1 | 3.1 | 0.612 |
| walker |  | 2637 | 116 | python decl names surface in src/pluggy/_manager.py |  |  | 0.614 |
| walker |  | 2637 | 0 | python decl at src/pluggy/_manager.py:42 |  |  | 0.614 |
| walker |  | 2637 | 0 | python decl at src/pluggy/_manager.py:52 |  |  | 0.614 |
| walker |  | 2637 | 0 | python decl at src/pluggy/_manager.py:65 |  |  | 0.614 |
| walker |  | 2637 | 0 | python decl at src/pluggy/_manager.py:83 |  |  | 0.614 |
| walker |  | 2637 | 0 | python decl at src/pluggy/_manager.py:525 |  |  | 0.614 |
| walker |  | 2648 | 11 | python decl doc at src/pluggy/_manager.py:65 |  |  | 0.614 |
| walker |  | 2689 | 41 | python decl doc at src/pluggy/_manager.py:52 |  |  | 0.614 |
| walker |  | 2705 | 16 | python decl body at src/pluggy/_manager.py:525 body 526 |  |  | 0.614 |
| walker |  | 2733 | 28 | python decl at src/pluggy/_manager.py:37 |  |  | 0.614 |
| ns | 2781 |  | 248 | HookimplOpts TypedDict fields | 4.2 | 3.1 | 0.588 |
| walker |  | 2792 | 59 | python imports in src/pluggy/_tracing.py |  |  | 0.588 |
| walker |  | 2961 | 169 | python decl doc at src/pluggy/_manager.py:83 |  |  | 0.588 |
| ns | 3060 |  | 279 | docs/index.rst section heading map | 4.3 |  | 0.546 |
| walker |  | 3132 | 171 | python decl doc at src/pluggy/_warnings.py:10 |  |  | 0.546 |
| walker |  | 3140 | 8 | listing of 'docs/examples/eggsample' |  |  | 0.546 |
| walker |  | 3160 | 20 | listing of 'docs/examples/eggsample/eggsample' |  |  | 0.546 |
| walker |  | 3168 | 8 | python imports in docs/examples/eggsample/eggsample/__init__.py |  |  | 0.546 |
| walker |  | 3255 | 87 | python method doc at src/pluggy/_result.py:67 |  |  | 0.546 |
| walker |  | 3304 | 49 | python method body at src/pluggy/_tracing.py:17 body 18 |  |  | 0.546 |
| walker |  | 3322 | 18 | python decl names surface in docs/examples/eggsample/eggsample/__init__.py |  |  | 0.546 |
| walker |  | 3333 | 11 | python method body at src/pluggy/_result.py:42 body 45 |  |  | 0.546 |
| ns | 3368 |  | 308 | HookCaller._add_hookimpl - the ordering algorithm | 4.4 |  | 0.524 |
| walker |  | 3434 | 101 | python imports in src/pluggy/_result.py |  |  | 0.524 |
| walker |  | 3489 | 55 | python method body at src/pluggy/_tracing.py:51 body 52 |  |  | 0.524 |
| walker |  | 3502 | 13 | listing of '.github' |  |  | 0.524 |
| walker |  | 3506 | 4 | listing of '.github/workflows' |  |  | 0.524 |
| walker |  | 3520 | 14 | listing of 'scripts' |  |  | 0.524 |
| walker |  | 3701 | 181 | python decl names surface in src/pluggy/_hooks.py |  |  | 0.527 |
| walker |  | 3701 | 0 | python decl at src/pluggy/_hooks.py:40 |  |  | 0.527 |
| walker |  | 3701 | 0 | python decl at src/pluggy/_hooks.py:56 |  |  | 0.527 |
| walker |  | 3701 | 0 | python decl at src/pluggy/_hooks.py:281 |  |  | 0.527 |
| walker |  | 3708 | 7 | python decl at src/pluggy/_hooks.py:77 |  |  | 0.528 |
| walker |  | 3715 | 7 | python decl at src/pluggy/_hooks.py:164 |  |  | 0.529 |
| walker |  | 3725 | 10 | python decl doc at src/pluggy/_hooks.py:40 |  |  | 0.530 |
| walker |  | 3735 | 10 | python decl doc at src/pluggy/_hooks.py:56 |  |  | 0.530 |
| walker |  | 3749 | 14 | python class body at src/pluggy/_hooks.py:77 |  |  | 0.530 |
| walker |  | 3763 | 14 | python class body at src/pluggy/_hooks.py:164 |  |  | 0.530 |
| walker |  | 3840 | 77 | python decl doc at src/pluggy/_hooks.py:164 |  |  | 0.530 |
| walker |  | 3918 | 78 | python decl doc at src/pluggy/_hooks.py:77 |  |  | 0.530 |
| walker |  | 4052 | 134 | python method sigs in src/pluggy/_hooks.py |  |  | 0.530 |
| walker |  | 4052 | 0 | python method at src/pluggy/_hooks.py:88 |  |  | 0.530 |
| walker |  | 4052 | 0 | python method at src/pluggy/_hooks.py:175 |  |  | 0.530 |
| walker |  | 4064 | 12 | python method body at src/pluggy/_hooks.py:88 body 89 |  |  | 0.530 |
| walker |  | 4076 | 12 | python method body at src/pluggy/_hooks.py:175 body 176 |  |  | 0.530 |
| walker |  | 4112 | 36 | python decl at src/pluggy/_hooks.py:33 |  |  | 0.530 |
| walker |  | 4194 | 82 | python method at src/pluggy/_hooks.py:91 |  |  | 0.530 |
| walker |  | 4205 | 11 | python method body at src/pluggy/_hooks.py:91 body 99 |  |  | 0.530 |
| walker |  | 4295 | 90 | python method at src/pluggy/_hooks.py:101 |  |  | 0.530 |
| walker |  | 4311 | 16 | python method body at src/pluggy/_hooks.py:101 body 109 |  |  | 0.530 |
| ns | 4391 |  | 1023 | _multicall body - the call loop | 4.5 | 3.4 | 0.463 |
| walker |  | 4475 | 164 | python class body at src/pluggy/_hooks.py:40 |  |  | 0.490 |
| walker |  | 4570 | 95 | python method at src/pluggy/_hooks.py:111 |  |  | 0.490 |
| walker |  | 4667 | 97 | python method at src/pluggy/_hooks.py:178 |  |  | 0.490 |
| walker |  | 4678 | 11 | python method body at src/pluggy/_hooks.py:178 body 188 |  |  | 0.490 |
| walker |  | 4783 | 105 | python method at src/pluggy/_hooks.py:190 |  |  | 0.490 |
| walker |  | 4799 | 16 | python method body at src/pluggy/_hooks.py:190 body 200 |  |  | 0.490 |
| walker |  | 4909 | 110 | python method at src/pluggy/_hooks.py:202 |  |  | 0.490 |
| ns | 5054 |  | 663 | PluginManager.register body | 4.6 | 3.2 | 0.460 |
| walker |  | 5135 | 226 | python class body at src/pluggy/_hooks.py:56 |  |  | 0.493 |
| walker |  | 5210 | 75 | python decl body at src/pluggy/_hooks.py:281 body 282 |  |  | 0.493 |
| ns | 5262 |  | 208 | HookCaller.__call__ body | 4.7 | 3.1 | 0.484 |
| walker |  | 5323 | 113 | README.rst section #1 |  |  | 0.484 |
| walker |  | 5436 | 113 | python decl names surface #1 in src/pluggy/_hooks.py |  |  | 0.492 |
| walker |  | 5436 | 0 | python decl at src/pluggy/_hooks.py:293 |  |  | 0.492 |
| walker |  | 5436 | 0 | python decl at src/pluggy/_hooks.py:382 |  |  | 0.492 |
| walker |  | 5436 | 0 | python decl at src/pluggy/_hooks.py:593 |  |  | 0.492 |
| walker |  | 5442 | 6 | python decl at src/pluggy/_hooks.py:358 |  |  | 0.494 |
| walker |  | 5448 | 6 | python decl at src/pluggy/_hooks.py:638 |  |  | 0.496 |
| walker |  | 5454 | 6 | python decl at src/pluggy/_hooks.py:696 |  |  | 0.496 |
| walker |  | 5469 | 15 | python decl doc at src/pluggy/_hooks.py:382 |  |  | 0.496 |
| walker |  | 5487 | 18 | python decl doc at src/pluggy/_hooks.py:638 |  |  | 0.496 |
| walker |  | 5518 | 31 | python decl doc at src/pluggy/_hooks.py:358 |  |  | 0.496 |
| walker |  | 5563 | 45 | python class body at src/pluggy/_hooks.py:358 |  |  | 0.496 |
| ns | 5585 |  | 323 | HookCaller.call_historic body | 4.8 | 3.1 | 0.482 |
| walker |  | 5589 | 26 | python decl at src/pluggy/_hooks.py:377 |  |  | 0.482 |
| walker |  | 5677 | 88 | python class body at src/pluggy/_hooks.py:382 |  |  | 0.482 |
| walker |  | 5765 | 88 | python class body at src/pluggy/_hooks.py:696 |  |  | 0.482 |
| walker |  | 5795 | 30 | python decl doc at src/pluggy/_hooks.py:593 |  |  | 0.482 |
| walker |  | 5907 | 112 | python class body at src/pluggy/_hooks.py:638 |  |  | 0.482 |
| walker |  | 5979 | 72 | python decl doc at src/pluggy/_hooks.py:293 |  |  | 0.482 |
| ns | 6025 |  | 440 | HookCaller.call_extra body | 4.9 | 3.1 | 0.465 |
| walker |  | 6363 | 384 | python method sigs #1 in src/pluggy/_hooks.py |  |  | 0.483 |
| walker |  | 6363 | 0 | python method at src/pluggy/_hooks.py:365 |  |  | 0.483 |
| walker |  | 6363 | 0 | python method at src/pluggy/_hooks.py:420 |  |  | 0.483 |
| walker |  | 6363 | 0 | python method at src/pluggy/_hooks.py:438 |  |  | 0.483 |
| walker |  | 6363 | 0 | python method at src/pluggy/_hooks.py:442 |  |  | 0.483 |
| walker |  | 6363 | 0 | python method at src/pluggy/_hooks.py:449 |  |  | 0.483 |
| walker |  | 6363 | 0 | python method at src/pluggy/_hooks.py:453 |  |  | 0.483 |
| walker |  | 6363 | 0 | python method at src/pluggy/_hooks.py:477 |  |  | 0.483 |
| walker |  | 6363 | 0 | python method at src/pluggy/_hooks.py:499 |  |  | 0.483 |
| walker |  | 6363 | 0 | python method at src/pluggy/_hooks.py:577 |  |  | 0.483 |
| walker |  | 6363 | 0 | python method at src/pluggy/_hooks.py:612 |  |  | 0.483 |
| walker |  | 6363 | 0 | python method at src/pluggy/_hooks.py:634 |  |  | 0.483 |
| walker |  | 6363 | 0 | python method at src/pluggy/_hooks.py:692 |  |  | 0.483 |
| walker |  | 6363 | 0 | python method at src/pluggy/_hooks.py:709 |  |  | 0.483 |
| walker |  | 6384 | 21 | python method at src/pluggy/_hooks.py:626 |  |  | 0.483 |
| walker |  | 6394 | 10 | python method doc at src/pluggy/_hooks.py:365 |  |  | 0.483 |
| walker |  | 6407 | 13 | python method doc at src/pluggy/_hooks.py:449 |  |  | 0.483 |
| walker |  | 6436 | 29 | python method at src/pluggy/_hooks.py:543 |  |  | 0.483 |
| walker |  | 6446 | 10 | python method body at src/pluggy/_hooks.py:420 body 421 |  |  | 0.483 |
| walker |  | 6457 | 11 | python method body at src/pluggy/_hooks.py:626 body 628 |  |  | 0.483 |
| walker |  | 6473 | 16 | python method doc at src/pluggy/_hooks.py:438 |  |  | 0.483 |
| walker |  | 6488 | 15 | python method at src/pluggy/_hooks.py:480 |  |  | 0.483 |
| walker |  | 6501 | 13 | python method body at src/pluggy/_hooks.py:449 body 451 |  |  | 0.483 |
| walker |  | 6540 | 39 | python method at src/pluggy/_hooks.py:424 |  |  | 0.483 |
| walker |  | 6556 | 16 | python method at src/pluggy/_hooks.py:618 |  |  | 0.483 |
| walker |  | 6570 | 14 | python method body at src/pluggy/_hooks.py:438 body 440 |  |  | 0.483 |
| walker |  | 6585 | 15 | python method body at src/pluggy/_hooks.py:477 body 478 |  |  | 0.483 |
| walker |  | 6597 | 12 | python method doc at src/pluggy/_hooks.py:453 |  |  | 0.483 |
| walker |  | 6614 | 17 | python method body at src/pluggy/_hooks.py:634 body 635 |  |  | 0.483 |
| walker |  | 6664 | 50 | python method at src/pluggy/_hooks.py:516 |  |  | 0.485 |
| walker |  | 6688 | 24 | python method at src/pluggy/_hooks.py:630 |  |  | 0.485 |
| ns | 6694 |  | 669 | PluginManager._verify_hook body - validation rules | 4.10 |  | 0.463 |
| ns | 6748 |  | 54 | docs/examples listing (eggsample + eggsample-spam) | 5.1 |  | 0.468 |
| walker |  | 6749 | 61 | python method at src/pluggy/_hooks.py:656 |  |  | 0.468 |
| walker |  | 6761 | 12 | python method doc at src/pluggy/_hooks.py:656 |  |  | 0.468 |
| walker |  | 6829 | 68 | python method at src/pluggy/_hooks.py:393 |  |  | 0.468 |
| walker |  | 6841 | 12 | python method doc at src/pluggy/_hooks.py:393 |  |  | 0.468 |
| walker |  | 6860 | 19 | python method doc at src/pluggy/_hooks.py:577 |  |  | 0.468 |
| walker |  | 6905 | 45 | python method doc at src/pluggy/_hooks.py:543 |  |  | 0.470 |
| walker |  | 6930 | 25 | python method body at src/pluggy/_hooks.py:692 body 693 |  |  | 0.470 |
| walker |  | 6943 | 13 | python method body at src/pluggy/_hooks.py:630 body 632 |  |  | 0.470 |
| walker |  | 7008 | 65 | python method doc at src/pluggy/_hooks.py:499 |  |  | 0.473 |
| ns | 7024 |  | 276 | eggsample hookspecs | 5.2 |  | 0.462 |
| walker |  | 7101 | 93 | python method doc at src/pluggy/_hooks.py:516 |  |  | 0.468 |
| walker |  | 7163 | 62 | python method body at src/pluggy/_hooks.py:612 body 613 |  |  | 0.468 |
| ns | 7282 |  | 258 | eggsample host wiring + main() | 5.3 | 5.2 | 0.458 |
| ns | 7530 |  | 248 | eggsample-spam plugin code | 5.4 |  | 0.449 |
| ns | 7723 |  | 193 | eggsample + eggsample-spam setup.py: entry-point wiring | 5.5 |  | 0.442 |
| walker |  | 7777 | 614 | python method sigs in src/pluggy/_manager.py |  |  | 0.479 |
| walker |  | 7777 | 0 | python method at src/pluggy/_manager.py:59 |  |  | 0.479 |
| walker |  | 7777 | 0 | python method at src/pluggy/_manager.py:68 |  |  | 0.479 |
| walker |  | 7777 | 0 | python method at src/pluggy/_manager.py:76 |  |  | 0.479 |
| walker |  | 7777 | 0 | python method at src/pluggy/_manager.py:79 |  |  | 0.479 |
| walker |  | 7777 | 0 | python method at src/pluggy/_manager.py:125 |  |  | 0.479 |
| walker |  | 7777 | 0 | python method at src/pluggy/_manager.py:176 |  |  | 0.479 |
| walker |  | 7777 | 0 | python method at src/pluggy/_manager.py:233 |  |  | 0.479 |
| walker |  | 7777 | 0 | python method at src/pluggy/_manager.py:238 |  |  | 0.479 |
| walker |  | 7777 | 0 | python method at src/pluggy/_manager.py:242 |  |  | 0.479 |
| walker |  | 7777 | 0 | python method at src/pluggy/_manager.py:252 |  |  | 0.479 |
| walker |  | 7777 | 0 | python method at src/pluggy/_manager.py:296 |  |  | 0.479 |
| walker |  | 7777 | 0 | python method at src/pluggy/_manager.py:300 |  |  | 0.479 |
| walker |  | 7777 | 0 | python method at src/pluggy/_manager.py:304 |  |  | 0.479 |
| walker |  | 7777 | 0 | python method at src/pluggy/_manager.py:315 |  |  | 0.479 |
| walker |  | 7777 | 0 | python method at src/pluggy/_manager.py:319 |  |  | 0.479 |
| walker |  | 7777 | 0 | python method at src/pluggy/_manager.py:323 |  |  | 0.479 |
| walker |  | 7777 | 0 | python method at src/pluggy/_manager.py:331 |  |  | 0.479 |
| walker |  | 7777 | 0 | python method at src/pluggy/_manager.py:379 |  |  | 0.479 |
| walker |  | 7777 | 0 | python method at src/pluggy/_manager.py:395 |  |  | 0.479 |
| walker |  | 7777 | 0 | python method at src/pluggy/_manager.py:425 |  |  | 0.479 |
| walker |  | 7777 | 0 | python method at src/pluggy/_manager.py:430 |  |  | 0.479 |
| walker |  | 7777 | 0 | python method at src/pluggy/_manager.py:434 |  |  | 0.479 |
| walker |  | 7777 | 0 | python method at src/pluggy/_manager.py:487 |  |  | 0.479 |
| walker |  | 7785 | 8 | python method at src/pluggy/_manager.py:100 |  |  | 0.487 |
| ns | 7785 |  | 62 | testing/ directory listing | 5.6 |  | 0.487 |
| walker |  | 7796 | 11 | python method at src/pluggy/_manager.py:71 |  |  | 0.487 |
| walker |  | 7808 | 12 | python method doc at src/pluggy/_manager.py:300 |  |  | 0.487 |
| walker |  | 7834 | 26 | python method at src/pluggy/_manager.py:512 |  |  | 0.487 |
| walker |  | 7843 | 9 | python method body at src/pluggy/_manager.py:68 body 69 |  |  | 0.487 |
| walker |  | 7856 | 13 | python method doc at src/pluggy/_manager.py:238 |  |  | 0.487 |
| walker |  | 7869 | 13 | python method doc at src/pluggy/_manager.py:296 |  |  | 0.487 |
| walker |  | 7898 | 29 | python method at src/pluggy/_manager.py:278 |  |  | 0.487 |
| walker |  | 7927 | 29 | python method at src/pluggy/_manager.py:450 |  |  | 0.487 |
| walker |  | 7942 | 15 | python method doc at src/pluggy/_manager.py:319 |  |  | 0.487 |
| walker |  | 7958 | 16 | python method doc at src/pluggy/_manager.py:233 |  |  | 0.487 |
| walker |  | 7974 | 16 | python method doc at src/pluggy/_manager.py:315 |  |  | 0.487 |
| walker |  | 8007 | 33 | python method at src/pluggy/_manager.py:201 |  |  | 0.487 |
| walker |  | 8026 | 19 | python method doc at src/pluggy/_manager.py:430 |  |  | 0.487 |
| ns | 8033 |  | 248 | Shared test fixtures (conftest.py) | 5.7 |  | 0.478 |
| walker |  | 8039 | 13 | python method body at src/pluggy/_manager.py:76 body 77 |  |  | 0.478 |
| walker |  | 8052 | 13 | python method body at src/pluggy/_manager.py:425 body 428 |  |  | 0.478 |
| ns | 8065 |  | 32 | docs/ listing | 5.8 |  | 0.483 |
| walker |  | 8066 | 14 | python method body at src/pluggy/_manager.py:315 body 317 |  |  | 0.483 |
| walker |  | 8080 | 14 | python method body at src/pluggy/_manager.py:430 body 432 |  |  | 0.483 |
| walker |  | 8095 | 15 | python method body at src/pluggy/_manager.py:319 body 321 |  |  | 0.483 |
| walker |  | 8122 | 27 | python method doc at src/pluggy/_manager.py:323 |  |  | 0.483 |
| walker |  | 8150 | 28 | python method doc at src/pluggy/_manager.py:242 |  |  | 0.483 |
| walker |  | 8178 | 28 | python method doc at src/pluggy/_manager.py:425 |  |  | 0.483 |
| walker |  | 8197 | 19 | python method body at src/pluggy/_manager.py:79 body 80 |  |  | 0.483 |
| walker |  | 8217 | 20 | python method body at src/pluggy/_manager.py:300 body 302 |  |  | 0.483 |
| walker |  | 8252 | 35 | python method doc at src/pluggy/_manager.py:487 |  |  | 0.483 |
| walker |  | 8274 | 22 | python method body at src/pluggy/_manager.py:71 body 73 |  |  | 0.483 |
| walker |  | 8316 | 42 | python method doc at src/pluggy/_manager.py:379 |  |  | 0.483 |
| walker |  | 8340 | 24 | python method body at src/pluggy/_manager.py:238 body 240 |  |  | 0.483 |
| walker |  | 8364 | 24 | python method body at src/pluggy/_manager.py:296 body 298 |  |  | 0.483 |
| ns | 8413 |  | 348 | HookCaller.__init__ + the hookimpls-list layout comment | 5.9 | 3.1 | 0.475 |
| walker |  | 8414 | 50 | python method doc at src/pluggy/_manager.py:512 |  |  | 0.475 |
| walker |  | 8470 | 56 | python method doc at src/pluggy/_manager.py:252 |  |  | 0.475 |
| walker |  | 8528 | 58 | python method doc at src/pluggy/_manager.py:434 |  |  | 0.475 |
| walker |  | 8559 | 31 | python method body at src/pluggy/_manager.py:59 body 60 |  |  | 0.475 |
| walker |  | 8567 | 8 | python method body at src/pluggy/_manager.py:242 body 250 |  |  | 0.475 |
| walker |  | 8575 | 8 | python method body at src/pluggy/_manager.py:323 body 329 |  |  | 0.475 |
| walker |  | 8652 | 77 | python method doc at src/pluggy/_manager.py:201 |  |  | 0.475 |
| walker |  | 8742 | 90 | python method doc at src/pluggy/_manager.py:304 |  |  | 0.475 |
| ns | 8779 |  | 366 | test_pluginmanager.py: test function name list | 5.10 |  | 0.465 |
| walker |  | 8833 | 91 | python method doc at src/pluggy/_manager.py:395 |  |  | 0.465 |
| walker |  | 8843 | 10 | python method body at src/pluggy/_manager.py:233 body 235 |  |  | 0.465 |
| ns | 8916 |  | 137 | CHANGELOG.rst recent-release headers | 6.1 |  | 0.463 |
| walker |  | 8963 | 120 | python method doc at src/pluggy/_manager.py:125 |  |  | 0.465 |
| ns | 9083 |  | 167 | Test files: per-file test-fn name lists (smaller files) | 6.2 |  | 0.460 |
| walker |  | 9086 | 123 | python method doc at src/pluggy/_manager.py:176 |  |  | 0.460 |
| walker |  | 9098 | 12 | python method body at src/pluggy/_manager.py:304 body 313 |  |  | 0.460 |
| walker |  | 9234 | 136 | python method doc at src/pluggy/_manager.py:278 |  |  | 0.460 |
| walker |  | 9398 | 164 | python method doc at src/pluggy/_manager.py:450 |  |  | 0.460 |
| walker |  | 9413 | 15 | python method body at src/pluggy/_manager.py:233 body 236 |  |  | 0.460 |
| walker |  | 9479 | 66 | python decl body at src/pluggy/_callers.py:60 body 64 |  |  | 0.460 |
| ns | 9525 |  | 442 | api_reference.rst - autoclass directives | 6.3 |  | 0.446 |
| ns | 9618 |  | 93 | downstream/ + scripts/ + .github/ + changelog/ listings | 6.4 |  | 0.457 |
| walker |  | 9640 | 161 | python imports in src/pluggy/_callers.py |  |  | 0.457 |
| walker |  | 9737 | 97 | python method at src/pluggy/_manager.py:114 |  |  | 0.457 |
| walker |  | 9759 | 22 | python method body at src/pluggy/_manager.py:114 body 123 |  |  | 0.457 |
| walker |  | 9834 | 75 | python decl body at src/pluggy/_manager.py:42 body 43 |  |  | 0.457 |
| ns | 9930 |  | 312 | load_setuptools_entrypoints body | 6.5 | 3.2 | 0.452 |
