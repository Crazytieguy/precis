Score(3000)=0.697 I=0.872 C=0.557 ns_rows≤3K=17/40 (reached=10 partial=0 missing=7)

| source | ns_cum | walker_cum | marginal | descriptor | id | predecessor | Score(B=cum) |
|:-------|-------:|-----------:|---------:|:-----------|:---|:------------|-------------:|
| ns | 54 |  | 54 | README headline + tagline | 1.1 |  | 0.000 |
| walker |  | 96 | 96 | listing of '.' |  |  | 0.000 |
| walker |  | 100 | 4 | listing of 'src' |  |  | 0.000 |
| ns | 150 |  | 96 | Top-level fs listing | 1.2 |  | 0.639 |
| ns | 193 |  | 43 | src/pluggy listing | 1.3 |  | 0.550 |
| walker |  | 217 | 117 | README headline in README.rst |  |  | 0.860 |
| walker |  | 260 | 43 | listing of 'src/pluggy' |  |  | 1.000 |
| ns | 343 |  | 150 | Public API surface (__all__) | 1.4 |  | 0.802 |
| ns | 509 |  | 166 | __init__ re-export map (which file each symbol comes from) | 2.1 |  | 0.702 |
| walker |  | 576 | 316 | python imports in src/pluggy/__init__.py |  |  | 1.000 |
| walker |  | 594 | 18 | python decl names surface in src/pluggy/__init__.py |  |  | 1.000 |
| walker |  | 594 | 0 | python decl at src/pluggy/__init__.py:32 |  |  | 1.000 |
| walker |  | 613 | 19 | listing of 'changelog' |  |  | 1.000 |
| ns | 624 |  | 115 | pyproject.toml build-system + license + authors | 2.2 |  | 0.911 |
| walker |  | 645 | 32 | listing of 'docs' |  |  | 0.912 |
| walker |  | 690 | 45 | [package] in pyproject.toml |  |  | 0.915 |
| walker |  | 733 | 43 | listing of 'downstream' |  |  | 0.916 |
| walker |  | 755 | 22 | python decl names surface in src/pluggy/_warnings.py |  |  | 0.916 |
| walker |  | 755 | 0 | python decl at src/pluggy/_warnings.py:4 |  |  | 0.916 |
| walker |  | 769 | 14 | python decl at src/pluggy/_warnings.py:10 |  |  | 0.916 |
| walker |  | 782 | 13 | python class body at src/pluggy/_warnings.py:4 |  |  | 0.916 |
| ns | 789 |  | 165 | pyproject.toml description + requires-python + dep groups + package layout | 2.3 | 2.2 | 0.832 |
| walker |  | 795 | 13 | python class body at src/pluggy/_warnings.py:10 |  |  | 0.832 |
| walker |  | 809 | 14 | python decl doc at src/pluggy/_warnings.py:4 |  |  | 0.832 |
| walker |  | 818 | 9 | python imports in src/pluggy/_warnings.py |  |  | 0.832 |
| ns | 1108 |  | 319 | README example: spec + impls | 2.4 |  | 0.695 |
| ns | 1321 |  | 213 | README example: register + call + output | 2.5 | 2.4 | 0.634 |
| walker |  | 1364 | 546 | README.rst section #0 |  |  | 0.884 |
| walker |  | 1367 | 3 | listing of 'docs/_static' |  |  | 0.884 |
| walker |  | 1371 | 4 | listing of 'docs/_static/img' |  |  | 0.884 |
| walker |  | 1450 | 79 | python decl names surface in src/pluggy/_result.py |  |  | 0.884 |
| walker |  | 1450 | 0 | python decl at src/pluggy/_result.py:20 |  |  | 0.884 |
| walker |  | 1459 | 9 | python decl at src/pluggy/_result.py:24 |  |  | 0.884 |
| walker |  | 1468 | 9 | python decl doc at src/pluggy/_result.py:20 |  |  | 0.884 |
| walker |  | 1488 | 20 | python class body at src/pluggy/_result.py:24 |  |  | 0.884 |
| ns | 1493 |  | 172 | _hooks.py: marker + caller class/fn name surface | 3.1 |  | 0.825 |
| walker |  | 1522 | 34 | python decl doc at src/pluggy/_result.py:24 |  |  | 0.825 |
| walker |  | 1619 | 97 | python method sigs in src/pluggy/_result.py |  |  | 0.826 |
| walker |  | 1619 | 0 | python method at src/pluggy/_result.py:67 |  |  | 0.826 |
| walker |  | 1619 | 0 | python method at src/pluggy/_result.py:80 |  |  | 0.826 |
| walker |  | 1619 | 0 | python method at src/pluggy/_result.py:91 |  |  | 0.826 |
| walker |  | 1632 | 13 | python method at src/pluggy/_result.py:51 |  |  | 0.826 |
| walker |  | 1647 | 15 | python method at src/pluggy/_result.py:42 |  |  | 0.826 |
| walker |  | 1669 | 22 | python method at src/pluggy/_result.py:56 |  |  | 0.827 |
| walker |  | 1681 | 12 | python method doc at src/pluggy/_result.py:42 |  |  | 0.827 |
| walker |  | 1693 | 12 | python method doc at src/pluggy/_result.py:51 |  |  | 0.827 |
| walker |  | 1705 | 12 | python method doc at src/pluggy/_result.py:56 |  |  | 0.827 |
| walker |  | 1715 | 10 | python method body at src/pluggy/_result.py:51 body 54 |  |  | 0.827 |
| ns | 1748 |  | 255 | _manager.py: PluginManager + helpers method-name surface | 3.2 |  | 0.758 |
| walker |  | 1753 | 38 | python method at src/pluggy/_result.py:31 |  |  | 0.758 |
| walker |  | 1765 | 12 | python method doc at src/pluggy/_result.py:31 |  |  | 0.758 |
| walker |  | 1831 | 66 | python decl body at src/pluggy/__init__.py:32 body 33 |  |  | 0.758 |
| walker |  | 1947 | 116 | python decl names surface in src/pluggy/_manager.py |  |  | 0.759 |
| walker |  | 1947 | 0 | python decl at src/pluggy/_manager.py:42 |  |  | 0.759 |
| walker |  | 1947 | 0 | python decl at src/pluggy/_manager.py:52 |  |  | 0.759 |
| walker |  | 1947 | 0 | python decl at src/pluggy/_manager.py:65 |  |  | 0.759 |
| walker |  | 1947 | 0 | python decl at src/pluggy/_manager.py:83 |  |  | 0.759 |
| walker |  | 1947 | 0 | python decl at src/pluggy/_manager.py:525 |  |  | 0.759 |
| walker |  | 1958 | 11 | python decl doc at src/pluggy/_manager.py:65 |  |  | 0.759 |
| walker |  | 2004 | 46 | python decl doc at src/pluggy/_manager.py:52 |  |  | 0.759 |
| walker |  | 2020 | 16 | python decl body at src/pluggy/_manager.py:525 body 526 |  |  | 0.759 |
| walker |  | 2048 | 28 | python decl at src/pluggy/_manager.py:37 |  |  | 0.759 |
| ns | 2171 |  | 423 | Tutorial: toy-example.py in full | 3.3 |  | 0.675 |
| walker |  | 2204 | 156 | python decl names surface in src/pluggy/_hooks.py |  |  | 0.688 |
| walker |  | 2204 | 0 | python decl at src/pluggy/_hooks.py:40 |  |  | 0.688 |
| walker |  | 2204 | 0 | python decl at src/pluggy/_hooks.py:56 |  |  | 0.688 |
| walker |  | 2204 | 0 | python decl at src/pluggy/_hooks.py:281 |  |  | 0.688 |
| walker |  | 2204 | 0 | python decl at src/pluggy/_hooks.py:293 |  |  | 0.688 |
| walker |  | 2204 | 0 | python decl at src/pluggy/_hooks.py:382 |  |  | 0.688 |
| walker |  | 2204 | 0 | python decl at src/pluggy/_hooks.py:593 |  |  | 0.688 |
| walker |  | 2210 | 6 | python decl at src/pluggy/_hooks.py:358 |  |  | 0.690 |
| ns | 2213 |  | 42 | _callers.py top-level function names | 3.4 |  | 0.684 |
| walker |  | 2216 | 6 | python decl at src/pluggy/_hooks.py:638 |  |  | 0.686 |
| walker |  | 2222 | 6 | python decl at src/pluggy/_hooks.py:696 |  |  | 0.686 |
| walker |  | 2229 | 7 | python decl at src/pluggy/_hooks.py:77 |  |  | 0.690 |
| walker |  | 2236 | 7 | python decl at src/pluggy/_hooks.py:164 |  |  | 0.694 |
| walker |  | 2246 | 10 | python decl doc at src/pluggy/_hooks.py:40 |  |  | 0.694 |
| walker |  | 2256 | 10 | python decl doc at src/pluggy/_hooks.py:56 |  |  | 0.694 |
| walker |  | 2270 | 14 | python class body at src/pluggy/_hooks.py:77 |  |  | 0.694 |
| walker |  | 2284 | 14 | python class body at src/pluggy/_hooks.py:164 |  |  | 0.694 |
| walker |  | 2299 | 15 | python decl doc at src/pluggy/_hooks.py:382 |  |  | 0.694 |
| walker |  | 2317 | 18 | python decl doc at src/pluggy/_hooks.py:638 |  |  | 0.694 |
| ns | 2320 |  | 107 | _result.py class + method surface | 3.5 |  | 0.702 |
| ns | 2347 |  | 27 | _warnings.py classes | 3.6 |  | 0.704 |
| walker |  | 2348 | 31 | python decl doc at src/pluggy/_hooks.py:358 |  |  | 0.704 |
| walker |  | 2398 | 50 | python class body at src/pluggy/_hooks.py:358 |  |  | 0.704 |
| walker |  | 2510 | 112 | python class body at src/pluggy/_hooks.py:638 |  |  | 0.704 |
| ns | 2533 |  | 186 | HookspecOpts TypedDict fields | 4.1 | 3.1 | 0.680 |
| walker |  | 2598 | 88 | python class body at src/pluggy/_hooks.py:696 |  |  | 0.680 |
| walker |  | 2686 | 88 | python class body at src/pluggy/_hooks.py:382 |  |  | 0.680 |
| ns | 2781 |  | 248 | HookimplOpts TypedDict fields | 4.2 | 3.1 | 0.652 |
| walker |  | 2912 | 226 | python class body at src/pluggy/_hooks.py:56 |  |  | 0.697 |
| ns | 3060 |  | 279 | docs/index.rst section heading map | 4.3 |  | 0.647 |
| walker |  | 3076 | 164 | python class body at src/pluggy/_hooks.py:40 |  |  | 0.676 |
| walker |  | 3106 | 30 | python decl doc at src/pluggy/_hooks.py:593 |  |  | 0.676 |
| walker |  | 3188 | 82 | python decl doc at src/pluggy/_hooks.py:164 |  |  | 0.676 |
| walker |  | 3271 | 83 | python decl doc at src/pluggy/_hooks.py:77 |  |  | 0.676 |
| ns | 3368 |  | 308 | HookCaller._add_hookimpl - the ordering algorithm | 4.4 |  | 0.648 |
| walker |  | 3459 | 188 | python method sigs in src/pluggy/_hooks.py |  |  | 0.651 |
| walker |  | 3459 | 0 | python method at src/pluggy/_hooks.py:88 |  |  | 0.651 |
| walker |  | 3459 | 0 | python method at src/pluggy/_hooks.py:175 |  |  | 0.651 |
| walker |  | 3459 | 0 | python method at src/pluggy/_hooks.py:365 |  |  | 0.651 |
| walker |  | 3459 | 0 | python method at src/pluggy/_hooks.py:420 |  |  | 0.651 |
| walker |  | 3469 | 10 | python method doc at src/pluggy/_hooks.py:365 |  |  | 0.651 |
| walker |  | 3479 | 10 | python method body at src/pluggy/_hooks.py:420 body 421 |  |  | 0.651 |
| walker |  | 3491 | 12 | python method body at src/pluggy/_hooks.py:88 body 89 |  |  | 0.651 |
| walker |  | 3503 | 12 | python method body at src/pluggy/_hooks.py:175 body 176 |  |  | 0.651 |
| walker |  | 3542 | 39 | python method at src/pluggy/_hooks.py:424 |  |  | 0.651 |
| walker |  | 3610 | 68 | python method at src/pluggy/_hooks.py:393 |  |  | 0.651 |
| walker |  | 3622 | 12 | python method doc at src/pluggy/_hooks.py:393 |  |  | 0.651 |
| walker |  | 3699 | 77 | python decl doc at src/pluggy/_hooks.py:293 |  |  | 0.651 |
| walker |  | 3946 | 247 | python method sigs in src/pluggy/_manager.py |  |  | 0.657 |
| walker |  | 3946 | 0 | python method at src/pluggy/_manager.py:59 |  |  | 0.657 |
| walker |  | 3946 | 0 | python method at src/pluggy/_manager.py:68 |  |  | 0.657 |
| walker |  | 3946 | 0 | python method at src/pluggy/_manager.py:76 |  |  | 0.657 |
| walker |  | 3946 | 0 | python method at src/pluggy/_manager.py:79 |  |  | 0.657 |
| walker |  | 3946 | 0 | python method at src/pluggy/_manager.py:125 |  |  | 0.657 |
| walker |  | 3946 | 0 | python method at src/pluggy/_manager.py:176 |  |  | 0.657 |
| walker |  | 3946 | 0 | python method at src/pluggy/_manager.py:233 |  |  | 0.657 |
| walker |  | 3946 | 0 | python method at src/pluggy/_manager.py:238 |  |  | 0.657 |
| walker |  | 3954 | 8 | python method at src/pluggy/_manager.py:100 |  |  | 0.657 |
| walker |  | 3965 | 11 | python method at src/pluggy/_manager.py:71 |  |  | 0.657 |
| walker |  | 3974 | 9 | python method body at src/pluggy/_manager.py:68 body 69 |  |  | 0.657 |
| walker |  | 3987 | 13 | python method doc at src/pluggy/_manager.py:238 |  |  | 0.657 |
| walker |  | 4003 | 16 | python method doc at src/pluggy/_manager.py:233 |  |  | 0.657 |
| walker |  | 4036 | 33 | python method at src/pluggy/_manager.py:201 |  |  | 0.657 |
| walker |  | 4049 | 13 | python method body at src/pluggy/_manager.py:76 body 77 |  |  | 0.657 |
| walker |  | 4068 | 19 | python method body at src/pluggy/_manager.py:79 body 80 |  |  | 0.657 |
| walker |  | 4090 | 22 | python method body at src/pluggy/_manager.py:71 body 73 |  |  | 0.657 |
| walker |  | 4114 | 24 | python method body at src/pluggy/_manager.py:238 body 240 |  |  | 0.657 |
| walker |  | 4196 | 82 | python method at src/pluggy/_hooks.py:91 |  |  | 0.657 |
| walker |  | 4207 | 11 | python method body at src/pluggy/_hooks.py:91 body 99 |  |  | 0.657 |
| walker |  | 4269 | 62 | listing of 'testing' |  |  | 0.658 |
| walker |  | 4328 | 59 | python method doc at src/pluggy/_result.py:80 |  |  | 0.658 |
| ns | 4391 |  | 1023 | _multicall body - the call loop | 4.5 | 3.4 | 0.573 |
| walker |  | 4418 | 90 | python method at src/pluggy/_hooks.py:101 |  |  | 0.573 |
| walker |  | 4434 | 16 | python method body at src/pluggy/_hooks.py:101 body 109 |  |  | 0.573 |
| walker |  | 4465 | 31 | python method body at src/pluggy/_manager.py:59 body 60 |  |  | 0.573 |
| walker |  | 4526 | 61 | python method doc at src/pluggy/_result.py:91 |  |  | 0.573 |
| walker |  | 4621 | 95 | python method at src/pluggy/_hooks.py:111 |  |  | 0.573 |
| walker |  | 4718 | 97 | python method at src/pluggy/_hooks.py:178 |  |  | 0.573 |
| walker |  | 4729 | 11 | python method body at src/pluggy/_hooks.py:178 body 188 |  |  | 0.573 |
| walker |  | 4794 | 65 | python decl names surface in src/pluggy/_tracing.py |  |  | 0.573 |
| walker |  | 4794 | 0 | python decl at src/pluggy/_tracing.py:16 |  |  | 0.573 |
| walker |  | 4794 | 0 | python decl at src/pluggy/_tracing.py:59 |  |  | 0.573 |
| walker |  | 4899 | 105 | python method at src/pluggy/_hooks.py:190 |  |  | 0.573 |
| walker |  | 4915 | 16 | python method body at src/pluggy/_hooks.py:190 body 200 |  |  | 0.573 |
| walker |  | 5025 | 110 | python method at src/pluggy/_hooks.py:202 |  |  | 0.573 |
| ns | 5054 |  | 663 | PluginManager.register body | 4.6 | 3.2 | 0.538 |
| walker |  | 5235 | 210 | python method sigs in src/pluggy/_tracing.py |  |  | 0.538 |
| walker |  | 5235 | 0 | python method at src/pluggy/_tracing.py:17 |  |  | 0.538 |
| walker |  | 5235 | 0 | python method at src/pluggy/_tracing.py:22 |  |  | 0.538 |
| walker |  | 5235 | 0 | python method at src/pluggy/_tracing.py:25 |  |  | 0.538 |
| walker |  | 5235 | 0 | python method at src/pluggy/_tracing.py:42 |  |  | 0.538 |
| walker |  | 5235 | 0 | python method at src/pluggy/_tracing.py:48 |  |  | 0.538 |
| walker |  | 5235 | 0 | python method at src/pluggy/_tracing.py:51 |  |  | 0.538 |
| walker |  | 5235 | 0 | python method at src/pluggy/_tracing.py:60 |  |  | 0.538 |
| walker |  | 5235 | 0 | python method at src/pluggy/_tracing.py:64 |  |  | 0.538 |
| walker |  | 5235 | 0 | python method at src/pluggy/_tracing.py:67 |  |  | 0.538 |
| walker |  | 5244 | 9 | python method body at src/pluggy/_tracing.py:48 body 49 |  |  | 0.538 |
| walker |  | 5256 | 12 | python method body at src/pluggy/_tracing.py:22 body 23 |  |  | 0.538 |
| ns | 5262 |  | 208 | HookCaller.__call__ body | 4.7 | 3.1 | 0.528 |
| walker |  | 5269 | 13 | python method body at src/pluggy/_tracing.py:64 body 65 |  |  | 0.528 |
| walker |  | 5286 | 17 | python method body at src/pluggy/_tracing.py:67 body 68 |  |  | 0.528 |
| walker |  | 5304 | 18 | python method body at src/pluggy/_tracing.py:60 body 61 |  |  | 0.528 |
| walker |  | 5308 | 4 | listing of '.claude' |  |  | 0.528 |
| walker |  | 5323 | 15 | listing of 'docs/examples' |  |  | 0.528 |
| walker |  | 5398 | 75 | python decl body at src/pluggy/_hooks.py:281 body 282 |  |  | 0.528 |
| walker |  | 5476 | 78 | python decl names surface in src/pluggy/_callers.py |  |  | 0.535 |
| walker |  | 5505 | 29 | python decl at src/pluggy/_callers.py:27 |  |  | 0.535 |
| walker |  | 5536 | 31 | python decl doc at src/pluggy/_callers.py:27 |  |  | 0.535 |
| walker |  | 5563 | 27 | python decl at src/pluggy/_callers.py:70 |  |  | 0.535 |
| ns | 5585 |  | 323 | HookCaller.call_historic body | 4.8 | 3.1 | 0.520 |
| walker |  | 5595 | 32 | python decl at src/pluggy/_callers.py:60 |  |  | 0.520 |
| walker |  | 5682 | 87 | python method doc at src/pluggy/_manager.py:201 |  |  | 0.520 |
| walker |  | 5871 | 189 | python decl doc at src/pluggy/_manager.py:83 |  |  | 0.520 |
| ns | 6025 |  | 440 | HookCaller.call_extra body | 4.9 | 3.1 | 0.502 |
| walker |  | 6062 | 191 | python decl doc at src/pluggy/_warnings.py:10 |  |  | 0.502 |
| walker |  | 6298 | 236 | python method sigs #1 in src/pluggy/_hooks.py |  |  | 0.517 |
| walker |  | 6298 | 0 | python method at src/pluggy/_hooks.py:438 |  |  | 0.517 |
| walker |  | 6298 | 0 | python method at src/pluggy/_hooks.py:442 |  |  | 0.517 |
| walker |  | 6298 | 0 | python method at src/pluggy/_hooks.py:449 |  |  | 0.517 |
| walker |  | 6298 | 0 | python method at src/pluggy/_hooks.py:453 |  |  | 0.517 |
| walker |  | 6298 | 0 | python method at src/pluggy/_hooks.py:477 |  |  | 0.517 |
| walker |  | 6298 | 0 | python method at src/pluggy/_hooks.py:499 |  |  | 0.517 |
| walker |  | 6298 | 0 | python method at src/pluggy/_hooks.py:577 |  |  | 0.517 |
| walker |  | 6298 | 0 | python method at src/pluggy/_hooks.py:612 |  |  | 0.517 |
| walker |  | 6311 | 13 | python method doc at src/pluggy/_hooks.py:449 |  |  | 0.517 |
| walker |  | 6340 | 29 | python method at src/pluggy/_hooks.py:543 |  |  | 0.517 |
| walker |  | 6356 | 16 | python method doc at src/pluggy/_hooks.py:438 |  |  | 0.517 |
| walker |  | 6371 | 15 | python method at src/pluggy/_hooks.py:480 |  |  | 0.517 |
| walker |  | 6384 | 13 | python method body at src/pluggy/_hooks.py:449 body 451 |  |  | 0.517 |
| walker |  | 6400 | 16 | python method at src/pluggy/_hooks.py:618 |  |  | 0.517 |
| walker |  | 6414 | 14 | python method body at src/pluggy/_hooks.py:438 body 440 |  |  | 0.517 |
| walker |  | 6429 | 15 | python method body at src/pluggy/_hooks.py:477 body 478 |  |  | 0.517 |
| walker |  | 6441 | 12 | python method doc at src/pluggy/_hooks.py:453 |  |  | 0.518 |
| walker |  | 6491 | 50 | python method at src/pluggy/_hooks.py:516 |  |  | 0.519 |
| walker |  | 6510 | 19 | python method doc at src/pluggy/_hooks.py:577 |  |  | 0.519 |
| walker |  | 6555 | 45 | python method doc at src/pluggy/_hooks.py:543 |  |  | 0.520 |
| walker |  | 6630 | 75 | python method doc at src/pluggy/_hooks.py:499 |  |  | 0.525 |
| ns | 6694 |  | 669 | PluginManager._verify_hook body - validation rules | 4.10 |  | 0.501 |
| walker |  | 6727 | 97 | python method doc at src/pluggy/_result.py:67 |  |  | 0.501 |
| ns | 6748 |  | 54 | docs/examples listing (eggsample + eggsample-spam) | 5.1 |  | 0.497 |
| walker |  | 6825 | 98 | python method doc at src/pluggy/_hooks.py:516 |  |  | 0.504 |
| ns | 7024 |  | 276 | eggsample hookspecs | 5.2 |  | 0.492 |
| walker |  | 7072 | 247 | python method sigs #1 in src/pluggy/_manager.py |  |  | 0.510 |
| walker |  | 7072 | 0 | python method at src/pluggy/_manager.py:242 |  |  | 0.510 |
| walker |  | 7072 | 0 | python method at src/pluggy/_manager.py:252 |  |  | 0.510 |
| walker |  | 7072 | 0 | python method at src/pluggy/_manager.py:296 |  |  | 0.510 |
| walker |  | 7072 | 0 | python method at src/pluggy/_manager.py:300 |  |  | 0.510 |
| walker |  | 7072 | 0 | python method at src/pluggy/_manager.py:304 |  |  | 0.510 |
| walker |  | 7072 | 0 | python method at src/pluggy/_manager.py:315 |  |  | 0.510 |
| walker |  | 7072 | 0 | python method at src/pluggy/_manager.py:319 |  |  | 0.510 |
| walker |  | 7072 | 0 | python method at src/pluggy/_manager.py:323 |  |  | 0.510 |
| walker |  | 7072 | 0 | python method at src/pluggy/_manager.py:331 |  |  | 0.510 |
| walker |  | 7072 | 0 | python method at src/pluggy/_manager.py:379 |  |  | 0.510 |
| walker |  | 7072 | 0 | python method at src/pluggy/_manager.py:395 |  |  | 0.510 |
| walker |  | 7084 | 12 | python method doc at src/pluggy/_manager.py:300 |  |  | 0.510 |
| walker |  | 7097 | 13 | python method doc at src/pluggy/_manager.py:296 |  |  | 0.510 |
| walker |  | 7126 | 29 | python method at src/pluggy/_manager.py:278 |  |  | 0.510 |
| walker |  | 7141 | 15 | python method doc at src/pluggy/_manager.py:319 |  |  | 0.510 |
| walker |  | 7157 | 16 | python method doc at src/pluggy/_manager.py:315 |  |  | 0.510 |
| walker |  | 7171 | 14 | python method body at src/pluggy/_manager.py:315 body 317 |  |  | 0.510 |
| walker |  | 7186 | 15 | python method body at src/pluggy/_manager.py:319 body 321 |  |  | 0.510 |
| walker |  | 7213 | 27 | python method doc at src/pluggy/_manager.py:323 |  |  | 0.510 |
| walker |  | 7246 | 33 | python method doc at src/pluggy/_manager.py:242 |  |  | 0.510 |
| walker |  | 7266 | 20 | python method body at src/pluggy/_manager.py:300 body 302 |  |  | 0.510 |
| ns | 7282 |  | 258 | eggsample host wiring + main() | 5.3 | 5.2 | 0.498 |
| walker |  | 7308 | 42 | python method doc at src/pluggy/_manager.py:379 |  |  | 0.498 |
| walker |  | 7332 | 24 | python method body at src/pluggy/_manager.py:296 body 298 |  |  | 0.498 |
| walker |  | 7393 | 61 | python method doc at src/pluggy/_manager.py:252 |  |  | 0.498 |
| walker |  | 7401 | 8 | python method body at src/pluggy/_manager.py:242 body 250 |  |  | 0.498 |
| walker |  | 7409 | 8 | python method body at src/pluggy/_manager.py:323 body 329 |  |  | 0.498 |
| walker |  | 7504 | 95 | python method doc at src/pluggy/_manager.py:304 |  |  | 0.498 |
| ns | 7530 |  | 248 | eggsample-spam plugin code | 5.4 |  | 0.489 |
| walker |  | 7605 | 101 | python method doc at src/pluggy/_manager.py:395 |  |  | 0.489 |
| walker |  | 7615 | 10 | python method body at src/pluggy/_manager.py:233 body 235 |  |  | 0.489 |
| ns | 7723 |  | 193 | eggsample + eggsample-spam setup.py: entry-point wiring | 5.5 |  | 0.482 |
| ns | 7785 |  | 62 | testing/ directory listing | 5.6 |  | 0.490 |
| walker |  | 7833 | 218 | python class body at src/pluggy/_hooks.py:593 |  |  | 0.490 |
| walker |  | 7890 | 57 | python decl at src/pluggy/_callers.py:82 |  |  | 0.490 |
| ns | 8033 |  | 248 | Shared test fixtures (conftest.py) | 5.7 |  | 0.481 |
| walker |  | 8043 | 153 | python decl names surface #1 in src/pluggy/_hooks.py |  |  | 0.481 |
| ns | 8065 |  | 32 | docs/ listing | 5.8 |  | 0.486 |
| walker |  | 8069 | 26 | python decl at src/pluggy/_hooks.py:377 |  |  | 0.486 |
| walker |  | 8105 | 36 | python decl at src/pluggy/_hooks.py:33 |  |  | 0.486 |
| walker |  | 8116 | 11 | python method body at src/pluggy/_result.py:42 body 45 |  |  | 0.486 |
| walker |  | 8210 | 94 | python method sigs #2 in src/pluggy/_hooks.py |  |  | 0.486 |
| walker |  | 8210 | 0 | python method at src/pluggy/_hooks.py:634 |  |  | 0.486 |
| walker |  | 8210 | 0 | python method at src/pluggy/_hooks.py:692 |  |  | 0.486 |
| walker |  | 8210 | 0 | python method at src/pluggy/_hooks.py:709 |  |  | 0.486 |
| walker |  | 8231 | 21 | python method at src/pluggy/_hooks.py:626 |  |  | 0.486 |
| walker |  | 8242 | 11 | python method body at src/pluggy/_hooks.py:626 body 628 |  |  | 0.486 |
| walker |  | 8259 | 17 | python method body at src/pluggy/_hooks.py:634 body 635 |  |  | 0.486 |
| walker |  | 8283 | 24 | python method at src/pluggy/_hooks.py:630 |  |  | 0.486 |
| walker |  | 8344 | 61 | python method at src/pluggy/_hooks.py:656 |  |  | 0.486 |
| walker |  | 8356 | 12 | python method doc at src/pluggy/_hooks.py:656 |  |  | 0.486 |
| walker |  | 8381 | 25 | python method body at src/pluggy/_hooks.py:692 body 693 |  |  | 0.486 |
| walker |  | 8394 | 13 | python method body at src/pluggy/_hooks.py:630 body 632 |  |  | 0.486 |
| walker |  | 8406 | 12 | python method body at src/pluggy/_manager.py:304 body 313 |  |  | 0.486 |
| ns | 8413 |  | 348 | HookCaller.__init__ + the hookimpls-list layout comment | 5.9 | 3.1 | 0.479 |
| walker |  | 8539 | 133 | python method doc at src/pluggy/_manager.py:176 |  |  | 0.479 |
| walker |  | 8674 | 135 | python method doc at src/pluggy/_manager.py:125 |  |  | 0.482 |
| walker |  | 8730 | 56 | python decl doc at src/pluggy/_callers.py:82 |  |  | 0.483 |
| ns | 8779 |  | 366 | test_pluginmanager.py: test function name list | 5.10 |  | 0.472 |
| walker |  | 8841 | 111 | python imports in src/pluggy/_result.py |  |  | 0.472 |
| ns | 8916 |  | 137 | CHANGELOG.rst recent-release headers | 6.1 |  | 0.470 |
| walker |  | 8987 | 146 | python method doc at src/pluggy/_manager.py:278 |  |  | 0.470 |
| walker |  | 9049 | 62 | python method body at src/pluggy/_hooks.py:612 body 613 |  |  | 0.470 |
| walker |  | 9057 | 8 | listing of 'docs/examples/eggsample' |  |  | 0.471 |
| walker |  | 9077 | 20 | listing of 'docs/examples/eggsample/eggsample' |  |  | 0.477 |
| ns | 9083 |  | 167 | Test files: per-file test-fn name lists (smaller files) | 6.2 |  | 0.472 |
| walker |  | 9085 | 8 | python imports in docs/examples/eggsample/eggsample/__init__.py |  |  | 0.472 |
| walker |  | 9154 | 69 | python imports in src/pluggy/_tracing.py |  |  | 0.472 |
| walker |  | 9274 | 120 | python method sigs #2 in src/pluggy/_manager.py |  |  | 0.485 |
| walker |  | 9274 | 0 | python method at src/pluggy/_manager.py:425 |  |  | 0.485 |
| walker |  | 9274 | 0 | python method at src/pluggy/_manager.py:430 |  |  | 0.485 |
| walker |  | 9274 | 0 | python method at src/pluggy/_manager.py:434 |  |  | 0.485 |
| walker |  | 9274 | 0 | python method at src/pluggy/_manager.py:487 |  |  | 0.485 |
| walker |  | 9300 | 26 | python method at src/pluggy/_manager.py:512 |  |  | 0.485 |
| walker |  | 9329 | 29 | python method at src/pluggy/_manager.py:450 |  |  | 0.485 |
| walker |  | 9348 | 19 | python method doc at src/pluggy/_manager.py:430 |  |  | 0.485 |
| walker |  | 9361 | 13 | python method body at src/pluggy/_manager.py:425 body 428 |  |  | 0.485 |
| walker |  | 9375 | 14 | python method body at src/pluggy/_manager.py:430 body 432 |  |  | 0.485 |
| walker |  | 9403 | 28 | python method doc at src/pluggy/_manager.py:425 |  |  | 0.485 |
| walker |  | 9443 | 40 | python method doc at src/pluggy/_manager.py:487 |  |  | 0.485 |
| walker |  | 9493 | 50 | python method doc at src/pluggy/_manager.py:512 |  |  | 0.485 |
| ns | 9525 |  | 442 | api_reference.rst - autoclass directives | 6.3 |  | 0.470 |
| walker |  | 9556 | 63 | python method doc at src/pluggy/_manager.py:434 |  |  | 0.470 |
| walker |  | 9571 | 15 | python method body at src/pluggy/_manager.py:233 body 236 |  |  | 0.470 |
| ns | 9618 |  | 93 | downstream/ + scripts/ + .github/ + changelog/ listings | 6.4 |  | 0.472 |
| walker |  | 9750 | 179 | python method doc at src/pluggy/_manager.py:450 |  |  | 0.472 |
| walker |  | 9799 | 49 | python method body at src/pluggy/_tracing.py:17 body 18 |  |  | 0.472 |
| walker |  | 9817 | 18 | python decl names surface in docs/examples/eggsample/eggsample/__init__.py |  |  | 0.472 |
| walker |  | 9914 | 97 | python method at src/pluggy/_manager.py:114 |  |  | 0.472 |
| ns | 9930 |  | 312 | load_setuptools_entrypoints body | 6.5 | 3.2 | 0.468 |
| walker |  | 9936 | 22 | python method body at src/pluggy/_manager.py:114 body 123 |  |  | 0.468 |
| walker |  | 9991 | 55 | python method body at src/pluggy/_tracing.py:51 body 52 |  |  | 0.468 |
