Score(3000)=0.679 I=0.855 C=0.540 ns_rows≤3K=17/40 (reached=9 partial=2 missing=6)

| source | ns_cum | walker_cum | marginal | descriptor | id | predecessor | Score(B=cum) |
|:-------|-------:|-----------:|---------:|:-----------|:---|:------------|-------------:|
| ns | 58 |  | 58 | README headline + tagline | 1.1 |  | 0.000 |
| walker |  | 96 | 96 | listing of '.' |  |  | 0.000 |
| walker |  | 100 | 4 | listing of 'src' |  |  | 0.000 |
| ns | 154 |  | 96 | Top-level fs listing | 1.2 |  | 0.639 |
| ns | 197 |  | 43 | src/pluggy listing | 1.3 |  | 0.550 |
| walker |  | 204 | 104 | README headline in README.rst |  |  | 0.860 |
| walker |  | 247 | 43 | listing of 'src/pluggy' |  |  | 1.000 |
| ns | 349 |  | 152 | Public API surface (__all__) | 1.4 |  | 0.802 |
| ns | 515 |  | 166 | __init__ re-export map (which file each symbol comes from) | 2.1 |  | 0.702 |
| walker |  | 565 | 318 | python imports in src/pluggy/__init__.py |  |  | 1.000 |
| walker |  | 581 | 16 | python decl names surface in src/pluggy/__init__.py |  |  | 1.000 |
| walker |  | 581 | 0 | python decl at src/pluggy/__init__.py:32 |  |  | 1.000 |
| walker |  | 600 | 19 | listing of 'changelog' |  |  | 1.000 |
| walker |  | 632 | 32 | listing of 'docs' |  |  | 0.912 |
| ns | 632 |  | 117 | pyproject.toml build-system + license + authors | 2.2 |  | 0.912 |
| walker |  | 675 | 43 | listing of 'downstream' |  |  | 0.913 |
| walker |  | 699 | 24 | python decl names surface in src/pluggy/_warnings.py |  |  | 0.913 |
| walker |  | 699 | 0 | python decl at src/pluggy/_warnings.py:4 |  |  | 0.913 |
| walker |  | 715 | 16 | python decl at src/pluggy/_warnings.py:10 |  |  | 0.914 |
| walker |  | 728 | 13 | python class body at src/pluggy/_warnings.py:4 |  |  | 0.914 |
| walker |  | 741 | 13 | python class body at src/pluggy/_warnings.py:10 |  |  | 0.914 |
| walker |  | 755 | 14 | python decl doc at src/pluggy/_warnings.py:4 |  |  | 0.914 |
| walker |  | 762 | 7 | python imports in src/pluggy/_warnings.py |  |  | 0.914 |
| ns | 799 |  | 167 | pyproject.toml description + requires-python + dep groups + package layout | 2.3 | 2.2 | 0.827 |
| walker |  | 815 | 53 | [package] in pyproject.toml |  |  | 0.832 |
| ns | 1120 |  | 321 | README example: spec + impls | 2.4 |  | 0.695 |
| walker |  | 1271 | 456 | README.rst section #0 |  |  | 0.820 |
| walker |  | 1274 | 3 | listing of 'docs/_static' |  |  | 0.820 |
| walker |  | 1330 | 56 | python decl body at src/pluggy/__init__.py:32 body 33 |  |  | 0.820 |
| ns | 1333 |  | 213 | README example: register + call + output | 2.5 | 2.4 | 0.804 |
| walker |  | 1401 | 71 | python decl names surface in src/pluggy/_result.py |  |  | 0.804 |
| walker |  | 1401 | 0 | python decl at src/pluggy/_result.py:20 |  |  | 0.804 |
| walker |  | 1412 | 11 | python decl at src/pluggy/_result.py:24 |  |  | 0.804 |
| walker |  | 1421 | 9 | python decl doc at src/pluggy/_result.py:20 |  |  | 0.804 |
| walker |  | 1443 | 22 | python class body at src/pluggy/_result.py:24 |  |  | 0.804 |
| walker |  | 1475 | 32 | python decl doc at src/pluggy/_result.py:24 |  |  | 0.804 |
| ns | 1541 |  | 208 | _hooks.py: marker + caller class/fn name surface | 3.1 |  | 0.750 |
| walker |  | 1570 | 95 | python method sigs in src/pluggy/_result.py |  |  | 0.751 |
| walker |  | 1570 | 0 | python method at src/pluggy/_result.py:67 |  |  | 0.751 |
| walker |  | 1570 | 0 | python method at src/pluggy/_result.py:80 |  |  | 0.751 |
| walker |  | 1570 | 0 | python method at src/pluggy/_result.py:91 |  |  | 0.751 |
| walker |  | 1585 | 15 | python method at src/pluggy/_result.py:51 |  |  | 0.751 |
| walker |  | 1602 | 17 | python method at src/pluggy/_result.py:42 |  |  | 0.752 |
| walker |  | 1626 | 24 | python method at src/pluggy/_result.py:56 |  |  | 0.752 |
| walker |  | 1638 | 12 | python method doc at src/pluggy/_result.py:42 |  |  | 0.752 |
| walker |  | 1650 | 12 | python method doc at src/pluggy/_result.py:51 |  |  | 0.752 |
| walker |  | 1658 | 8 | python method body at src/pluggy/_result.py:51 body 54 |  |  | 0.752 |
| walker |  | 1670 | 12 | python method doc at src/pluggy/_result.py:56 |  |  | 0.752 |
| walker |  | 1710 | 40 | python method at src/pluggy/_result.py:31 |  |  | 0.752 |
| walker |  | 1722 | 12 | python method doc at src/pluggy/_result.py:31 |  |  | 0.752 |
| walker |  | 1726 | 4 | listing of 'docs/_static/img' |  |  | 0.752 |
| walker |  | 1846 | 120 | python decl names surface in src/pluggy/_manager.py |  |  | 0.753 |
| walker |  | 1846 | 0 | python decl at src/pluggy/_manager.py:42 |  |  | 0.753 |
| walker |  | 1846 | 0 | python decl at src/pluggy/_manager.py:52 |  |  | 0.753 |
| walker |  | 1846 | 0 | python decl at src/pluggy/_manager.py:65 |  |  | 0.753 |
| walker |  | 1846 | 0 | python decl at src/pluggy/_manager.py:83 |  |  | 0.753 |
| walker |  | 1846 | 0 | python decl at src/pluggy/_manager.py:525 |  |  | 0.753 |
| ns | 1850 |  | 309 | _manager.py: PluginManager + helpers method-name surface | 3.2 |  | 0.691 |
| walker |  | 1859 | 13 | python decl doc at src/pluggy/_manager.py:65 |  |  | 0.691 |
| walker |  | 1902 | 43 | python decl doc at src/pluggy/_manager.py:52 |  |  | 0.691 |
| walker |  | 1928 | 26 | python decl at src/pluggy/_manager.py:37 |  |  | 0.691 |
| walker |  | 1944 | 16 | python decl body at src/pluggy/_manager.py:525 body 526 |  |  | 0.691 |
| walker |  | 2104 | 160 | python decl names surface in src/pluggy/_hooks.py |  |  | 0.706 |
| walker |  | 2104 | 0 | python decl at src/pluggy/_hooks.py:40 |  |  | 0.706 |
| walker |  | 2104 | 0 | python decl at src/pluggy/_hooks.py:56 |  |  | 0.706 |
| walker |  | 2104 | 0 | python decl at src/pluggy/_hooks.py:281 |  |  | 0.706 |
| walker |  | 2104 | 0 | python decl at src/pluggy/_hooks.py:293 |  |  | 0.706 |
| walker |  | 2104 | 0 | python decl at src/pluggy/_hooks.py:382 |  |  | 0.706 |
| walker |  | 2104 | 0 | python decl at src/pluggy/_hooks.py:593 |  |  | 0.706 |
| walker |  | 2112 | 8 | python decl at src/pluggy/_hooks.py:358 |  |  | 0.709 |
| walker |  | 2120 | 8 | python decl at src/pluggy/_hooks.py:638 |  |  | 0.713 |
| walker |  | 2128 | 8 | python decl at src/pluggy/_hooks.py:696 |  |  | 0.713 |
| walker |  | 2137 | 9 | python decl at src/pluggy/_hooks.py:77 |  |  | 0.717 |
| walker |  | 2146 | 9 | python decl at src/pluggy/_hooks.py:164 |  |  | 0.722 |
| walker |  | 2158 | 12 | python decl doc at src/pluggy/_hooks.py:40 |  |  | 0.722 |
| walker |  | 2170 | 12 | python decl doc at src/pluggy/_hooks.py:56 |  |  | 0.722 |
| walker |  | 2186 | 16 | python class body at src/pluggy/_hooks.py:77 |  |  | 0.722 |
| walker |  | 2202 | 16 | python class body at src/pluggy/_hooks.py:164 |  |  | 0.722 |
| walker |  | 2219 | 17 | python decl doc at src/pluggy/_hooks.py:382 |  |  | 0.722 |
| walker |  | 2237 | 18 | python decl doc at src/pluggy/_hooks.py:638 |  |  | 0.722 |
| walker |  | 2268 | 31 | python decl doc at src/pluggy/_hooks.py:358 |  |  | 0.722 |
| ns | 2273 |  | 423 | Tutorial: toy-example.py in full | 3.3 |  | 0.642 |
| walker |  | 2315 | 47 | python class body at src/pluggy/_hooks.py:358 |  |  | 0.642 |
| ns | 2325 |  | 52 | _callers.py top-level function names | 3.4 |  | 0.635 |
| walker |  | 2427 | 112 | python class body at src/pluggy/_hooks.py:638 |  |  | 0.635 |
| ns | 2450 |  | 125 | _result.py class + method surface | 3.5 |  | 0.645 |
| ns | 2483 |  | 33 | _warnings.py classes | 3.6 |  | 0.648 |
| walker |  | 2515 | 88 | python class body at src/pluggy/_hooks.py:696 |  |  | 0.648 |
| walker |  | 2607 | 92 | python class body at src/pluggy/_hooks.py:382 |  |  | 0.648 |
| ns | 2667 |  | 184 | HookspecOpts TypedDict fields | 4.1 | 3.1 | 0.626 |
| walker |  | 2831 | 224 | python class body at src/pluggy/_hooks.py:56 |  |  | 0.630 |
| ns | 2915 |  | 248 | HookimplOpts TypedDict fields | 4.2 | 3.1 | 0.647 |
| walker |  | 2993 | 162 | python class body at src/pluggy/_hooks.py:40 |  |  | 0.679 |
| walker |  | 3068 | 75 | python decl doc at src/pluggy/_hooks.py:164 |  |  | 0.679 |
| walker |  | 3144 | 76 | python decl doc at src/pluggy/_hooks.py:77 |  |  | 0.679 |
| ns | 3256 |  | 341 | docs/index.rst section heading map | 4.3 |  | 0.631 |
| walker |  | 3320 | 176 | python method sigs in src/pluggy/_hooks.py |  |  | 0.634 |
| walker |  | 3320 | 0 | python method at src/pluggy/_hooks.py:88 |  |  | 0.634 |
| walker |  | 3320 | 0 | python method at src/pluggy/_hooks.py:175 |  |  | 0.634 |
| walker |  | 3320 | 0 | python method at src/pluggy/_hooks.py:365 |  |  | 0.634 |
| walker |  | 3320 | 0 | python method at src/pluggy/_hooks.py:420 |  |  | 0.634 |
| walker |  | 3330 | 10 | python method doc at src/pluggy/_hooks.py:365 |  |  | 0.634 |
| walker |  | 3340 | 10 | python method body at src/pluggy/_hooks.py:420 body 421 |  |  | 0.634 |
| walker |  | 3352 | 12 | python method body at src/pluggy/_hooks.py:88 body 89 |  |  | 0.634 |
| walker |  | 3364 | 12 | python method body at src/pluggy/_hooks.py:175 body 176 |  |  | 0.634 |
| walker |  | 3405 | 41 | python method at src/pluggy/_hooks.py:424 |  |  | 0.634 |
| walker |  | 3437 | 32 | python decl doc at src/pluggy/_hooks.py:593 |  |  | 0.634 |
| walker |  | 3511 | 74 | python decl doc at src/pluggy/_hooks.py:293 |  |  | 0.634 |
| ns | 3566 |  | 310 | HookCaller._add_hookimpl - the ordering algorithm | 4.4 |  | 0.608 |
| walker |  | 3581 | 70 | python method at src/pluggy/_hooks.py:393 |  |  | 0.608 |
| walker |  | 3593 | 12 | python method doc at src/pluggy/_hooks.py:393 |  |  | 0.608 |
| walker |  | 3644 | 51 | python method doc at src/pluggy/_result.py:80 |  |  | 0.608 |
| walker |  | 3887 | 243 | python method sigs in src/pluggy/_manager.py |  |  | 0.614 |
| walker |  | 3887 | 0 | python method at src/pluggy/_manager.py:59 |  |  | 0.614 |
| walker |  | 3887 | 0 | python method at src/pluggy/_manager.py:68 |  |  | 0.614 |
| walker |  | 3887 | 0 | python method at src/pluggy/_manager.py:76 |  |  | 0.614 |
| walker |  | 3887 | 0 | python method at src/pluggy/_manager.py:79 |  |  | 0.614 |
| walker |  | 3887 | 0 | python method at src/pluggy/_manager.py:125 |  |  | 0.614 |
| walker |  | 3887 | 0 | python method at src/pluggy/_manager.py:176 |  |  | 0.614 |
| walker |  | 3887 | 0 | python method at src/pluggy/_manager.py:233 |  |  | 0.614 |
| walker |  | 3887 | 0 | python method at src/pluggy/_manager.py:238 |  |  | 0.614 |
| walker |  | 3897 | 10 | python method at src/pluggy/_manager.py:100 |  |  | 0.614 |
| walker |  | 3910 | 13 | python method at src/pluggy/_manager.py:71 |  |  | 0.614 |
| walker |  | 3919 | 9 | python method body at src/pluggy/_manager.py:68 body 69 |  |  | 0.614 |
| walker |  | 3934 | 15 | python method doc at src/pluggy/_manager.py:238 |  |  | 0.614 |
| walker |  | 3969 | 35 | python method at src/pluggy/_manager.py:201 |  |  | 0.614 |
| walker |  | 3987 | 18 | python method doc at src/pluggy/_manager.py:233 |  |  | 0.614 |
| walker |  | 4000 | 13 | python method body at src/pluggy/_manager.py:76 body 77 |  |  | 0.614 |
| walker |  | 4019 | 19 | python method body at src/pluggy/_manager.py:79 body 80 |  |  | 0.614 |
| walker |  | 4039 | 20 | python method body at src/pluggy/_manager.py:71 body 73 |  |  | 0.614 |
| walker |  | 4063 | 24 | python method body at src/pluggy/_manager.py:238 body 240 |  |  | 0.614 |
| walker |  | 4147 | 84 | python method at src/pluggy/_hooks.py:91 |  |  | 0.614 |
| walker |  | 4156 | 9 | python method body at src/pluggy/_hooks.py:91 body 99 |  |  | 0.614 |
| walker |  | 4218 | 62 | listing of 'testing' |  |  | 0.615 |
| walker |  | 4276 | 58 | python method doc at src/pluggy/_result.py:91 |  |  | 0.615 |
| walker |  | 4307 | 31 | python method body at src/pluggy/_manager.py:59 body 60 |  |  | 0.615 |
| walker |  | 4399 | 92 | python method at src/pluggy/_hooks.py:101 |  |  | 0.615 |
| walker |  | 4413 | 14 | python method body at src/pluggy/_hooks.py:101 body 109 |  |  | 0.615 |
| walker |  | 4470 | 57 | python decl names surface in src/pluggy/_tracing.py |  |  | 0.615 |
| walker |  | 4470 | 0 | python decl at src/pluggy/_tracing.py:16 |  |  | 0.615 |
| walker |  | 4470 | 0 | python decl at src/pluggy/_tracing.py:59 |  |  | 0.615 |
| walker |  | 4567 | 97 | python method at src/pluggy/_hooks.py:111 |  |  | 0.615 |
| ns | 4587 |  | 1021 | _multicall body - the call loop | 4.5 | 3.4 | 0.535 |
| walker |  | 4666 | 99 | python method at src/pluggy/_hooks.py:178 |  |  | 0.535 |
| walker |  | 4675 | 9 | python method body at src/pluggy/_hooks.py:178 body 188 |  |  | 0.535 |
| walker |  | 4782 | 107 | python method at src/pluggy/_hooks.py:190 |  |  | 0.535 |
| walker |  | 4796 | 14 | python method body at src/pluggy/_hooks.py:190 body 200 |  |  | 0.535 |
| walker |  | 4873 | 77 | python method doc at src/pluggy/_manager.py:201 |  |  | 0.535 |
| walker |  | 4943 | 70 | python decl names surface in src/pluggy/_callers.py |  |  | 0.543 |
| walker |  | 4974 | 31 | python decl at src/pluggy/_callers.py:27 |  |  | 0.543 |
| walker |  | 5005 | 31 | python decl doc at src/pluggy/_callers.py:27 |  |  | 0.543 |
| walker |  | 5034 | 29 | python decl at src/pluggy/_callers.py:70 |  |  | 0.543 |
| walker |  | 5068 | 34 | python decl at src/pluggy/_callers.py:60 |  |  | 0.543 |
| walker |  | 5180 | 112 | python method at src/pluggy/_hooks.py:202 |  |  | 0.543 |
| ns | 5248 |  | 661 | PluginManager.register body | 4.6 | 3.2 | 0.510 |
| walker |  | 5390 | 210 | python method sigs in src/pluggy/_tracing.py |  |  | 0.510 |
| walker |  | 5390 | 0 | python method at src/pluggy/_tracing.py:17 |  |  | 0.510 |
| walker |  | 5390 | 0 | python method at src/pluggy/_tracing.py:22 |  |  | 0.510 |
| walker |  | 5390 | 0 | python method at src/pluggy/_tracing.py:25 |  |  | 0.510 |
| walker |  | 5390 | 0 | python method at src/pluggy/_tracing.py:42 |  |  | 0.510 |
| walker |  | 5390 | 0 | python method at src/pluggy/_tracing.py:48 |  |  | 0.510 |
| walker |  | 5390 | 0 | python method at src/pluggy/_tracing.py:51 |  |  | 0.510 |
| walker |  | 5390 | 0 | python method at src/pluggy/_tracing.py:60 |  |  | 0.510 |
| walker |  | 5390 | 0 | python method at src/pluggy/_tracing.py:64 |  |  | 0.510 |
| walker |  | 5390 | 0 | python method at src/pluggy/_tracing.py:67 |  |  | 0.510 |
| walker |  | 5399 | 9 | python method body at src/pluggy/_tracing.py:48 body 49 |  |  | 0.510 |
| walker |  | 5411 | 12 | python method body at src/pluggy/_tracing.py:22 body 23 |  |  | 0.510 |
| walker |  | 5424 | 13 | python method body at src/pluggy/_tracing.py:64 body 65 |  |  | 0.510 |
| walker |  | 5441 | 17 | python method body at src/pluggy/_tracing.py:67 body 68 |  |  | 0.510 |
| ns | 5454 |  | 206 | HookCaller.__call__ body | 4.7 | 3.1 | 0.501 |
| walker |  | 5459 | 18 | python method body at src/pluggy/_tracing.py:60 body 61 |  |  | 0.501 |
| walker |  | 5463 | 4 | listing of '.claude' |  |  | 0.501 |
| walker |  | 5478 | 15 | listing of 'docs/examples' |  |  | 0.501 |
| walker |  | 5647 | 169 | python decl doc at src/pluggy/_manager.py:83 |  |  | 0.501 |
| ns | 5775 |  | 321 | HookCaller.call_historic body | 4.8 | 3.1 | 0.486 |
| walker |  | 5816 | 169 | python decl doc at src/pluggy/_warnings.py:10 |  |  | 0.486 |
| walker |  | 5893 | 77 | python decl body at src/pluggy/_hooks.py:281 body 282 |  |  | 0.486 |
| walker |  | 5982 | 89 | python method doc at src/pluggy/_result.py:67 |  |  | 0.486 |
| ns | 6215 |  | 440 | HookCaller.call_extra body | 4.9 | 3.1 | 0.470 |
| walker |  | 6218 | 236 | python method sigs #1 in src/pluggy/_hooks.py |  |  | 0.485 |
| walker |  | 6218 | 0 | python method at src/pluggy/_hooks.py:438 |  |  | 0.485 |
| walker |  | 6218 | 0 | python method at src/pluggy/_hooks.py:442 |  |  | 0.485 |
| walker |  | 6218 | 0 | python method at src/pluggy/_hooks.py:449 |  |  | 0.485 |
| walker |  | 6218 | 0 | python method at src/pluggy/_hooks.py:453 |  |  | 0.485 |
| walker |  | 6218 | 0 | python method at src/pluggy/_hooks.py:477 |  |  | 0.485 |
| walker |  | 6218 | 0 | python method at src/pluggy/_hooks.py:499 |  |  | 0.485 |
| walker |  | 6218 | 0 | python method at src/pluggy/_hooks.py:577 |  |  | 0.485 |
| walker |  | 6218 | 0 | python method at src/pluggy/_hooks.py:612 |  |  | 0.485 |
| walker |  | 6233 | 15 | python method doc at src/pluggy/_hooks.py:449 |  |  | 0.485 |
| walker |  | 6264 | 31 | python method at src/pluggy/_hooks.py:543 |  |  | 0.485 |
| walker |  | 6275 | 11 | python method body at src/pluggy/_hooks.py:449 body 451 |  |  | 0.485 |
| walker |  | 6293 | 18 | python method doc at src/pluggy/_hooks.py:438 |  |  | 0.485 |
| walker |  | 6305 | 12 | python method body at src/pluggy/_hooks.py:438 body 440 |  |  | 0.485 |
| walker |  | 6322 | 17 | python method at src/pluggy/_hooks.py:480 |  |  | 0.485 |
| walker |  | 6337 | 15 | python method body at src/pluggy/_hooks.py:477 body 478 |  |  | 0.485 |
| walker |  | 6355 | 18 | python method at src/pluggy/_hooks.py:618 |  |  | 0.485 |
| walker |  | 6407 | 52 | python method at src/pluggy/_hooks.py:516 |  |  | 0.486 |
| walker |  | 6421 | 14 | python method doc at src/pluggy/_hooks.py:453 |  |  | 0.487 |
| walker |  | 6466 | 45 | python method doc at src/pluggy/_hooks.py:543 |  |  | 0.488 |
| walker |  | 6487 | 21 | python method doc at src/pluggy/_hooks.py:577 |  |  | 0.488 |
| walker |  | 6554 | 67 | python method doc at src/pluggy/_hooks.py:499 |  |  | 0.492 |
| walker |  | 6647 | 93 | python method doc at src/pluggy/_hooks.py:516 |  |  | 0.499 |
| ns | 6884 |  | 669 | PluginManager._verify_hook body - validation rules | 4.10 |  | 0.477 |
| walker |  | 6892 | 245 | python method sigs #1 in src/pluggy/_manager.py |  |  | 0.495 |
| walker |  | 6892 | 0 | python method at src/pluggy/_manager.py:242 |  |  | 0.495 |
| walker |  | 6892 | 0 | python method at src/pluggy/_manager.py:252 |  |  | 0.495 |
| walker |  | 6892 | 0 | python method at src/pluggy/_manager.py:296 |  |  | 0.495 |
| walker |  | 6892 | 0 | python method at src/pluggy/_manager.py:300 |  |  | 0.495 |
| walker |  | 6892 | 0 | python method at src/pluggy/_manager.py:304 |  |  | 0.495 |
| walker |  | 6892 | 0 | python method at src/pluggy/_manager.py:315 |  |  | 0.495 |
| walker |  | 6892 | 0 | python method at src/pluggy/_manager.py:319 |  |  | 0.495 |
| walker |  | 6892 | 0 | python method at src/pluggy/_manager.py:323 |  |  | 0.495 |
| walker |  | 6892 | 0 | python method at src/pluggy/_manager.py:331 |  |  | 0.495 |
| walker |  | 6892 | 0 | python method at src/pluggy/_manager.py:379 |  |  | 0.495 |
| walker |  | 6892 | 0 | python method at src/pluggy/_manager.py:395 |  |  | 0.495 |
| walker |  | 6906 | 14 | python method doc at src/pluggy/_manager.py:300 |  |  | 0.495 |
| walker |  | 6921 | 15 | python method doc at src/pluggy/_manager.py:296 |  |  | 0.495 |
| ns | 6938 |  | 54 | docs/examples listing (eggsample + eggsample-spam) | 5.1 |  | 0.491 |
| walker |  | 6952 | 31 | python method at src/pluggy/_manager.py:278 |  |  | 0.491 |
| walker |  | 6969 | 17 | python method doc at src/pluggy/_manager.py:319 |  |  | 0.491 |
| walker |  | 6987 | 18 | python method doc at src/pluggy/_manager.py:315 |  |  | 0.491 |
| walker |  | 6999 | 12 | python method body at src/pluggy/_manager.py:315 body 317 |  |  | 0.491 |
| walker |  | 7012 | 13 | python method body at src/pluggy/_manager.py:319 body 321 |  |  | 0.491 |
| walker |  | 7041 | 29 | python method doc at src/pluggy/_manager.py:323 |  |  | 0.491 |
| walker |  | 7071 | 30 | python method doc at src/pluggy/_manager.py:242 |  |  | 0.491 |
| walker |  | 7089 | 18 | python method body at src/pluggy/_manager.py:300 body 302 |  |  | 0.491 |
| walker |  | 7111 | 22 | python method body at src/pluggy/_manager.py:296 body 298 |  |  | 0.491 |
| walker |  | 7155 | 44 | python method doc at src/pluggy/_manager.py:379 |  |  | 0.491 |
| walker |  | 7213 | 58 | python method doc at src/pluggy/_manager.py:252 |  |  | 0.491 |
| ns | 7214 |  | 276 | eggsample hookspecs | 5.2 |  | 0.479 |
| walker |  | 7221 | 8 | python method body at src/pluggy/_manager.py:242 body 250 |  |  | 0.479 |
| walker |  | 7229 | 8 | python method body at src/pluggy/_manager.py:323 body 329 |  |  | 0.479 |
| walker |  | 7321 | 92 | python method doc at src/pluggy/_manager.py:304 |  |  | 0.479 |
| walker |  | 7414 | 93 | python method doc at src/pluggy/_manager.py:395 |  |  | 0.480 |
| walker |  | 7424 | 10 | python method body at src/pluggy/_manager.py:233 body 235 |  |  | 0.480 |
| ns | 7474 |  | 260 | eggsample host wiring + main() | 5.3 | 5.2 | 0.469 |
| walker |  | 7562 | 138 | python decl names surface #1 in src/pluggy/_hooks.py |  |  | 0.469 |
| walker |  | 7586 | 24 | python decl at src/pluggy/_hooks.py:377 |  |  | 0.469 |
| walker |  | 7620 | 34 | python decl at src/pluggy/_hooks.py:33 |  |  | 0.469 |
| ns | 7722 |  | 248 | eggsample-spam plugin code | 5.4 |  | 0.460 |
| walker |  | 7831 | 211 | python class body at src/pluggy/_hooks.py:593 |  |  | 0.460 |
| walker |  | 7890 | 59 | python decl at src/pluggy/_callers.py:82 |  |  | 0.460 |
| walker |  | 7901 | 11 | python method body at src/pluggy/_result.py:42 body 45 |  |  | 0.460 |
| ns | 7915 |  | 193 | eggsample + eggsample-spam setup.py: entry-point wiring | 5.5 |  | 0.453 |
| ns | 7977 |  | 62 | testing/ directory listing | 5.6 |  | 0.462 |
| walker |  | 7991 | 90 | python method sigs #2 in src/pluggy/_hooks.py |  |  | 0.462 |
| walker |  | 7991 | 0 | python method at src/pluggy/_hooks.py:634 |  |  | 0.462 |
| walker |  | 7991 | 0 | python method at src/pluggy/_hooks.py:692 |  |  | 0.462 |
| walker |  | 7991 | 0 | python method at src/pluggy/_hooks.py:709 |  |  | 0.462 |
| walker |  | 8014 | 23 | python method at src/pluggy/_hooks.py:626 |  |  | 0.462 |
| walker |  | 8023 | 9 | python method body at src/pluggy/_hooks.py:626 body 628 |  |  | 0.462 |
| walker |  | 8040 | 17 | python method body at src/pluggy/_hooks.py:634 body 635 |  |  | 0.462 |
| walker |  | 8103 | 63 | python method at src/pluggy/_hooks.py:656 |  |  | 0.462 |
| walker |  | 8115 | 12 | python method doc at src/pluggy/_hooks.py:656 |  |  | 0.462 |
| walker |  | 8141 | 26 | python method at src/pluggy/_hooks.py:630 |  |  | 0.462 |
| walker |  | 8152 | 11 | python method body at src/pluggy/_hooks.py:630 body 632 |  |  | 0.462 |
| walker |  | 8177 | 25 | python method body at src/pluggy/_hooks.py:692 body 693 |  |  | 0.462 |
| ns | 8225 |  | 248 | Shared test fixtures (conftest.py) | 5.7 |  | 0.453 |
| ns | 8257 |  | 32 | docs/ listing | 5.8 |  | 0.459 |
| walker |  | 8299 | 122 | python method doc at src/pluggy/_manager.py:125 |  |  | 0.461 |
| walker |  | 8398 | 99 | python imports in src/pluggy/_result.py |  |  | 0.461 |
| walker |  | 8449 | 51 | python decl doc at src/pluggy/_callers.py:82 |  |  | 0.462 |
| walker |  | 8574 | 125 | python method doc at src/pluggy/_manager.py:176 |  |  | 0.462 |
| ns | 8607 |  | 350 | HookCaller.__init__ + the hookimpls-list layout comment | 5.9 | 3.1 | 0.455 |
| walker |  | 8631 | 57 | python imports in src/pluggy/_tracing.py |  |  | 0.455 |
| walker |  | 8643 | 12 | python method body at src/pluggy/_manager.py:304 body 313 |  |  | 0.455 |
| walker |  | 8779 | 136 | python method doc at src/pluggy/_manager.py:278 |  |  | 0.455 |
| walker |  | 8792 | 13 | python method body at src/pluggy/_manager.py:233 body 236 |  |  | 0.455 |
| walker |  | 8854 | 62 | python method body at src/pluggy/_hooks.py:612 body 613 |  |  | 0.455 |
| walker |  | 8862 | 8 | listing of 'docs/examples/eggsample' |  |  | 0.457 |
| walker |  | 8882 | 20 | listing of 'docs/examples/eggsample/eggsample' |  |  | 0.463 |
| walker |  | 8892 | 10 | python imports in docs/examples/eggsample/eggsample/__init__.py |  |  | 0.463 |
| walker |  | 9012 | 120 | python method sigs #2 in src/pluggy/_manager.py |  |  | 0.477 |
| walker |  | 9012 | 0 | python method at src/pluggy/_manager.py:425 |  |  | 0.477 |
| walker |  | 9012 | 0 | python method at src/pluggy/_manager.py:430 |  |  | 0.477 |
| walker |  | 9012 | 0 | python method at src/pluggy/_manager.py:434 |  |  | 0.477 |
| walker |  | 9012 | 0 | python method at src/pluggy/_manager.py:487 |  |  | 0.477 |
| ns | 9039 |  | 432 | test_pluginmanager.py: test function name list | 5.10 |  | 0.467 |
| walker |  | 9040 | 28 | python method at src/pluggy/_manager.py:512 |  |  | 0.467 |
| walker |  | 9071 | 31 | python method at src/pluggy/_manager.py:450 |  |  | 0.467 |
| walker |  | 9084 | 13 | python method body at src/pluggy/_manager.py:425 body 428 |  |  | 0.467 |
| walker |  | 9105 | 21 | python method doc at src/pluggy/_manager.py:430 |  |  | 0.467 |
| walker |  | 9117 | 12 | python method body at src/pluggy/_manager.py:430 body 432 |  |  | 0.467 |
| walker |  | 9145 | 28 | python method doc at src/pluggy/_manager.py:425 |  |  | 0.467 |
| walker |  | 9182 | 37 | python method doc at src/pluggy/_manager.py:487 |  |  | 0.467 |
| ns | 9192 |  | 153 | CHANGELOG.rst recent-release headers | 6.1 |  | 0.464 |
| walker |  | 9232 | 50 | python method doc at src/pluggy/_manager.py:512 |  |  | 0.464 |
| walker |  | 9292 | 60 | python method doc at src/pluggy/_manager.py:434 |  |  | 0.464 |
| ns | 9403 |  | 211 | Test files: per-file test-fn name lists (smaller files) | 6.2 |  | 0.460 |
| walker |  | 9456 | 164 | python method doc at src/pluggy/_manager.py:450 |  |  | 0.460 |
| walker |  | 9505 | 49 | python method body at src/pluggy/_tracing.py:17 body 18 |  |  | 0.460 |
| walker |  | 9522 | 17 | python method body at src/pluggy/_manager.py:304 body 312 |  |  | 0.460 |
| walker |  | 9540 | 18 | python decl names surface in docs/examples/eggsample/eggsample/__init__.py |  |  | 0.460 |
| walker |  | 9639 | 99 | python method at src/pluggy/_manager.py:114 |  |  | 0.460 |
| walker |  | 9659 | 20 | python method body at src/pluggy/_manager.py:114 body 123 |  |  | 0.460 |
| walker |  | 9714 | 55 | python method body at src/pluggy/_tracing.py:51 body 52 |  |  | 0.460 |
| walker |  | 9789 | 75 | python decl body at src/pluggy/_manager.py:42 body 43 |  |  | 0.460 |
| walker |  | 9797 | 8 | python method body at src/pluggy/_manager.py:252 body 258 |  |  | 0.460 |
| walker |  | 9805 | 8 | python method body at src/pluggy/_manager.py:278 body 294 |  |  | 0.460 |
| ns | 9847 |  | 444 | api_reference.rst - autoclass directives | 6.3 |  | 0.445 |
| ns | 9940 |  | 93 | downstream/ + scripts/ + .github/ + changelog/ listings | 6.4 |  | 0.448 |
| ns | 10250 |  | 310 | load_setuptools_entrypoints body | 6.5 | 3.2 | 0.443 |
