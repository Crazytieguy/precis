Score(3000)=0.587 I=0.833 C=0.414 ns_rows≤3K=17/40 (reached=8 partial=2 missing=7)

| source | ns_cum | walker_cum | marginal | descriptor | id | predecessor | Score(B=cum) |
|:-------|-------:|-----------:|---------:|:-----------|:---|:------------|-------------:|
| ns | 54 |  | 54 | README headline + tagline | 1.1 |  | 0.000 |
| walker |  | 96 | 96 | listing of '.' |  |  | 0.000 |
| walker |  | 100 | 4 | listing of 'src' |  |  | 0.000 |
| ns | 150 |  | 96 | Top-level fs listing | 1.2 |  | 0.639 |
| ns | 193 |  | 43 | src/pluggy listing | 1.3 |  | 0.550 |
| walker |  | 202 | 102 | README headline in README.rst |  |  | 0.860 |
| walker |  | 245 | 43 | listing of 'src/pluggy' |  |  | 1.000 |
| ns | 343 |  | 150 | Public API surface (__all__) | 1.4 |  | 0.802 |
| ns | 509 |  | 166 | __init__ re-export map (which file each symbol comes from) | 2.1 |  | 0.702 |
| walker |  | 561 | 316 | python imports in src/pluggy/__init__.py |  |  | 1.000 |
| walker |  | 579 | 18 | python decl names surface in src/pluggy/__init__.py |  |  | 1.000 |
| walker |  | 579 | 0 | python decl at src/pluggy/__init__.py:32 |  |  | 1.000 |
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
| ns | 2171 |  | 423 | Tutorial: toy-example.py in full | 3.3 |  | 0.615 |
| ns | 2213 |  | 42 | _callers.py top-level function names | 3.4 |  | 0.621 |
| walker |  | 2220 | 210 | python method sigs in src/pluggy/_tracing.py |  |  | 0.621 |
| walker |  | 2220 | 0 | python method at src/pluggy/_tracing.py:17 |  |  | 0.621 |
| walker |  | 2220 | 0 | python method at src/pluggy/_tracing.py:22 |  |  | 0.621 |
| walker |  | 2220 | 0 | python method at src/pluggy/_tracing.py:25 |  |  | 0.621 |
| walker |  | 2220 | 0 | python method at src/pluggy/_tracing.py:42 |  |  | 0.621 |
| walker |  | 2220 | 0 | python method at src/pluggy/_tracing.py:48 |  |  | 0.621 |
| walker |  | 2220 | 0 | python method at src/pluggy/_tracing.py:51 |  |  | 0.621 |
| walker |  | 2220 | 0 | python method at src/pluggy/_tracing.py:60 |  |  | 0.621 |
| walker |  | 2220 | 0 | python method at src/pluggy/_tracing.py:64 |  |  | 0.621 |
| walker |  | 2220 | 0 | python method at src/pluggy/_tracing.py:67 |  |  | 0.621 |
| walker |  | 2229 | 9 | python method body at src/pluggy/_tracing.py:48 body 49 |  |  | 0.621 |
| walker |  | 2241 | 12 | python method body at src/pluggy/_tracing.py:22 body 23 |  |  | 0.621 |
| walker |  | 2254 | 13 | python method body at src/pluggy/_tracing.py:64 body 65 |  |  | 0.621 |
| walker |  | 2271 | 17 | python method body at src/pluggy/_tracing.py:67 body 68 |  |  | 0.621 |
| walker |  | 2289 | 18 | python method body at src/pluggy/_tracing.py:60 body 61 |  |  | 0.621 |
| walker |  | 2293 | 4 | listing of '.claude' |  |  | 0.621 |
| walker |  | 2308 | 15 | listing of 'docs/examples' |  |  | 0.621 |
| ns | 2320 |  | 107 | _result.py class + method surface | 3.5 |  | 0.631 |
| ns | 2347 |  | 27 | _warnings.py classes | 3.6 |  | 0.634 |
| walker |  | 2357 | 49 | python method doc at src/pluggy/_result.py:80 |  |  | 0.634 |
| walker |  | 2413 | 56 | python method doc at src/pluggy/_result.py:91 |  |  | 0.634 |
| walker |  | 2470 | 57 | python decl at src/pluggy/_callers.py:82 |  |  | 0.634 |
| ns | 2533 |  | 186 | HookspecOpts TypedDict fields | 4.1 | 3.1 | 0.612 |
| walker |  | 2586 | 116 | python decl names surface in src/pluggy/_manager.py |  |  | 0.613 |
| walker |  | 2586 | 0 | python decl at src/pluggy/_manager.py:42 |  |  | 0.613 |
| walker |  | 2586 | 0 | python decl at src/pluggy/_manager.py:52 |  |  | 0.613 |
| walker |  | 2586 | 0 | python decl at src/pluggy/_manager.py:65 |  |  | 0.613 |
| walker |  | 2586 | 0 | python decl at src/pluggy/_manager.py:83 |  |  | 0.613 |
| walker |  | 2586 | 0 | python decl at src/pluggy/_manager.py:525 |  |  | 0.613 |
| walker |  | 2597 | 11 | python decl doc at src/pluggy/_manager.py:65 |  |  | 0.613 |
| walker |  | 2638 | 41 | python decl doc at src/pluggy/_manager.py:52 |  |  | 0.613 |
| walker |  | 2654 | 16 | python decl body at src/pluggy/_manager.py:525 body 526 |  |  | 0.613 |
| walker |  | 2682 | 28 | python decl at src/pluggy/_manager.py:37 |  |  | 0.613 |
| ns | 2781 |  | 248 | HookimplOpts TypedDict fields | 4.2 | 3.1 | 0.587 |
| ns | 3060 |  | 279 | docs/index.rst section heading map | 4.3 |  | 0.546 |
| walker |  | 3296 | 614 | python method sigs in src/pluggy/_manager.py |  |  | 0.611 |
| walker |  | 3296 | 0 | python method at src/pluggy/_manager.py:59 |  |  | 0.611 |
| walker |  | 3296 | 0 | python method at src/pluggy/_manager.py:68 |  |  | 0.611 |
| walker |  | 3296 | 0 | python method at src/pluggy/_manager.py:76 |  |  | 0.611 |
| walker |  | 3296 | 0 | python method at src/pluggy/_manager.py:79 |  |  | 0.611 |
| walker |  | 3296 | 0 | python method at src/pluggy/_manager.py:125 |  |  | 0.611 |
| walker |  | 3296 | 0 | python method at src/pluggy/_manager.py:176 |  |  | 0.611 |
| walker |  | 3296 | 0 | python method at src/pluggy/_manager.py:233 |  |  | 0.611 |
| walker |  | 3296 | 0 | python method at src/pluggy/_manager.py:238 |  |  | 0.611 |
| walker |  | 3296 | 0 | python method at src/pluggy/_manager.py:242 |  |  | 0.611 |
| walker |  | 3296 | 0 | python method at src/pluggy/_manager.py:252 |  |  | 0.611 |
| walker |  | 3296 | 0 | python method at src/pluggy/_manager.py:296 |  |  | 0.611 |
| walker |  | 3296 | 0 | python method at src/pluggy/_manager.py:300 |  |  | 0.611 |
| walker |  | 3296 | 0 | python method at src/pluggy/_manager.py:304 |  |  | 0.611 |
| walker |  | 3296 | 0 | python method at src/pluggy/_manager.py:315 |  |  | 0.611 |
| walker |  | 3296 | 0 | python method at src/pluggy/_manager.py:319 |  |  | 0.611 |
| walker |  | 3296 | 0 | python method at src/pluggy/_manager.py:323 |  |  | 0.611 |
| walker |  | 3296 | 0 | python method at src/pluggy/_manager.py:331 |  |  | 0.611 |
| walker |  | 3296 | 0 | python method at src/pluggy/_manager.py:379 |  |  | 0.611 |
| walker |  | 3296 | 0 | python method at src/pluggy/_manager.py:395 |  |  | 0.611 |
| walker |  | 3296 | 0 | python method at src/pluggy/_manager.py:425 |  |  | 0.611 |
| walker |  | 3296 | 0 | python method at src/pluggy/_manager.py:430 |  |  | 0.611 |
| walker |  | 3296 | 0 | python method at src/pluggy/_manager.py:434 |  |  | 0.611 |
| walker |  | 3296 | 0 | python method at src/pluggy/_manager.py:487 |  |  | 0.611 |
| walker |  | 3304 | 8 | python method at src/pluggy/_manager.py:100 |  |  | 0.611 |
| walker |  | 3315 | 11 | python method at src/pluggy/_manager.py:71 |  |  | 0.611 |
| walker |  | 3327 | 12 | python method doc at src/pluggy/_manager.py:300 |  |  | 0.611 |
| walker |  | 3353 | 26 | python method at src/pluggy/_manager.py:512 |  |  | 0.611 |
| walker |  | 3362 | 9 | python method body at src/pluggy/_manager.py:68 body 69 |  |  | 0.611 |
| ns | 3368 |  | 308 | HookCaller._add_hookimpl - the ordering algorithm | 4.4 |  | 0.586 |
| walker |  | 3375 | 13 | python method doc at src/pluggy/_manager.py:238 |  |  | 0.586 |
| walker |  | 3388 | 13 | python method doc at src/pluggy/_manager.py:296 |  |  | 0.586 |
| walker |  | 3417 | 29 | python method at src/pluggy/_manager.py:278 |  |  | 0.586 |
| walker |  | 3446 | 29 | python method at src/pluggy/_manager.py:450 |  |  | 0.586 |
| walker |  | 3461 | 15 | python method doc at src/pluggy/_manager.py:319 |  |  | 0.586 |
| walker |  | 3477 | 16 | python method doc at src/pluggy/_manager.py:233 |  |  | 0.586 |
| walker |  | 3493 | 16 | python method doc at src/pluggy/_manager.py:315 |  |  | 0.586 |
| walker |  | 3526 | 33 | python method at src/pluggy/_manager.py:201 |  |  | 0.586 |
| walker |  | 3545 | 19 | python method doc at src/pluggy/_manager.py:430 |  |  | 0.586 |
| walker |  | 3558 | 13 | python method body at src/pluggy/_manager.py:76 body 77 |  |  | 0.586 |
| walker |  | 3571 | 13 | python method body at src/pluggy/_manager.py:425 body 428 |  |  | 0.586 |
| walker |  | 3585 | 14 | python method body at src/pluggy/_manager.py:315 body 317 |  |  | 0.586 |
| walker |  | 3599 | 14 | python method body at src/pluggy/_manager.py:430 body 432 |  |  | 0.586 |
| walker |  | 3614 | 15 | python method body at src/pluggy/_manager.py:319 body 321 |  |  | 0.586 |
| walker |  | 3641 | 27 | python method doc at src/pluggy/_manager.py:323 |  |  | 0.586 |
| walker |  | 3669 | 28 | python method doc at src/pluggy/_manager.py:242 |  |  | 0.586 |
| walker |  | 3697 | 28 | python method doc at src/pluggy/_manager.py:425 |  |  | 0.586 |
| walker |  | 3716 | 19 | python method body at src/pluggy/_manager.py:79 body 80 |  |  | 0.586 |
| walker |  | 3736 | 20 | python method body at src/pluggy/_manager.py:300 body 302 |  |  | 0.586 |
| walker |  | 3771 | 35 | python method doc at src/pluggy/_manager.py:487 |  |  | 0.586 |
| walker |  | 3793 | 22 | python method body at src/pluggy/_manager.py:71 body 73 |  |  | 0.586 |
| walker |  | 3835 | 42 | python method doc at src/pluggy/_manager.py:379 |  |  | 0.586 |
| walker |  | 3859 | 24 | python method body at src/pluggy/_manager.py:238 body 240 |  |  | 0.586 |
| walker |  | 3883 | 24 | python method body at src/pluggy/_manager.py:296 body 298 |  |  | 0.586 |
| walker |  | 3933 | 50 | python method doc at src/pluggy/_manager.py:512 |  |  | 0.586 |
| walker |  | 3989 | 56 | python method doc at src/pluggy/_manager.py:252 |  |  | 0.586 |
| walker |  | 4047 | 58 | python method doc at src/pluggy/_manager.py:434 |  |  | 0.586 |
| walker |  | 4078 | 31 | python method body at src/pluggy/_manager.py:59 body 60 |  |  | 0.586 |
| walker |  | 4129 | 51 | python decl doc at src/pluggy/_callers.py:82 |  |  | 0.586 |
| walker |  | 4188 | 59 | python imports in src/pluggy/_tracing.py |  |  | 0.586 |
| walker |  | 4196 | 8 | python method body at src/pluggy/_manager.py:242 body 250 |  |  | 0.586 |
| walker |  | 4204 | 8 | python method body at src/pluggy/_manager.py:323 body 329 |  |  | 0.586 |
| walker |  | 4281 | 77 | python method doc at src/pluggy/_manager.py:201 |  |  | 0.586 |
| ns | 4391 |  | 1023 | _multicall body - the call loop | 4.5 | 3.4 | 0.512 |
| walker |  | 4462 | 181 | python decl names surface in src/pluggy/_hooks.py |  |  | 0.514 |
| walker |  | 4462 | 0 | python decl at src/pluggy/_hooks.py:40 |  |  | 0.514 |
| walker |  | 4462 | 0 | python decl at src/pluggy/_hooks.py:56 |  |  | 0.514 |
| walker |  | 4462 | 0 | python decl at src/pluggy/_hooks.py:281 |  |  | 0.514 |
| walker |  | 4469 | 7 | python decl at src/pluggy/_hooks.py:77 |  |  | 0.515 |
| walker |  | 4476 | 7 | python decl at src/pluggy/_hooks.py:164 |  |  | 0.516 |
| walker |  | 4486 | 10 | python decl doc at src/pluggy/_hooks.py:40 |  |  | 0.516 |
| walker |  | 4496 | 10 | python decl doc at src/pluggy/_hooks.py:56 |  |  | 0.517 |
| walker |  | 4510 | 14 | python class body at src/pluggy/_hooks.py:77 |  |  | 0.517 |
| walker |  | 4524 | 14 | python class body at src/pluggy/_hooks.py:164 |  |  | 0.517 |
| walker |  | 4750 | 226 | python class body at src/pluggy/_hooks.py:56 |  |  | 0.551 |
| walker |  | 4884 | 134 | python method sigs in src/pluggy/_hooks.py |  |  | 0.551 |
| walker |  | 4884 | 0 | python method at src/pluggy/_hooks.py:88 |  |  | 0.551 |
| walker |  | 4884 | 0 | python method at src/pluggy/_hooks.py:175 |  |  | 0.551 |
| walker |  | 4896 | 12 | python method body at src/pluggy/_hooks.py:88 body 89 |  |  | 0.551 |
| walker |  | 4908 | 12 | python method body at src/pluggy/_hooks.py:175 body 176 |  |  | 0.551 |
| ns | 5054 |  | 663 | PluginManager.register body | 4.6 | 3.2 | 0.518 |
| walker |  | 5072 | 164 | python class body at src/pluggy/_hooks.py:40 |  |  | 0.540 |
| walker |  | 5149 | 77 | python decl doc at src/pluggy/_hooks.py:164 |  |  | 0.540 |
| walker |  | 5227 | 78 | python decl doc at src/pluggy/_hooks.py:77 |  |  | 0.540 |
| ns | 5262 |  | 208 | HookCaller.__call__ body | 4.7 | 3.1 | 0.530 |
| walker |  | 5263 | 36 | python decl at src/pluggy/_hooks.py:33 |  |  | 0.530 |
| walker |  | 5345 | 82 | python method at src/pluggy/_hooks.py:91 |  |  | 0.530 |
| walker |  | 5356 | 11 | python method body at src/pluggy/_hooks.py:91 body 99 |  |  | 0.530 |
| walker |  | 5446 | 90 | python method at src/pluggy/_hooks.py:101 |  |  | 0.530 |
| walker |  | 5462 | 16 | python method body at src/pluggy/_hooks.py:101 body 109 |  |  | 0.530 |
| walker |  | 5557 | 95 | python method at src/pluggy/_hooks.py:111 |  |  | 0.530 |
| ns | 5585 |  | 323 | HookCaller.call_historic body | 4.8 | 3.1 | 0.515 |
| walker |  | 5654 | 97 | python method at src/pluggy/_hooks.py:178 |  |  | 0.515 |
| walker |  | 5665 | 11 | python method body at src/pluggy/_hooks.py:178 body 188 |  |  | 0.515 |
| walker |  | 5770 | 105 | python method at src/pluggy/_hooks.py:190 |  |  | 0.515 |
| walker |  | 5786 | 16 | python method body at src/pluggy/_hooks.py:190 body 200 |  |  | 0.515 |
| walker |  | 5896 | 110 | python method at src/pluggy/_hooks.py:202 |  |  | 0.515 |
| ns | 6025 |  | 440 | HookCaller.call_extra body | 4.9 | 3.1 | 0.497 |
| walker |  | 6065 | 169 | python decl doc at src/pluggy/_manager.py:83 |  |  | 0.497 |
| walker |  | 6236 | 171 | python decl doc at src/pluggy/_warnings.py:10 |  |  | 0.497 |
| walker |  | 6244 | 8 | listing of 'docs/examples/eggsample' |  |  | 0.497 |
| walker |  | 6264 | 20 | listing of 'docs/examples/eggsample/eggsample' |  |  | 0.498 |
| walker |  | 6272 | 8 | python imports in docs/examples/eggsample/eggsample/__init__.py |  |  | 0.498 |
| walker |  | 6347 | 75 | python decl body at src/pluggy/_hooks.py:281 body 282 |  |  | 0.498 |
| walker |  | 6434 | 87 | python method doc at src/pluggy/_result.py:67 |  |  | 0.498 |
| walker |  | 6524 | 90 | python method doc at src/pluggy/_manager.py:304 |  |  | 0.498 |
| walker |  | 6615 | 91 | python method doc at src/pluggy/_manager.py:395 |  |  | 0.498 |
| walker |  | 6625 | 10 | python method body at src/pluggy/_manager.py:233 body 235 |  |  | 0.498 |
| walker |  | 6674 | 49 | python method body at src/pluggy/_tracing.py:17 body 18 |  |  | 0.498 |
| walker |  | 6692 | 18 | python decl names surface in docs/examples/eggsample/eggsample/__init__.py |  |  | 0.498 |
| ns | 6694 |  | 669 | PluginManager._verify_hook body - validation rules | 4.10 |  | 0.475 |
| walker |  | 6703 | 11 | python method body at src/pluggy/_result.py:42 body 45 |  |  | 0.475 |
| ns | 6748 |  | 54 | docs/examples listing (eggsample + eggsample-spam) | 5.1 |  | 0.480 |
| walker |  | 6823 | 120 | python method doc at src/pluggy/_manager.py:125 |  |  | 0.483 |
| walker |  | 6936 | 113 | python decl names surface #1 in src/pluggy/_hooks.py |  |  | 0.490 |
| walker |  | 6936 | 0 | python decl at src/pluggy/_hooks.py:293 |  |  | 0.490 |
| walker |  | 6936 | 0 | python decl at src/pluggy/_hooks.py:382 |  |  | 0.490 |
| walker |  | 6936 | 0 | python decl at src/pluggy/_hooks.py:593 |  |  | 0.490 |
| walker |  | 6942 | 6 | python decl at src/pluggy/_hooks.py:358 |  |  | 0.492 |
| walker |  | 6948 | 6 | python decl at src/pluggy/_hooks.py:638 |  |  | 0.493 |
| walker |  | 6954 | 6 | python decl at src/pluggy/_hooks.py:696 |  |  | 0.493 |
| walker |  | 6969 | 15 | python decl doc at src/pluggy/_hooks.py:382 |  |  | 0.493 |
| walker |  | 6987 | 18 | python decl doc at src/pluggy/_hooks.py:638 |  |  | 0.493 |
| walker |  | 7018 | 31 | python decl doc at src/pluggy/_hooks.py:358 |  |  | 0.493 |
| ns | 7024 |  | 276 | eggsample hookspecs | 5.2 |  | 0.482 |
| walker |  | 7063 | 45 | python class body at src/pluggy/_hooks.py:358 |  |  | 0.482 |
| walker |  | 7175 | 112 | python class body at src/pluggy/_hooks.py:638 |  |  | 0.482 |
| walker |  | 7263 | 88 | python class body at src/pluggy/_hooks.py:696 |  |  | 0.482 |
| ns | 7282 |  | 258 | eggsample host wiring + main() | 5.3 | 5.2 | 0.471 |
| walker |  | 7351 | 88 | python class body at src/pluggy/_hooks.py:382 |  |  | 0.471 |
| walker |  | 7377 | 26 | python decl at src/pluggy/_hooks.py:377 |  |  | 0.471 |
| ns | 7530 |  | 248 | eggsample-spam plugin code | 5.4 |  | 0.462 |
| ns | 7723 |  | 193 | eggsample + eggsample-spam setup.py: entry-point wiring | 5.5 |  | 0.455 |
| walker |  | 7761 | 384 | python method sigs #1 in src/pluggy/_hooks.py |  |  | 0.469 |
| walker |  | 7761 | 0 | python method at src/pluggy/_hooks.py:365 |  |  | 0.469 |
| walker |  | 7761 | 0 | python method at src/pluggy/_hooks.py:420 |  |  | 0.469 |
| walker |  | 7761 | 0 | python method at src/pluggy/_hooks.py:438 |  |  | 0.469 |
| walker |  | 7761 | 0 | python method at src/pluggy/_hooks.py:442 |  |  | 0.469 |
| walker |  | 7761 | 0 | python method at src/pluggy/_hooks.py:449 |  |  | 0.469 |
| walker |  | 7761 | 0 | python method at src/pluggy/_hooks.py:453 |  |  | 0.469 |
| walker |  | 7761 | 0 | python method at src/pluggy/_hooks.py:477 |  |  | 0.469 |
| walker |  | 7761 | 0 | python method at src/pluggy/_hooks.py:499 |  |  | 0.469 |
| walker |  | 7761 | 0 | python method at src/pluggy/_hooks.py:577 |  |  | 0.469 |
| walker |  | 7761 | 0 | python method at src/pluggy/_hooks.py:612 |  |  | 0.469 |
| walker |  | 7761 | 0 | python method at src/pluggy/_hooks.py:634 |  |  | 0.469 |
| walker |  | 7761 | 0 | python method at src/pluggy/_hooks.py:692 |  |  | 0.469 |
| walker |  | 7761 | 0 | python method at src/pluggy/_hooks.py:709 |  |  | 0.469 |
| walker |  | 7782 | 21 | python method at src/pluggy/_hooks.py:626 |  |  | 0.469 |
| ns | 7785 |  | 62 | testing/ directory listing | 5.6 |  | 0.478 |
| walker |  | 7792 | 10 | python method doc at src/pluggy/_hooks.py:365 |  |  | 0.478 |
| walker |  | 7805 | 13 | python method doc at src/pluggy/_hooks.py:449 |  |  | 0.478 |
| walker |  | 7834 | 29 | python method at src/pluggy/_hooks.py:543 |  |  | 0.478 |
| walker |  | 7844 | 10 | python method body at src/pluggy/_hooks.py:420 body 421 |  |  | 0.478 |
| walker |  | 7855 | 11 | python method body at src/pluggy/_hooks.py:626 body 628 |  |  | 0.478 |
| walker |  | 7871 | 16 | python method doc at src/pluggy/_hooks.py:438 |  |  | 0.478 |
| walker |  | 7886 | 15 | python method at src/pluggy/_hooks.py:480 |  |  | 0.478 |
| walker |  | 7899 | 13 | python method body at src/pluggy/_hooks.py:449 body 451 |  |  | 0.478 |
| walker |  | 7938 | 39 | python method at src/pluggy/_hooks.py:424 |  |  | 0.478 |
| walker |  | 7954 | 16 | python method at src/pluggy/_hooks.py:618 |  |  | 0.478 |
| walker |  | 7968 | 14 | python method body at src/pluggy/_hooks.py:438 body 440 |  |  | 0.478 |
| walker |  | 7983 | 15 | python method body at src/pluggy/_hooks.py:477 body 478 |  |  | 0.478 |
| walker |  | 7995 | 12 | python method doc at src/pluggy/_hooks.py:453 |  |  | 0.478 |
| walker |  | 8012 | 17 | python method body at src/pluggy/_hooks.py:634 body 635 |  |  | 0.478 |
| ns | 8033 |  | 248 | Shared test fixtures (conftest.py) | 5.7 |  | 0.469 |
| walker |  | 8062 | 50 | python method at src/pluggy/_hooks.py:516 |  |  | 0.470 |
| ns | 8065 |  | 32 | docs/ listing | 5.8 |  | 0.475 |
| walker |  | 8086 | 24 | python method at src/pluggy/_hooks.py:630 |  |  | 0.475 |
| walker |  | 8116 | 30 | python decl doc at src/pluggy/_hooks.py:593 |  |  | 0.475 |
| walker |  | 8177 | 61 | python method at src/pluggy/_hooks.py:656 |  |  | 0.475 |
| walker |  | 8189 | 12 | python method doc at src/pluggy/_hooks.py:656 |  |  | 0.475 |
| walker |  | 8261 | 72 | python decl doc at src/pluggy/_hooks.py:293 |  |  | 0.475 |
| walker |  | 8329 | 68 | python method at src/pluggy/_hooks.py:393 |  |  | 0.475 |
| walker |  | 8341 | 12 | python method doc at src/pluggy/_hooks.py:393 |  |  | 0.475 |
| walker |  | 8360 | 19 | python method doc at src/pluggy/_hooks.py:577 |  |  | 0.475 |
| walker |  | 8405 | 45 | python method doc at src/pluggy/_hooks.py:543 |  |  | 0.477 |
| ns | 8413 |  | 348 | HookCaller.__init__ + the hookimpls-list layout comment | 5.9 | 3.1 | 0.469 |
| walker |  | 8430 | 25 | python method body at src/pluggy/_hooks.py:692 body 693 |  |  | 0.469 |
| walker |  | 8443 | 13 | python method body at src/pluggy/_hooks.py:630 body 632 |  |  | 0.469 |
| walker |  | 8508 | 65 | python method doc at src/pluggy/_hooks.py:499 |  |  | 0.472 |
| walker |  | 8601 | 93 | python method doc at src/pluggy/_hooks.py:516 |  |  | 0.478 |
| ns | 8779 |  | 366 | test_pluginmanager.py: test function name list | 5.10 |  | 0.467 |
| walker |  | 8814 | 213 | python class body at src/pluggy/_hooks.py:593 |  |  | 0.467 |
| ns | 8916 |  | 137 | CHANGELOG.rst recent-release headers | 6.1 |  | 0.465 |
| walker |  | 8937 | 123 | python method doc at src/pluggy/_manager.py:176 |  |  | 0.465 |
| walker |  | 9038 | 101 | python imports in src/pluggy/_result.py |  |  | 0.465 |
| ns | 9083 |  | 167 | Test files: per-file test-fn name lists (smaller files) | 6.2 |  | 0.460 |
| walker |  | 9093 | 55 | python method body at src/pluggy/_tracing.py:51 body 52 |  |  | 0.460 |
| walker |  | 9105 | 12 | python method body at src/pluggy/_manager.py:304 body 313 |  |  | 0.460 |
| walker |  | 9241 | 136 | python method doc at src/pluggy/_manager.py:278 |  |  | 0.460 |
| walker |  | 9254 | 13 | listing of '.github' |  |  | 0.460 |
| walker |  | 9258 | 4 | listing of '.github/workflows' |  |  | 0.460 |
| walker |  | 9320 | 62 | python method body at src/pluggy/_hooks.py:612 body 613 |  |  | 0.460 |
| walker |  | 9334 | 14 | listing of 'scripts' |  |  | 0.460 |
| walker |  | 9498 | 164 | python method doc at src/pluggy/_manager.py:450 |  |  | 0.460 |
| ns | 9525 |  | 442 | api_reference.rst - autoclass directives | 6.3 |  | 0.446 |
| walker |  | 9611 | 113 | README.rst section #1 |  |  | 0.446 |
| ns | 9618 |  | 93 | downstream/ + scripts/ + .github/ + changelog/ listings | 6.4 |  | 0.457 |
| walker |  | 9626 | 15 | python method body at src/pluggy/_manager.py:233 body 236 |  |  | 0.457 |
| walker |  | 9692 | 66 | python decl body at src/pluggy/_callers.py:60 body 64 |  |  | 0.457 |
| walker |  | 9853 | 161 | python imports in src/pluggy/_callers.py |  |  | 0.457 |
| ns | 9930 |  | 312 | load_setuptools_entrypoints body | 6.5 | 3.2 | 0.452 |
| walker |  | 9950 | 97 | python method at src/pluggy/_manager.py:114 |  |  | 0.452 |
| walker |  | 9972 | 22 | python method body at src/pluggy/_manager.py:114 body 123 |  |  | 0.452 |
