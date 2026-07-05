Score(3000)=0.697 I=0.872 C=0.557 ns_rows≤3K=17/40 (reached=10 partial=0 missing=7)

| source | ns_cum | walker_cum | marginal | descriptor | id | predecessor | Score(B=cum) |
|:-------|-------:|-----------:|---------:|:-----------|:---|:------------|-------------:|
| ns | 58 |  | 58 | README headline + tagline | 1.1 |  | 0.000 |
| walker |  | 96 | 96 | listing of '.' |  |  | 0.000 |
| walker |  | 100 | 4 | listing of 'src' |  |  | 0.000 |
| ns | 154 |  | 96 | Top-level fs listing | 1.2 |  | 0.639 |
| ns | 197 |  | 43 | src/pluggy listing | 1.3 |  | 0.550 |
| walker |  | 219 | 119 | README headline in README.rst |  |  | 0.860 |
| walker |  | 262 | 43 | listing of 'src/pluggy' |  |  | 1.000 |
| ns | 349 |  | 152 | Public API surface (__all__) | 1.4 |  | 0.802 |
| ns | 515 |  | 166 | __init__ re-export map (which file each symbol comes from) | 2.1 |  | 0.702 |
| walker |  | 580 | 318 | python imports in src/pluggy/__init__.py |  |  | 1.000 |
| walker |  | 596 | 16 | python decl names surface in src/pluggy/__init__.py |  |  | 1.000 |
| walker |  | 596 | 0 | python decl at src/pluggy/__init__.py:32 |  |  | 1.000 |
| walker |  | 615 | 19 | listing of 'changelog' |  |  | 1.000 |
| ns | 632 |  | 117 | pyproject.toml build-system + license + authors | 2.2 |  | 0.911 |
| walker |  | 647 | 32 | listing of 'docs' |  |  | 0.912 |
| walker |  | 690 | 43 | listing of 'downstream' |  |  | 0.913 |
| walker |  | 714 | 24 | python decl names surface in src/pluggy/_warnings.py |  |  | 0.913 |
| walker |  | 714 | 0 | python decl at src/pluggy/_warnings.py:4 |  |  | 0.913 |
| walker |  | 730 | 16 | python decl at src/pluggy/_warnings.py:10 |  |  | 0.914 |
| walker |  | 743 | 13 | python class body at src/pluggy/_warnings.py:4 |  |  | 0.914 |
| walker |  | 756 | 13 | python class body at src/pluggy/_warnings.py:10 |  |  | 0.914 |
| walker |  | 770 | 14 | python decl doc at src/pluggy/_warnings.py:4 |  |  | 0.914 |
| walker |  | 777 | 7 | python imports in src/pluggy/_warnings.py |  |  | 0.914 |
| ns | 799 |  | 167 | pyproject.toml description + requires-python + dep groups + package layout | 2.3 | 2.2 | 0.827 |
| walker |  | 830 | 53 | [package] in pyproject.toml |  |  | 0.832 |
| ns | 1120 |  | 321 | README example: spec + impls | 2.4 |  | 0.695 |
| ns | 1333 |  | 213 | README example: register + call + output | 2.5 | 2.4 | 0.634 |
| walker |  | 1376 | 546 | README.rst section #0 |  |  | 0.884 |
| walker |  | 1379 | 3 | listing of 'docs/_static' |  |  | 0.884 |
| walker |  | 1383 | 4 | listing of 'docs/_static/img' |  |  | 0.884 |
| walker |  | 1449 | 66 | python decl body at src/pluggy/__init__.py:32 body 33 |  |  | 0.884 |
| walker |  | 1530 | 81 | python decl names surface in src/pluggy/_result.py |  |  | 0.884 |
| walker |  | 1530 | 0 | python decl at src/pluggy/_result.py:20 |  |  | 0.884 |
| walker |  | 1541 | 11 | python decl at src/pluggy/_result.py:24 |  |  | 0.825 |
| ns | 1541 |  | 208 | _hooks.py: marker + caller class/fn name surface | 3.1 |  | 0.825 |
| walker |  | 1550 | 9 | python decl doc at src/pluggy/_result.py:20 |  |  | 0.825 |
| walker |  | 1572 | 22 | python class body at src/pluggy/_result.py:24 |  |  | 0.825 |
| walker |  | 1604 | 32 | python decl doc at src/pluggy/_result.py:24 |  |  | 0.825 |
| walker |  | 1699 | 95 | python method sigs in src/pluggy/_result.py |  |  | 0.826 |
| walker |  | 1699 | 0 | python method at src/pluggy/_result.py:67 |  |  | 0.826 |
| walker |  | 1699 | 0 | python method at src/pluggy/_result.py:80 |  |  | 0.826 |
| walker |  | 1699 | 0 | python method at src/pluggy/_result.py:91 |  |  | 0.826 |
| walker |  | 1714 | 15 | python method at src/pluggy/_result.py:51 |  |  | 0.826 |
| walker |  | 1731 | 17 | python method at src/pluggy/_result.py:42 |  |  | 0.826 |
| walker |  | 1755 | 24 | python method at src/pluggy/_result.py:56 |  |  | 0.827 |
| walker |  | 1767 | 12 | python method doc at src/pluggy/_result.py:42 |  |  | 0.827 |
| walker |  | 1779 | 12 | python method doc at src/pluggy/_result.py:51 |  |  | 0.827 |
| walker |  | 1787 | 8 | python method body at src/pluggy/_result.py:51 body 54 |  |  | 0.827 |
| walker |  | 1799 | 12 | python method doc at src/pluggy/_result.py:56 |  |  | 0.827 |
| walker |  | 1839 | 40 | python method at src/pluggy/_result.py:31 |  |  | 0.827 |
| ns | 1850 |  | 309 | _manager.py: PluginManager + helpers method-name surface | 3.2 |  | 0.758 |
| walker |  | 1851 | 12 | python method doc at src/pluggy/_result.py:31 |  |  | 0.758 |
| walker |  | 1971 | 120 | python decl names surface in src/pluggy/_manager.py |  |  | 0.759 |
| walker |  | 1971 | 0 | python decl at src/pluggy/_manager.py:42 |  |  | 0.759 |
| walker |  | 1971 | 0 | python decl at src/pluggy/_manager.py:52 |  |  | 0.759 |
| walker |  | 1971 | 0 | python decl at src/pluggy/_manager.py:65 |  |  | 0.759 |
| walker |  | 1971 | 0 | python decl at src/pluggy/_manager.py:83 |  |  | 0.759 |
| walker |  | 1971 | 0 | python decl at src/pluggy/_manager.py:525 |  |  | 0.759 |
| walker |  | 1984 | 13 | python decl doc at src/pluggy/_manager.py:65 |  |  | 0.759 |
| walker |  | 2032 | 48 | python decl doc at src/pluggy/_manager.py:52 |  |  | 0.759 |
| walker |  | 2058 | 26 | python decl at src/pluggy/_manager.py:37 |  |  | 0.759 |
| walker |  | 2074 | 16 | python decl body at src/pluggy/_manager.py:525 body 526 |  |  | 0.759 |
| walker |  | 2234 | 160 | python decl names surface in src/pluggy/_hooks.py |  |  | 0.774 |
| walker |  | 2234 | 0 | python decl at src/pluggy/_hooks.py:40 |  |  | 0.774 |
| walker |  | 2234 | 0 | python decl at src/pluggy/_hooks.py:56 |  |  | 0.774 |
| walker |  | 2234 | 0 | python decl at src/pluggy/_hooks.py:281 |  |  | 0.774 |
| walker |  | 2234 | 0 | python decl at src/pluggy/_hooks.py:293 |  |  | 0.774 |
| walker |  | 2234 | 0 | python decl at src/pluggy/_hooks.py:382 |  |  | 0.774 |
| walker |  | 2234 | 0 | python decl at src/pluggy/_hooks.py:593 |  |  | 0.774 |
| walker |  | 2242 | 8 | python decl at src/pluggy/_hooks.py:358 |  |  | 0.777 |
| walker |  | 2250 | 8 | python decl at src/pluggy/_hooks.py:638 |  |  | 0.780 |
| walker |  | 2258 | 8 | python decl at src/pluggy/_hooks.py:696 |  |  | 0.780 |
| walker |  | 2267 | 9 | python decl at src/pluggy/_hooks.py:77 |  |  | 0.784 |
| ns | 2273 |  | 423 | Tutorial: toy-example.py in full | 3.3 |  | 0.697 |
| walker |  | 2276 | 9 | python decl at src/pluggy/_hooks.py:164 |  |  | 0.701 |
| walker |  | 2288 | 12 | python decl doc at src/pluggy/_hooks.py:40 |  |  | 0.701 |
| walker |  | 2300 | 12 | python decl doc at src/pluggy/_hooks.py:56 |  |  | 0.701 |
| walker |  | 2316 | 16 | python class body at src/pluggy/_hooks.py:77 |  |  | 0.701 |
| ns | 2325 |  | 52 | _callers.py top-level function names | 3.4 |  | 0.694 |
| walker |  | 2332 | 16 | python class body at src/pluggy/_hooks.py:164 |  |  | 0.694 |
| walker |  | 2349 | 17 | python decl doc at src/pluggy/_hooks.py:382 |  |  | 0.694 |
| walker |  | 2367 | 18 | python decl doc at src/pluggy/_hooks.py:638 |  |  | 0.694 |
| walker |  | 2398 | 31 | python decl doc at src/pluggy/_hooks.py:358 |  |  | 0.694 |
| ns | 2450 |  | 125 | _result.py class + method surface | 3.5 |  | 0.702 |
| ns | 2483 |  | 33 | _warnings.py classes | 3.6 |  | 0.704 |
| walker |  | 2510 | 112 | python class body at src/pluggy/_hooks.py:638 |  |  | 0.704 |
| walker |  | 2562 | 52 | python class body at src/pluggy/_hooks.py:358 |  |  | 0.704 |
| walker |  | 2650 | 88 | python class body at src/pluggy/_hooks.py:696 |  |  | 0.704 |
| ns | 2667 |  | 184 | HookspecOpts TypedDict fields | 4.1 | 3.1 | 0.680 |
| walker |  | 2742 | 92 | python class body at src/pluggy/_hooks.py:382 |  |  | 0.680 |
| ns | 2915 |  | 248 | HookimplOpts TypedDict fields | 4.2 | 3.1 | 0.652 |
| walker |  | 2966 | 224 | python class body at src/pluggy/_hooks.py:56 |  |  | 0.697 |
| walker |  | 3128 | 162 | python class body at src/pluggy/_hooks.py:40 |  |  | 0.728 |
| ns | 3256 |  | 341 | docs/index.rst section heading map | 4.3 |  | 0.676 |
| walker |  | 3304 | 176 | python method sigs in src/pluggy/_hooks.py |  |  | 0.679 |
| walker |  | 3304 | 0 | python method at src/pluggy/_hooks.py:88 |  |  | 0.679 |
| walker |  | 3304 | 0 | python method at src/pluggy/_hooks.py:175 |  |  | 0.679 |
| walker |  | 3304 | 0 | python method at src/pluggy/_hooks.py:365 |  |  | 0.679 |
| walker |  | 3304 | 0 | python method at src/pluggy/_hooks.py:420 |  |  | 0.679 |
| walker |  | 3314 | 10 | python method doc at src/pluggy/_hooks.py:365 |  |  | 0.679 |
| walker |  | 3324 | 10 | python method body at src/pluggy/_hooks.py:420 body 421 |  |  | 0.679 |
| walker |  | 3336 | 12 | python method body at src/pluggy/_hooks.py:88 body 89 |  |  | 0.679 |
| walker |  | 3348 | 12 | python method body at src/pluggy/_hooks.py:175 body 176 |  |  | 0.679 |
| walker |  | 3389 | 41 | python method at src/pluggy/_hooks.py:424 |  |  | 0.679 |
| walker |  | 3421 | 32 | python decl doc at src/pluggy/_hooks.py:593 |  |  | 0.679 |
| walker |  | 3501 | 80 | python decl doc at src/pluggy/_hooks.py:164 |  |  | 0.679 |
| ns | 3566 |  | 310 | HookCaller._add_hookimpl - the ordering algorithm | 4.4 |  | 0.651 |
| walker |  | 3582 | 81 | python decl doc at src/pluggy/_hooks.py:77 |  |  | 0.651 |
| walker |  | 3652 | 70 | python method at src/pluggy/_hooks.py:393 |  |  | 0.651 |
| walker |  | 3664 | 12 | python method doc at src/pluggy/_hooks.py:393 |  |  | 0.651 |
| walker |  | 3743 | 79 | python decl doc at src/pluggy/_hooks.py:293 |  |  | 0.651 |
| walker |  | 3986 | 243 | python method sigs in src/pluggy/_manager.py |  |  | 0.657 |
| walker |  | 3986 | 0 | python method at src/pluggy/_manager.py:59 |  |  | 0.657 |
| walker |  | 3986 | 0 | python method at src/pluggy/_manager.py:68 |  |  | 0.657 |
| walker |  | 3986 | 0 | python method at src/pluggy/_manager.py:76 |  |  | 0.657 |
| walker |  | 3986 | 0 | python method at src/pluggy/_manager.py:79 |  |  | 0.657 |
| walker |  | 3986 | 0 | python method at src/pluggy/_manager.py:125 |  |  | 0.657 |
| walker |  | 3986 | 0 | python method at src/pluggy/_manager.py:176 |  |  | 0.657 |
| walker |  | 3986 | 0 | python method at src/pluggy/_manager.py:233 |  |  | 0.657 |
| walker |  | 3986 | 0 | python method at src/pluggy/_manager.py:238 |  |  | 0.657 |
| walker |  | 3996 | 10 | python method at src/pluggy/_manager.py:100 |  |  | 0.657 |
| walker |  | 4009 | 13 | python method at src/pluggy/_manager.py:71 |  |  | 0.657 |
| walker |  | 4018 | 9 | python method body at src/pluggy/_manager.py:68 body 69 |  |  | 0.657 |
| walker |  | 4033 | 15 | python method doc at src/pluggy/_manager.py:238 |  |  | 0.657 |
| walker |  | 4068 | 35 | python method at src/pluggy/_manager.py:201 |  |  | 0.657 |
| walker |  | 4086 | 18 | python method doc at src/pluggy/_manager.py:233 |  |  | 0.657 |
| walker |  | 4099 | 13 | python method body at src/pluggy/_manager.py:76 body 77 |  |  | 0.657 |
| walker |  | 4118 | 19 | python method body at src/pluggy/_manager.py:79 body 80 |  |  | 0.657 |
| walker |  | 4138 | 20 | python method body at src/pluggy/_manager.py:71 body 73 |  |  | 0.657 |
| walker |  | 4162 | 24 | python method body at src/pluggy/_manager.py:238 body 240 |  |  | 0.657 |
| walker |  | 4246 | 84 | python method at src/pluggy/_hooks.py:91 |  |  | 0.657 |
| walker |  | 4255 | 9 | python method body at src/pluggy/_hooks.py:91 body 99 |  |  | 0.657 |
| walker |  | 4317 | 62 | listing of 'testing' |  |  | 0.658 |
| walker |  | 4348 | 31 | python method body at src/pluggy/_manager.py:59 body 60 |  |  | 0.658 |
| walker |  | 4409 | 61 | python method doc at src/pluggy/_result.py:80 |  |  | 0.658 |
| walker |  | 4501 | 92 | python method at src/pluggy/_hooks.py:101 |  |  | 0.658 |
| walker |  | 4515 | 14 | python method body at src/pluggy/_hooks.py:101 body 109 |  |  | 0.658 |
| walker |  | 4578 | 63 | python method doc at src/pluggy/_result.py:91 |  |  | 0.658 |
| ns | 4587 |  | 1021 | _multicall body - the call loop | 4.5 | 3.4 | 0.573 |
| walker |  | 4675 | 97 | python method at src/pluggy/_hooks.py:111 |  |  | 0.573 |
| walker |  | 4774 | 99 | python method at src/pluggy/_hooks.py:178 |  |  | 0.573 |
| walker |  | 4783 | 9 | python method body at src/pluggy/_hooks.py:178 body 188 |  |  | 0.573 |
| walker |  | 4850 | 67 | python decl names surface in src/pluggy/_tracing.py |  |  | 0.573 |
| walker |  | 4850 | 0 | python decl at src/pluggy/_tracing.py:16 |  |  | 0.573 |
| walker |  | 4850 | 0 | python decl at src/pluggy/_tracing.py:59 |  |  | 0.573 |
| walker |  | 4957 | 107 | python method at src/pluggy/_hooks.py:190 |  |  | 0.573 |
| walker |  | 4971 | 14 | python method body at src/pluggy/_hooks.py:190 body 200 |  |  | 0.573 |
| walker |  | 5083 | 112 | python method at src/pluggy/_hooks.py:202 |  |  | 0.573 |
| ns | 5248 |  | 661 | PluginManager.register body | 4.6 | 3.2 | 0.538 |
| walker |  | 5293 | 210 | python method sigs in src/pluggy/_tracing.py |  |  | 0.538 |
| walker |  | 5293 | 0 | python method at src/pluggy/_tracing.py:17 |  |  | 0.538 |
| walker |  | 5293 | 0 | python method at src/pluggy/_tracing.py:22 |  |  | 0.538 |
| walker |  | 5293 | 0 | python method at src/pluggy/_tracing.py:25 |  |  | 0.538 |
| walker |  | 5293 | 0 | python method at src/pluggy/_tracing.py:42 |  |  | 0.538 |
| walker |  | 5293 | 0 | python method at src/pluggy/_tracing.py:48 |  |  | 0.538 |
| walker |  | 5293 | 0 | python method at src/pluggy/_tracing.py:51 |  |  | 0.538 |
| walker |  | 5293 | 0 | python method at src/pluggy/_tracing.py:60 |  |  | 0.538 |
| walker |  | 5293 | 0 | python method at src/pluggy/_tracing.py:64 |  |  | 0.538 |
| walker |  | 5293 | 0 | python method at src/pluggy/_tracing.py:67 |  |  | 0.538 |
| walker |  | 5302 | 9 | python method body at src/pluggy/_tracing.py:48 body 49 |  |  | 0.538 |
| walker |  | 5314 | 12 | python method body at src/pluggy/_tracing.py:22 body 23 |  |  | 0.538 |
| walker |  | 5327 | 13 | python method body at src/pluggy/_tracing.py:64 body 65 |  |  | 0.538 |
| walker |  | 5344 | 17 | python method body at src/pluggy/_tracing.py:67 body 68 |  |  | 0.538 |
| walker |  | 5362 | 18 | python method body at src/pluggy/_tracing.py:60 body 61 |  |  | 0.538 |
| walker |  | 5366 | 4 | listing of '.claude' |  |  | 0.538 |
| walker |  | 5381 | 15 | listing of 'docs/examples' |  |  | 0.538 |
| ns | 5454 |  | 206 | HookCaller.__call__ body | 4.7 | 3.1 | 0.528 |
| walker |  | 5458 | 77 | python decl body at src/pluggy/_hooks.py:281 body 282 |  |  | 0.528 |
| walker |  | 5545 | 87 | python method doc at src/pluggy/_manager.py:201 |  |  | 0.528 |
| walker |  | 5625 | 80 | python decl names surface in src/pluggy/_callers.py |  |  | 0.535 |
| walker |  | 5656 | 31 | python decl at src/pluggy/_callers.py:27 |  |  | 0.535 |
| walker |  | 5687 | 31 | python decl doc at src/pluggy/_callers.py:27 |  |  | 0.535 |
| walker |  | 5716 | 29 | python decl at src/pluggy/_callers.py:70 |  |  | 0.535 |
| walker |  | 5750 | 34 | python decl at src/pluggy/_callers.py:60 |  |  | 0.535 |
| ns | 5775 |  | 321 | HookCaller.call_historic body | 4.8 | 3.1 | 0.520 |
| walker |  | 5939 | 189 | python decl doc at src/pluggy/_manager.py:83 |  |  | 0.520 |
| walker |  | 6128 | 189 | python decl doc at src/pluggy/_warnings.py:10 |  |  | 0.520 |
| ns | 6215 |  | 440 | HookCaller.call_extra body | 4.9 | 3.1 | 0.502 |
| walker |  | 6364 | 236 | python method sigs #1 in src/pluggy/_hooks.py |  |  | 0.517 |
| walker |  | 6364 | 0 | python method at src/pluggy/_hooks.py:438 |  |  | 0.517 |
| walker |  | 6364 | 0 | python method at src/pluggy/_hooks.py:442 |  |  | 0.517 |
| walker |  | 6364 | 0 | python method at src/pluggy/_hooks.py:449 |  |  | 0.517 |
| walker |  | 6364 | 0 | python method at src/pluggy/_hooks.py:453 |  |  | 0.517 |
| walker |  | 6364 | 0 | python method at src/pluggy/_hooks.py:477 |  |  | 0.517 |
| walker |  | 6364 | 0 | python method at src/pluggy/_hooks.py:499 |  |  | 0.517 |
| walker |  | 6364 | 0 | python method at src/pluggy/_hooks.py:577 |  |  | 0.517 |
| walker |  | 6364 | 0 | python method at src/pluggy/_hooks.py:612 |  |  | 0.517 |
| walker |  | 6379 | 15 | python method doc at src/pluggy/_hooks.py:449 |  |  | 0.517 |
| walker |  | 6410 | 31 | python method at src/pluggy/_hooks.py:543 |  |  | 0.517 |
| walker |  | 6421 | 11 | python method body at src/pluggy/_hooks.py:449 body 451 |  |  | 0.517 |
| walker |  | 6439 | 18 | python method doc at src/pluggy/_hooks.py:438 |  |  | 0.517 |
| walker |  | 6451 | 12 | python method body at src/pluggy/_hooks.py:438 body 440 |  |  | 0.517 |
| walker |  | 6468 | 17 | python method at src/pluggy/_hooks.py:480 |  |  | 0.517 |
| walker |  | 6483 | 15 | python method body at src/pluggy/_hooks.py:477 body 478 |  |  | 0.517 |
| walker |  | 6501 | 18 | python method at src/pluggy/_hooks.py:618 |  |  | 0.517 |
| walker |  | 6553 | 52 | python method at src/pluggy/_hooks.py:516 |  |  | 0.518 |
| walker |  | 6567 | 14 | python method doc at src/pluggy/_hooks.py:453 |  |  | 0.519 |
| walker |  | 6612 | 45 | python method doc at src/pluggy/_hooks.py:543 |  |  | 0.520 |
| walker |  | 6633 | 21 | python method doc at src/pluggy/_hooks.py:577 |  |  | 0.520 |
| walker |  | 6710 | 77 | python method doc at src/pluggy/_hooks.py:499 |  |  | 0.525 |
| walker |  | 6808 | 98 | python method doc at src/pluggy/_hooks.py:516 |  |  | 0.533 |
| ns | 6884 |  | 669 | PluginManager._verify_hook body - validation rules | 4.10 |  | 0.509 |
| ns | 6938 |  | 54 | docs/examples listing (eggsample + eggsample-spam) | 5.1 |  | 0.504 |
| walker |  | 7053 | 245 | python method sigs #1 in src/pluggy/_manager.py |  |  | 0.522 |
| walker |  | 7053 | 0 | python method at src/pluggy/_manager.py:242 |  |  | 0.522 |
| walker |  | 7053 | 0 | python method at src/pluggy/_manager.py:252 |  |  | 0.522 |
| walker |  | 7053 | 0 | python method at src/pluggy/_manager.py:296 |  |  | 0.522 |
| walker |  | 7053 | 0 | python method at src/pluggy/_manager.py:300 |  |  | 0.522 |
| walker |  | 7053 | 0 | python method at src/pluggy/_manager.py:304 |  |  | 0.522 |
| walker |  | 7053 | 0 | python method at src/pluggy/_manager.py:315 |  |  | 0.522 |
| walker |  | 7053 | 0 | python method at src/pluggy/_manager.py:319 |  |  | 0.522 |
| walker |  | 7053 | 0 | python method at src/pluggy/_manager.py:323 |  |  | 0.522 |
| walker |  | 7053 | 0 | python method at src/pluggy/_manager.py:331 |  |  | 0.522 |
| walker |  | 7053 | 0 | python method at src/pluggy/_manager.py:379 |  |  | 0.522 |
| walker |  | 7053 | 0 | python method at src/pluggy/_manager.py:395 |  |  | 0.522 |
| walker |  | 7067 | 14 | python method doc at src/pluggy/_manager.py:300 |  |  | 0.522 |
| walker |  | 7082 | 15 | python method doc at src/pluggy/_manager.py:296 |  |  | 0.522 |
| walker |  | 7113 | 31 | python method at src/pluggy/_manager.py:278 |  |  | 0.522 |
| walker |  | 7130 | 17 | python method doc at src/pluggy/_manager.py:319 |  |  | 0.522 |
| walker |  | 7148 | 18 | python method doc at src/pluggy/_manager.py:315 |  |  | 0.522 |
| walker |  | 7160 | 12 | python method body at src/pluggy/_manager.py:315 body 317 |  |  | 0.522 |
| walker |  | 7173 | 13 | python method body at src/pluggy/_manager.py:319 body 321 |  |  | 0.522 |
| walker |  | 7202 | 29 | python method doc at src/pluggy/_manager.py:323 |  |  | 0.522 |
| ns | 7214 |  | 276 | eggsample hookspecs | 5.2 |  | 0.510 |
| walker |  | 7220 | 18 | python method body at src/pluggy/_manager.py:300 body 302 |  |  | 0.510 |
| walker |  | 7255 | 35 | python method doc at src/pluggy/_manager.py:242 |  |  | 0.510 |
| walker |  | 7277 | 22 | python method body at src/pluggy/_manager.py:296 body 298 |  |  | 0.510 |
| walker |  | 7321 | 44 | python method doc at src/pluggy/_manager.py:379 |  |  | 0.510 |
| walker |  | 7384 | 63 | python method doc at src/pluggy/_manager.py:252 |  |  | 0.510 |
| walker |  | 7392 | 8 | python method body at src/pluggy/_manager.py:242 body 250 |  |  | 0.510 |
| walker |  | 7400 | 8 | python method body at src/pluggy/_manager.py:323 body 329 |  |  | 0.510 |
| ns | 7474 |  | 260 | eggsample host wiring + main() | 5.3 | 5.2 | 0.498 |
| walker |  | 7497 | 97 | python method doc at src/pluggy/_manager.py:304 |  |  | 0.498 |
| walker |  | 7596 | 99 | python method doc at src/pluggy/_result.py:67 |  |  | 0.498 |
| walker |  | 7606 | 10 | python method body at src/pluggy/_manager.py:233 body 235 |  |  | 0.498 |
| walker |  | 7709 | 103 | python method doc at src/pluggy/_manager.py:395 |  |  | 0.498 |
| ns | 7722 |  | 248 | eggsample-spam plugin code | 5.4 |  | 0.489 |
| ns | 7915 |  | 193 | eggsample + eggsample-spam setup.py: entry-point wiring | 5.5 |  | 0.482 |
| walker |  | 7925 | 216 | python class body at src/pluggy/_hooks.py:593 |  |  | 0.482 |
| ns | 7977 |  | 62 | testing/ directory listing | 5.6 |  | 0.490 |
| walker |  | 8078 | 153 | python decl names surface #1 in src/pluggy/_hooks.py |  |  | 0.490 |
| walker |  | 8102 | 24 | python decl at src/pluggy/_hooks.py:377 |  |  | 0.490 |
| walker |  | 8136 | 34 | python decl at src/pluggy/_hooks.py:33 |  |  | 0.490 |
| walker |  | 8195 | 59 | python decl at src/pluggy/_callers.py:82 |  |  | 0.490 |
| walker |  | 8206 | 11 | python method body at src/pluggy/_result.py:42 body 45 |  |  | 0.490 |
| ns | 8225 |  | 248 | Shared test fixtures (conftest.py) | 5.7 |  | 0.481 |
| ns | 8257 |  | 32 | docs/ listing | 5.8 |  | 0.486 |
| walker |  | 8296 | 90 | python method sigs #2 in src/pluggy/_hooks.py |  |  | 0.486 |
| walker |  | 8296 | 0 | python method at src/pluggy/_hooks.py:634 |  |  | 0.486 |
| walker |  | 8296 | 0 | python method at src/pluggy/_hooks.py:692 |  |  | 0.486 |
| walker |  | 8296 | 0 | python method at src/pluggy/_hooks.py:709 |  |  | 0.486 |
| walker |  | 8319 | 23 | python method at src/pluggy/_hooks.py:626 |  |  | 0.486 |
| walker |  | 8328 | 9 | python method body at src/pluggy/_hooks.py:626 body 628 |  |  | 0.486 |
| walker |  | 8345 | 17 | python method body at src/pluggy/_hooks.py:634 body 635 |  |  | 0.486 |
| walker |  | 8408 | 63 | python method at src/pluggy/_hooks.py:656 |  |  | 0.486 |
| walker |  | 8420 | 12 | python method doc at src/pluggy/_hooks.py:656 |  |  | 0.486 |
| walker |  | 8446 | 26 | python method at src/pluggy/_hooks.py:630 |  |  | 0.486 |
| walker |  | 8457 | 11 | python method body at src/pluggy/_hooks.py:630 body 632 |  |  | 0.486 |
| walker |  | 8482 | 25 | python method body at src/pluggy/_hooks.py:692 body 693 |  |  | 0.486 |
| walker |  | 8494 | 12 | python method body at src/pluggy/_manager.py:304 body 313 |  |  | 0.486 |
| walker |  | 8603 | 109 | python imports in src/pluggy/_result.py |  |  | 0.486 |
| ns | 8607 |  | 350 | HookCaller.__init__ + the hookimpls-list layout comment | 5.9 | 3.1 | 0.479 |
| walker |  | 8738 | 135 | python method doc at src/pluggy/_manager.py:176 |  |  | 0.479 |
| walker |  | 8794 | 56 | python decl doc at src/pluggy/_callers.py:82 |  |  | 0.479 |
| walker |  | 8931 | 137 | python method doc at src/pluggy/_manager.py:125 |  |  | 0.483 |
| walker |  | 8944 | 13 | python method body at src/pluggy/_manager.py:233 body 236 |  |  | 0.483 |
| ns | 9039 |  | 432 | test_pluginmanager.py: test function name list | 5.10 |  | 0.472 |
| walker |  | 9090 | 146 | python method doc at src/pluggy/_manager.py:278 |  |  | 0.472 |
| walker |  | 9152 | 62 | python method body at src/pluggy/_hooks.py:612 body 613 |  |  | 0.472 |
| ns | 9192 |  | 153 | CHANGELOG.rst recent-release headers | 6.1 |  | 0.470 |
| walker |  | 9219 | 67 | python imports in src/pluggy/_tracing.py |  |  | 0.470 |
| walker |  | 9227 | 8 | listing of 'docs/examples/eggsample' |  |  | 0.471 |
| walker |  | 9247 | 20 | listing of 'docs/examples/eggsample/eggsample' |  |  | 0.477 |
| walker |  | 9257 | 10 | python imports in docs/examples/eggsample/eggsample/__init__.py |  |  | 0.477 |
| walker |  | 9377 | 120 | python method sigs #2 in src/pluggy/_manager.py |  |  | 0.491 |
| walker |  | 9377 | 0 | python method at src/pluggy/_manager.py:425 |  |  | 0.491 |
| walker |  | 9377 | 0 | python method at src/pluggy/_manager.py:430 |  |  | 0.491 |
| walker |  | 9377 | 0 | python method at src/pluggy/_manager.py:434 |  |  | 0.491 |
| walker |  | 9377 | 0 | python method at src/pluggy/_manager.py:487 |  |  | 0.491 |
| ns | 9403 |  | 211 | Test files: per-file test-fn name lists (smaller files) | 6.2 |  | 0.485 |
| walker |  | 9405 | 28 | python method at src/pluggy/_manager.py:512 |  |  | 0.485 |
| walker |  | 9436 | 31 | python method at src/pluggy/_manager.py:450 |  |  | 0.485 |
| walker |  | 9449 | 13 | python method body at src/pluggy/_manager.py:425 body 428 |  |  | 0.485 |
| walker |  | 9470 | 21 | python method doc at src/pluggy/_manager.py:430 |  |  | 0.485 |
| walker |  | 9482 | 12 | python method body at src/pluggy/_manager.py:430 body 432 |  |  | 0.485 |
| walker |  | 9510 | 28 | python method doc at src/pluggy/_manager.py:425 |  |  | 0.485 |
| walker |  | 9552 | 42 | python method doc at src/pluggy/_manager.py:487 |  |  | 0.485 |
| walker |  | 9602 | 50 | python method doc at src/pluggy/_manager.py:512 |  |  | 0.485 |
| walker |  | 9667 | 65 | python method doc at src/pluggy/_manager.py:434 |  |  | 0.485 |
| walker |  | 9846 | 179 | python method doc at src/pluggy/_manager.py:450 |  |  | 0.485 |
| ns | 9847 |  | 444 | api_reference.rst - autoclass directives | 6.3 |  | 0.470 |
| walker |  | 9895 | 49 | python method body at src/pluggy/_tracing.py:17 body 18 |  |  | 0.470 |
| walker |  | 9912 | 17 | python method body at src/pluggy/_manager.py:304 body 312 |  |  | 0.470 |
| walker |  | 9930 | 18 | python decl names surface in docs/examples/eggsample/eggsample/__init__.py |  |  | 0.470 |
| ns | 9940 |  | 93 | downstream/ + scripts/ + .github/ + changelog/ listings | 6.4 |  | 0.472 |
| ns | 10250 |  | 310 | load_setuptools_entrypoints body | 6.5 | 3.2 | 0.468 |
