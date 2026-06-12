Score(3000)=0.679 I=0.855 C=0.540 ns_rows≤3K=17/40 (reached=9 partial=2 missing=6)

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
| walker |  | 740 | 22 | python decl names surface in src/pluggy/_warnings.py |  |  | 0.916 |
| walker |  | 740 | 0 | python decl at src/pluggy/_warnings.py:4 |  |  | 0.916 |
| walker |  | 754 | 14 | python decl at src/pluggy/_warnings.py:10 |  |  | 0.916 |
| walker |  | 767 | 13 | python class body at src/pluggy/_warnings.py:4 |  |  | 0.916 |
| walker |  | 780 | 13 | python class body at src/pluggy/_warnings.py:10 |  |  | 0.916 |
| ns | 789 |  | 165 | pyproject.toml description + requires-python + dep groups + package layout | 2.3 | 2.2 | 0.832 |
| walker |  | 794 | 14 | python decl doc at src/pluggy/_warnings.py:4 |  |  | 0.832 |
| ns | 1108 |  | 319 | README example: spec + impls | 2.4 |  | 0.695 |
| walker |  | 1250 | 456 | README.rst section #0 |  |  | 0.820 |
| walker |  | 1259 | 9 | python imports in src/pluggy/_warnings.py |  |  | 0.820 |
| walker |  | 1262 | 3 | listing of 'docs/_static' |  |  | 0.820 |
| walker |  | 1318 | 56 | python decl body at src/pluggy/__init__.py:32 body 33 |  |  | 0.820 |
| ns | 1321 |  | 213 | README example: register + call + output | 2.5 | 2.4 | 0.804 |
| walker |  | 1387 | 69 | python decl names surface in src/pluggy/_result.py |  |  | 0.804 |
| walker |  | 1387 | 0 | python decl at src/pluggy/_result.py:20 |  |  | 0.804 |
| walker |  | 1396 | 9 | python decl at src/pluggy/_result.py:24 |  |  | 0.804 |
| walker |  | 1405 | 9 | python decl doc at src/pluggy/_result.py:20 |  |  | 0.804 |
| walker |  | 1425 | 20 | python class body at src/pluggy/_result.py:24 |  |  | 0.804 |
| walker |  | 1459 | 34 | python decl doc at src/pluggy/_result.py:24 |  |  | 0.804 |
| ns | 1493 |  | 172 | _hooks.py: marker + caller class/fn name surface | 3.1 |  | 0.750 |
| walker |  | 1556 | 97 | python method sigs in src/pluggy/_result.py |  |  | 0.751 |
| walker |  | 1556 | 0 | python method at src/pluggy/_result.py:67 |  |  | 0.751 |
| walker |  | 1556 | 0 | python method at src/pluggy/_result.py:80 |  |  | 0.751 |
| walker |  | 1556 | 0 | python method at src/pluggy/_result.py:91 |  |  | 0.751 |
| walker |  | 1569 | 13 | python method at src/pluggy/_result.py:51 |  |  | 0.751 |
| walker |  | 1584 | 15 | python method at src/pluggy/_result.py:42 |  |  | 0.752 |
| walker |  | 1606 | 22 | python method at src/pluggy/_result.py:56 |  |  | 0.752 |
| walker |  | 1618 | 12 | python method doc at src/pluggy/_result.py:42 |  |  | 0.752 |
| walker |  | 1630 | 12 | python method doc at src/pluggy/_result.py:51 |  |  | 0.752 |
| walker |  | 1642 | 12 | python method doc at src/pluggy/_result.py:56 |  |  | 0.752 |
| walker |  | 1652 | 10 | python method body at src/pluggy/_result.py:51 body 54 |  |  | 0.752 |
| walker |  | 1690 | 38 | python method at src/pluggy/_result.py:31 |  |  | 0.752 |
| walker |  | 1702 | 12 | python method doc at src/pluggy/_result.py:31 |  |  | 0.752 |
| walker |  | 1706 | 4 | listing of 'docs/_static/img' |  |  | 0.752 |
| ns | 1748 |  | 255 | _manager.py: PluginManager + helpers method-name surface | 3.2 |  | 0.689 |
| walker |  | 1822 | 116 | python decl names surface in src/pluggy/_manager.py |  |  | 0.691 |
| walker |  | 1822 | 0 | python decl at src/pluggy/_manager.py:42 |  |  | 0.691 |
| walker |  | 1822 | 0 | python decl at src/pluggy/_manager.py:52 |  |  | 0.691 |
| walker |  | 1822 | 0 | python decl at src/pluggy/_manager.py:65 |  |  | 0.691 |
| walker |  | 1822 | 0 | python decl at src/pluggy/_manager.py:83 |  |  | 0.691 |
| walker |  | 1822 | 0 | python decl at src/pluggy/_manager.py:525 |  |  | 0.691 |
| walker |  | 1833 | 11 | python decl doc at src/pluggy/_manager.py:65 |  |  | 0.691 |
| walker |  | 1874 | 41 | python decl doc at src/pluggy/_manager.py:52 |  |  | 0.691 |
| walker |  | 1890 | 16 | python decl body at src/pluggy/_manager.py:525 body 526 |  |  | 0.691 |
| walker |  | 1918 | 28 | python decl at src/pluggy/_manager.py:37 |  |  | 0.691 |
| walker |  | 2074 | 156 | python decl names surface in src/pluggy/_hooks.py |  |  | 0.706 |
| walker |  | 2074 | 0 | python decl at src/pluggy/_hooks.py:40 |  |  | 0.706 |
| walker |  | 2074 | 0 | python decl at src/pluggy/_hooks.py:56 |  |  | 0.706 |
| walker |  | 2074 | 0 | python decl at src/pluggy/_hooks.py:281 |  |  | 0.706 |
| walker |  | 2074 | 0 | python decl at src/pluggy/_hooks.py:293 |  |  | 0.706 |
| walker |  | 2074 | 0 | python decl at src/pluggy/_hooks.py:382 |  |  | 0.706 |
| walker |  | 2074 | 0 | python decl at src/pluggy/_hooks.py:593 |  |  | 0.706 |
| walker |  | 2080 | 6 | python decl at src/pluggy/_hooks.py:358 |  |  | 0.709 |
| walker |  | 2086 | 6 | python decl at src/pluggy/_hooks.py:638 |  |  | 0.713 |
| walker |  | 2092 | 6 | python decl at src/pluggy/_hooks.py:696 |  |  | 0.713 |
| walker |  | 2099 | 7 | python decl at src/pluggy/_hooks.py:77 |  |  | 0.717 |
| walker |  | 2106 | 7 | python decl at src/pluggy/_hooks.py:164 |  |  | 0.722 |
| walker |  | 2116 | 10 | python decl doc at src/pluggy/_hooks.py:40 |  |  | 0.722 |
| walker |  | 2126 | 10 | python decl doc at src/pluggy/_hooks.py:56 |  |  | 0.722 |
| walker |  | 2140 | 14 | python class body at src/pluggy/_hooks.py:77 |  |  | 0.722 |
| walker |  | 2154 | 14 | python class body at src/pluggy/_hooks.py:164 |  |  | 0.722 |
| walker |  | 2169 | 15 | python decl doc at src/pluggy/_hooks.py:382 |  |  | 0.722 |
| ns | 2171 |  | 423 | Tutorial: toy-example.py in full | 3.3 |  | 0.642 |
| walker |  | 2187 | 18 | python decl doc at src/pluggy/_hooks.py:638 |  |  | 0.642 |
| ns | 2213 |  | 42 | _callers.py top-level function names | 3.4 |  | 0.635 |
| walker |  | 2218 | 31 | python decl doc at src/pluggy/_hooks.py:358 |  |  | 0.635 |
| walker |  | 2263 | 45 | python class body at src/pluggy/_hooks.py:358 |  |  | 0.635 |
| ns | 2320 |  | 107 | _result.py class + method surface | 3.5 |  | 0.645 |
| ns | 2347 |  | 27 | _warnings.py classes | 3.6 |  | 0.648 |
| walker |  | 2375 | 112 | python class body at src/pluggy/_hooks.py:638 |  |  | 0.648 |
| walker |  | 2463 | 88 | python class body at src/pluggy/_hooks.py:696 |  |  | 0.648 |
| ns | 2533 |  | 186 | HookspecOpts TypedDict fields | 4.1 | 3.1 | 0.626 |
| walker |  | 2551 | 88 | python class body at src/pluggy/_hooks.py:382 |  |  | 0.626 |
| walker |  | 2777 | 226 | python class body at src/pluggy/_hooks.py:56 |  |  | 0.630 |
| ns | 2781 |  | 248 | HookimplOpts TypedDict fields | 4.2 | 3.1 | 0.647 |
| walker |  | 2941 | 164 | python class body at src/pluggy/_hooks.py:40 |  |  | 0.679 |
| walker |  | 2971 | 30 | python decl doc at src/pluggy/_hooks.py:593 |  |  | 0.679 |
| walker |  | 3048 | 77 | python decl doc at src/pluggy/_hooks.py:164 |  |  | 0.679 |
| ns | 3060 |  | 279 | docs/index.rst section heading map | 4.3 |  | 0.631 |
| walker |  | 3126 | 78 | python decl doc at src/pluggy/_hooks.py:77 |  |  | 0.631 |
| walker |  | 3314 | 188 | python method sigs in src/pluggy/_hooks.py |  |  | 0.634 |
| walker |  | 3314 | 0 | python method at src/pluggy/_hooks.py:88 |  |  | 0.634 |
| walker |  | 3314 | 0 | python method at src/pluggy/_hooks.py:175 |  |  | 0.634 |
| walker |  | 3314 | 0 | python method at src/pluggy/_hooks.py:365 |  |  | 0.634 |
| walker |  | 3314 | 0 | python method at src/pluggy/_hooks.py:420 |  |  | 0.634 |
| walker |  | 3324 | 10 | python method doc at src/pluggy/_hooks.py:365 |  |  | 0.634 |
| walker |  | 3334 | 10 | python method body at src/pluggy/_hooks.py:420 body 421 |  |  | 0.634 |
| walker |  | 3346 | 12 | python method body at src/pluggy/_hooks.py:88 body 89 |  |  | 0.634 |
| walker |  | 3358 | 12 | python method body at src/pluggy/_hooks.py:175 body 176 |  |  | 0.634 |
| ns | 3368 |  | 308 | HookCaller._add_hookimpl - the ordering algorithm | 4.4 |  | 0.608 |
| walker |  | 3397 | 39 | python method at src/pluggy/_hooks.py:424 |  |  | 0.608 |
| walker |  | 3469 | 72 | python decl doc at src/pluggy/_hooks.py:293 |  |  | 0.608 |
| walker |  | 3537 | 68 | python method at src/pluggy/_hooks.py:393 |  |  | 0.608 |
| walker |  | 3549 | 12 | python method doc at src/pluggy/_hooks.py:393 |  |  | 0.608 |
| walker |  | 3598 | 49 | python method doc at src/pluggy/_result.py:80 |  |  | 0.608 |
| walker |  | 3845 | 247 | python method sigs in src/pluggy/_manager.py |  |  | 0.614 |
| walker |  | 3845 | 0 | python method at src/pluggy/_manager.py:59 |  |  | 0.614 |
| walker |  | 3845 | 0 | python method at src/pluggy/_manager.py:68 |  |  | 0.614 |
| walker |  | 3845 | 0 | python method at src/pluggy/_manager.py:76 |  |  | 0.614 |
| walker |  | 3845 | 0 | python method at src/pluggy/_manager.py:79 |  |  | 0.614 |
| walker |  | 3845 | 0 | python method at src/pluggy/_manager.py:125 |  |  | 0.614 |
| walker |  | 3845 | 0 | python method at src/pluggy/_manager.py:176 |  |  | 0.614 |
| walker |  | 3845 | 0 | python method at src/pluggy/_manager.py:233 |  |  | 0.614 |
| walker |  | 3845 | 0 | python method at src/pluggy/_manager.py:238 |  |  | 0.614 |
| walker |  | 3853 | 8 | python method at src/pluggy/_manager.py:100 |  |  | 0.614 |
| walker |  | 3864 | 11 | python method at src/pluggy/_manager.py:71 |  |  | 0.614 |
| walker |  | 3873 | 9 | python method body at src/pluggy/_manager.py:68 body 69 |  |  | 0.614 |
| walker |  | 3886 | 13 | python method doc at src/pluggy/_manager.py:238 |  |  | 0.614 |
| walker |  | 3902 | 16 | python method doc at src/pluggy/_manager.py:233 |  |  | 0.614 |
| walker |  | 3935 | 33 | python method at src/pluggy/_manager.py:201 |  |  | 0.614 |
| walker |  | 3948 | 13 | python method body at src/pluggy/_manager.py:76 body 77 |  |  | 0.614 |
| walker |  | 3967 | 19 | python method body at src/pluggy/_manager.py:79 body 80 |  |  | 0.614 |
| walker |  | 3989 | 22 | python method body at src/pluggy/_manager.py:71 body 73 |  |  | 0.614 |
| walker |  | 4013 | 24 | python method body at src/pluggy/_manager.py:238 body 240 |  |  | 0.614 |
| walker |  | 4095 | 82 | python method at src/pluggy/_hooks.py:91 |  |  | 0.614 |
| walker |  | 4106 | 11 | python method body at src/pluggy/_hooks.py:91 body 99 |  |  | 0.614 |
| walker |  | 4162 | 56 | python method doc at src/pluggy/_result.py:91 |  |  | 0.614 |
| walker |  | 4224 | 62 | listing of 'testing' |  |  | 0.615 |
| walker |  | 4314 | 90 | python method at src/pluggy/_hooks.py:101 |  |  | 0.615 |
| walker |  | 4330 | 16 | python method body at src/pluggy/_hooks.py:101 body 109 |  |  | 0.615 |
| walker |  | 4385 | 55 | python decl names surface in src/pluggy/_tracing.py |  |  | 0.615 |
| walker |  | 4385 | 0 | python decl at src/pluggy/_tracing.py:16 |  |  | 0.615 |
| walker |  | 4385 | 0 | python decl at src/pluggy/_tracing.py:59 |  |  | 0.615 |
| ns | 4391 |  | 1023 | _multicall body - the call loop | 4.5 | 3.4 | 0.535 |
| walker |  | 4416 | 31 | python method body at src/pluggy/_manager.py:59 body 60 |  |  | 0.535 |
| walker |  | 4511 | 95 | python method at src/pluggy/_hooks.py:111 |  |  | 0.535 |
| walker |  | 4608 | 97 | python method at src/pluggy/_hooks.py:178 |  |  | 0.535 |
| walker |  | 4619 | 11 | python method body at src/pluggy/_hooks.py:178 body 188 |  |  | 0.535 |
| walker |  | 4724 | 105 | python method at src/pluggy/_hooks.py:190 |  |  | 0.535 |
| walker |  | 4740 | 16 | python method body at src/pluggy/_hooks.py:190 body 200 |  |  | 0.535 |
| walker |  | 4808 | 68 | python decl names surface in src/pluggy/_callers.py |  |  | 0.543 |
| walker |  | 4837 | 29 | python decl at src/pluggy/_callers.py:27 |  |  | 0.543 |
| walker |  | 4868 | 31 | python decl doc at src/pluggy/_callers.py:27 |  |  | 0.543 |
| walker |  | 4895 | 27 | python decl at src/pluggy/_callers.py:70 |  |  | 0.543 |
| walker |  | 4927 | 32 | python decl at src/pluggy/_callers.py:60 |  |  | 0.543 |
| walker |  | 5004 | 77 | python method doc at src/pluggy/_manager.py:201 |  |  | 0.543 |
| ns | 5054 |  | 663 | PluginManager.register body | 4.6 | 3.2 | 0.510 |
| walker |  | 5114 | 110 | python method at src/pluggy/_hooks.py:202 |  |  | 0.510 |
| ns | 5262 |  | 208 | HookCaller.__call__ body | 4.7 | 3.1 | 0.501 |
| walker |  | 5324 | 210 | python method sigs in src/pluggy/_tracing.py |  |  | 0.501 |
| walker |  | 5324 | 0 | python method at src/pluggy/_tracing.py:17 |  |  | 0.501 |
| walker |  | 5324 | 0 | python method at src/pluggy/_tracing.py:22 |  |  | 0.501 |
| walker |  | 5324 | 0 | python method at src/pluggy/_tracing.py:25 |  |  | 0.501 |
| walker |  | 5324 | 0 | python method at src/pluggy/_tracing.py:42 |  |  | 0.501 |
| walker |  | 5324 | 0 | python method at src/pluggy/_tracing.py:48 |  |  | 0.501 |
| walker |  | 5324 | 0 | python method at src/pluggy/_tracing.py:51 |  |  | 0.501 |
| walker |  | 5324 | 0 | python method at src/pluggy/_tracing.py:60 |  |  | 0.501 |
| walker |  | 5324 | 0 | python method at src/pluggy/_tracing.py:64 |  |  | 0.501 |
| walker |  | 5324 | 0 | python method at src/pluggy/_tracing.py:67 |  |  | 0.501 |
| walker |  | 5333 | 9 | python method body at src/pluggy/_tracing.py:48 body 49 |  |  | 0.501 |
| walker |  | 5345 | 12 | python method body at src/pluggy/_tracing.py:22 body 23 |  |  | 0.501 |
| walker |  | 5358 | 13 | python method body at src/pluggy/_tracing.py:64 body 65 |  |  | 0.501 |
| walker |  | 5375 | 17 | python method body at src/pluggy/_tracing.py:67 body 68 |  |  | 0.501 |
| walker |  | 5393 | 18 | python method body at src/pluggy/_tracing.py:60 body 61 |  |  | 0.501 |
| walker |  | 5397 | 4 | listing of '.claude' |  |  | 0.501 |
| walker |  | 5412 | 15 | listing of 'docs/examples' |  |  | 0.501 |
| walker |  | 5581 | 169 | python decl doc at src/pluggy/_manager.py:83 |  |  | 0.501 |
| ns | 5585 |  | 323 | HookCaller.call_historic body | 4.8 | 3.1 | 0.486 |
| walker |  | 5752 | 171 | python decl doc at src/pluggy/_warnings.py:10 |  |  | 0.486 |
| walker |  | 5827 | 75 | python decl body at src/pluggy/_hooks.py:281 body 282 |  |  | 0.486 |
| walker |  | 5914 | 87 | python method doc at src/pluggy/_result.py:67 |  |  | 0.486 |
| ns | 6025 |  | 440 | HookCaller.call_extra body | 4.9 | 3.1 | 0.470 |
| walker |  | 6150 | 236 | python method sigs #1 in src/pluggy/_hooks.py |  |  | 0.485 |
| walker |  | 6150 | 0 | python method at src/pluggy/_hooks.py:438 |  |  | 0.485 |
| walker |  | 6150 | 0 | python method at src/pluggy/_hooks.py:442 |  |  | 0.485 |
| walker |  | 6150 | 0 | python method at src/pluggy/_hooks.py:449 |  |  | 0.485 |
| walker |  | 6150 | 0 | python method at src/pluggy/_hooks.py:453 |  |  | 0.485 |
| walker |  | 6150 | 0 | python method at src/pluggy/_hooks.py:477 |  |  | 0.485 |
| walker |  | 6150 | 0 | python method at src/pluggy/_hooks.py:499 |  |  | 0.485 |
| walker |  | 6150 | 0 | python method at src/pluggy/_hooks.py:577 |  |  | 0.485 |
| walker |  | 6150 | 0 | python method at src/pluggy/_hooks.py:612 |  |  | 0.485 |
| walker |  | 6163 | 13 | python method doc at src/pluggy/_hooks.py:449 |  |  | 0.485 |
| walker |  | 6192 | 29 | python method at src/pluggy/_hooks.py:543 |  |  | 0.485 |
| walker |  | 6208 | 16 | python method doc at src/pluggy/_hooks.py:438 |  |  | 0.485 |
| walker |  | 6223 | 15 | python method at src/pluggy/_hooks.py:480 |  |  | 0.485 |
| walker |  | 6236 | 13 | python method body at src/pluggy/_hooks.py:449 body 451 |  |  | 0.485 |
| walker |  | 6252 | 16 | python method at src/pluggy/_hooks.py:618 |  |  | 0.485 |
| walker |  | 6266 | 14 | python method body at src/pluggy/_hooks.py:438 body 440 |  |  | 0.485 |
| walker |  | 6281 | 15 | python method body at src/pluggy/_hooks.py:477 body 478 |  |  | 0.485 |
| walker |  | 6293 | 12 | python method doc at src/pluggy/_hooks.py:453 |  |  | 0.486 |
| walker |  | 6343 | 50 | python method at src/pluggy/_hooks.py:516 |  |  | 0.487 |
| walker |  | 6362 | 19 | python method doc at src/pluggy/_hooks.py:577 |  |  | 0.487 |
| walker |  | 6407 | 45 | python method doc at src/pluggy/_hooks.py:543 |  |  | 0.488 |
| walker |  | 6472 | 65 | python method doc at src/pluggy/_hooks.py:499 |  |  | 0.492 |
| walker |  | 6565 | 93 | python method doc at src/pluggy/_hooks.py:516 |  |  | 0.499 |
| ns | 6694 |  | 669 | PluginManager._verify_hook body - validation rules | 4.10 |  | 0.477 |
| ns | 6748 |  | 54 | docs/examples listing (eggsample + eggsample-spam) | 5.1 |  | 0.473 |
| walker |  | 6812 | 247 | python method sigs #1 in src/pluggy/_manager.py |  |  | 0.491 |
| walker |  | 6812 | 0 | python method at src/pluggy/_manager.py:242 |  |  | 0.491 |
| walker |  | 6812 | 0 | python method at src/pluggy/_manager.py:252 |  |  | 0.491 |
| walker |  | 6812 | 0 | python method at src/pluggy/_manager.py:296 |  |  | 0.491 |
| walker |  | 6812 | 0 | python method at src/pluggy/_manager.py:300 |  |  | 0.491 |
| walker |  | 6812 | 0 | python method at src/pluggy/_manager.py:304 |  |  | 0.491 |
| walker |  | 6812 | 0 | python method at src/pluggy/_manager.py:315 |  |  | 0.491 |
| walker |  | 6812 | 0 | python method at src/pluggy/_manager.py:319 |  |  | 0.491 |
| walker |  | 6812 | 0 | python method at src/pluggy/_manager.py:323 |  |  | 0.491 |
| walker |  | 6812 | 0 | python method at src/pluggy/_manager.py:331 |  |  | 0.491 |
| walker |  | 6812 | 0 | python method at src/pluggy/_manager.py:379 |  |  | 0.491 |
| walker |  | 6812 | 0 | python method at src/pluggy/_manager.py:395 |  |  | 0.491 |
| walker |  | 6824 | 12 | python method doc at src/pluggy/_manager.py:300 |  |  | 0.491 |
| walker |  | 6837 | 13 | python method doc at src/pluggy/_manager.py:296 |  |  | 0.491 |
| walker |  | 6866 | 29 | python method at src/pluggy/_manager.py:278 |  |  | 0.491 |
| walker |  | 6881 | 15 | python method doc at src/pluggy/_manager.py:319 |  |  | 0.491 |
| walker |  | 6897 | 16 | python method doc at src/pluggy/_manager.py:315 |  |  | 0.491 |
| walker |  | 6911 | 14 | python method body at src/pluggy/_manager.py:315 body 317 |  |  | 0.491 |
| walker |  | 6926 | 15 | python method body at src/pluggy/_manager.py:319 body 321 |  |  | 0.491 |
| walker |  | 6953 | 27 | python method doc at src/pluggy/_manager.py:323 |  |  | 0.491 |
| walker |  | 6981 | 28 | python method doc at src/pluggy/_manager.py:242 |  |  | 0.491 |
| walker |  | 7001 | 20 | python method body at src/pluggy/_manager.py:300 body 302 |  |  | 0.491 |
| ns | 7024 |  | 276 | eggsample hookspecs | 5.2 |  | 0.479 |
| walker |  | 7043 | 42 | python method doc at src/pluggy/_manager.py:379 |  |  | 0.479 |
| walker |  | 7067 | 24 | python method body at src/pluggy/_manager.py:296 body 298 |  |  | 0.479 |
| walker |  | 7123 | 56 | python method doc at src/pluggy/_manager.py:252 |  |  | 0.479 |
| walker |  | 7131 | 8 | python method body at src/pluggy/_manager.py:242 body 250 |  |  | 0.479 |
| walker |  | 7139 | 8 | python method body at src/pluggy/_manager.py:323 body 329 |  |  | 0.479 |
| walker |  | 7229 | 90 | python method doc at src/pluggy/_manager.py:304 |  |  | 0.479 |
| ns | 7282 |  | 258 | eggsample host wiring + main() | 5.3 | 5.2 | 0.468 |
| walker |  | 7320 | 91 | python method doc at src/pluggy/_manager.py:395 |  |  | 0.469 |
| walker |  | 7330 | 10 | python method body at src/pluggy/_manager.py:233 body 235 |  |  | 0.469 |
| walker |  | 7468 | 138 | python decl names surface #1 in src/pluggy/_hooks.py |  |  | 0.469 |
| walker |  | 7494 | 26 | python decl at src/pluggy/_hooks.py:377 |  |  | 0.469 |
| walker |  | 7530 | 36 | python decl at src/pluggy/_hooks.py:33 |  |  | 0.460 |
| ns | 7530 |  | 248 | eggsample-spam plugin code | 5.4 |  | 0.460 |
| ns | 7723 |  | 193 | eggsample + eggsample-spam setup.py: entry-point wiring | 5.5 |  | 0.453 |
| walker |  | 7743 | 213 | python class body at src/pluggy/_hooks.py:593 |  |  | 0.453 |
| ns | 7785 |  | 62 | testing/ directory listing | 5.6 |  | 0.462 |
| walker |  | 7800 | 57 | python decl at src/pluggy/_callers.py:82 |  |  | 0.462 |
| walker |  | 7811 | 11 | python method body at src/pluggy/_result.py:42 body 45 |  |  | 0.462 |
| walker |  | 7931 | 120 | python method doc at src/pluggy/_manager.py:125 |  |  | 0.465 |
| ns | 8033 |  | 248 | Shared test fixtures (conftest.py) | 5.7 |  | 0.456 |
| walker |  | 8054 | 123 | python method doc at src/pluggy/_manager.py:176 |  |  | 0.456 |
| ns | 8065 |  | 32 | docs/ listing | 5.8 |  | 0.461 |
| walker |  | 8105 | 51 | python decl doc at src/pluggy/_callers.py:82 |  |  | 0.462 |
| walker |  | 8206 | 101 | python imports in src/pluggy/_result.py |  |  | 0.462 |
| walker |  | 8300 | 94 | python method sigs #2 in src/pluggy/_hooks.py |  |  | 0.462 |
| walker |  | 8300 | 0 | python method at src/pluggy/_hooks.py:634 |  |  | 0.462 |
| walker |  | 8300 | 0 | python method at src/pluggy/_hooks.py:692 |  |  | 0.462 |
| walker |  | 8300 | 0 | python method at src/pluggy/_hooks.py:709 |  |  | 0.462 |
| walker |  | 8321 | 21 | python method at src/pluggy/_hooks.py:626 |  |  | 0.462 |
| walker |  | 8332 | 11 | python method body at src/pluggy/_hooks.py:626 body 628 |  |  | 0.462 |
| walker |  | 8349 | 17 | python method body at src/pluggy/_hooks.py:634 body 635 |  |  | 0.462 |
| walker |  | 8373 | 24 | python method at src/pluggy/_hooks.py:630 |  |  | 0.462 |
| ns | 8413 |  | 348 | HookCaller.__init__ + the hookimpls-list layout comment | 5.9 | 3.1 | 0.455 |
| walker |  | 8434 | 61 | python method at src/pluggy/_hooks.py:656 |  |  | 0.455 |
| walker |  | 8446 | 12 | python method doc at src/pluggy/_hooks.py:656 |  |  | 0.455 |
| walker |  | 8471 | 25 | python method body at src/pluggy/_hooks.py:692 body 693 |  |  | 0.455 |
| walker |  | 8484 | 13 | python method body at src/pluggy/_hooks.py:630 body 632 |  |  | 0.455 |
| walker |  | 8496 | 12 | python method body at src/pluggy/_manager.py:304 body 313 |  |  | 0.455 |
| walker |  | 8555 | 59 | python imports in src/pluggy/_tracing.py |  |  | 0.455 |
| walker |  | 8691 | 136 | python method doc at src/pluggy/_manager.py:278 |  |  | 0.455 |
| walker |  | 8753 | 62 | python method body at src/pluggy/_hooks.py:612 body 613 |  |  | 0.455 |
| walker |  | 8761 | 8 | listing of 'docs/examples/eggsample' |  |  | 0.457 |
| ns | 8779 |  | 366 | test_pluginmanager.py: test function name list | 5.10 |  | 0.447 |
| walker |  | 8781 | 20 | listing of 'docs/examples/eggsample/eggsample' |  |  | 0.452 |
| walker |  | 8789 | 8 | python imports in docs/examples/eggsample/eggsample/__init__.py |  |  | 0.452 |
| walker |  | 8909 | 120 | python method sigs #2 in src/pluggy/_manager.py |  |  | 0.467 |
| walker |  | 8909 | 0 | python method at src/pluggy/_manager.py:425 |  |  | 0.467 |
| walker |  | 8909 | 0 | python method at src/pluggy/_manager.py:430 |  |  | 0.467 |
| walker |  | 8909 | 0 | python method at src/pluggy/_manager.py:434 |  |  | 0.467 |
| walker |  | 8909 | 0 | python method at src/pluggy/_manager.py:487 |  |  | 0.467 |
| ns | 8916 |  | 137 | CHANGELOG.rst recent-release headers | 6.1 |  | 0.464 |
| walker |  | 8935 | 26 | python method at src/pluggy/_manager.py:512 |  |  | 0.464 |
| walker |  | 8964 | 29 | python method at src/pluggy/_manager.py:450 |  |  | 0.464 |
| walker |  | 8983 | 19 | python method doc at src/pluggy/_manager.py:430 |  |  | 0.464 |
| walker |  | 8996 | 13 | python method body at src/pluggy/_manager.py:425 body 428 |  |  | 0.464 |
| walker |  | 9010 | 14 | python method body at src/pluggy/_manager.py:430 body 432 |  |  | 0.464 |
| walker |  | 9038 | 28 | python method doc at src/pluggy/_manager.py:425 |  |  | 0.464 |
| walker |  | 9073 | 35 | python method doc at src/pluggy/_manager.py:487 |  |  | 0.464 |
| ns | 9083 |  | 167 | Test files: per-file test-fn name lists (smaller files) | 6.2 |  | 0.460 |
| walker |  | 9123 | 50 | python method doc at src/pluggy/_manager.py:512 |  |  | 0.460 |
| walker |  | 9181 | 58 | python method doc at src/pluggy/_manager.py:434 |  |  | 0.460 |
| walker |  | 9345 | 164 | python method doc at src/pluggy/_manager.py:450 |  |  | 0.460 |
| walker |  | 9360 | 15 | python method body at src/pluggy/_manager.py:233 body 236 |  |  | 0.460 |
| walker |  | 9409 | 49 | python method body at src/pluggy/_tracing.py:17 body 18 |  |  | 0.460 |
| walker |  | 9427 | 18 | python decl names surface in docs/examples/eggsample/eggsample/__init__.py |  |  | 0.460 |
| walker |  | 9524 | 97 | python method at src/pluggy/_manager.py:114 |  |  | 0.460 |
| ns | 9525 |  | 442 | api_reference.rst - autoclass directives | 6.3 |  | 0.445 |
| walker |  | 9546 | 22 | python method body at src/pluggy/_manager.py:114 body 123 |  |  | 0.445 |
| walker |  | 9601 | 55 | python method body at src/pluggy/_tracing.py:51 body 52 |  |  | 0.445 |
| ns | 9618 |  | 93 | downstream/ + scripts/ + .github/ + changelog/ listings | 6.4 |  | 0.448 |
| walker |  | 9676 | 75 | python decl body at src/pluggy/_manager.py:42 body 43 |  |  | 0.448 |
| walker |  | 9695 | 19 | python method body at src/pluggy/_manager.py:304 body 312 |  |  | 0.448 |
| walker |  | 9703 | 8 | python method body at src/pluggy/_manager.py:252 body 258 |  |  | 0.448 |
| walker |  | 9711 | 8 | python method body at src/pluggy/_manager.py:278 body 294 |  |  | 0.448 |
| walker |  | 9919 | 208 | python imports in src/pluggy/_hooks.py |  |  | 0.448 |
| ns | 9930 |  | 312 | load_setuptools_entrypoints body | 6.5 | 3.2 | 0.443 |
| walker |  | 9932 | 13 | listing of '.github' |  |  | 0.447 |
| walker |  | 9936 | 4 | listing of '.github/workflows' |  |  | 0.449 |
