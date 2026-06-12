Score(3000)=0.647 I=0.851 C=0.492 ns_rows≤3K=17/40 (reached=8 partial=2 missing=7)

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
| ns | 2171 |  | 423 | Tutorial: toy-example.py in full | 3.3 |  | 0.614 |
| ns | 2213 |  | 42 | _callers.py top-level function names | 3.4 |  | 0.608 |
| ns | 2320 |  | 107 | _result.py class + method surface | 3.5 |  | 0.619 |
| ns | 2347 |  | 27 | _warnings.py classes | 3.6 |  | 0.622 |
| walker |  | 2532 | 614 | python method sigs in src/pluggy/_manager.py |  |  | 0.699 |
| walker |  | 2532 | 0 | python method at src/pluggy/_manager.py:59 |  |  | 0.699 |
| walker |  | 2532 | 0 | python method at src/pluggy/_manager.py:68 |  |  | 0.699 |
| walker |  | 2532 | 0 | python method at src/pluggy/_manager.py:76 |  |  | 0.699 |
| walker |  | 2532 | 0 | python method at src/pluggy/_manager.py:79 |  |  | 0.699 |
| walker |  | 2532 | 0 | python method at src/pluggy/_manager.py:125 |  |  | 0.699 |
| walker |  | 2532 | 0 | python method at src/pluggy/_manager.py:176 |  |  | 0.699 |
| walker |  | 2532 | 0 | python method at src/pluggy/_manager.py:233 |  |  | 0.699 |
| walker |  | 2532 | 0 | python method at src/pluggy/_manager.py:238 |  |  | 0.699 |
| walker |  | 2532 | 0 | python method at src/pluggy/_manager.py:242 |  |  | 0.699 |
| walker |  | 2532 | 0 | python method at src/pluggy/_manager.py:252 |  |  | 0.699 |
| walker |  | 2532 | 0 | python method at src/pluggy/_manager.py:296 |  |  | 0.699 |
| walker |  | 2532 | 0 | python method at src/pluggy/_manager.py:300 |  |  | 0.699 |
| walker |  | 2532 | 0 | python method at src/pluggy/_manager.py:304 |  |  | 0.699 |
| walker |  | 2532 | 0 | python method at src/pluggy/_manager.py:315 |  |  | 0.699 |
| walker |  | 2532 | 0 | python method at src/pluggy/_manager.py:319 |  |  | 0.699 |
| walker |  | 2532 | 0 | python method at src/pluggy/_manager.py:323 |  |  | 0.699 |
| walker |  | 2532 | 0 | python method at src/pluggy/_manager.py:331 |  |  | 0.699 |
| walker |  | 2532 | 0 | python method at src/pluggy/_manager.py:379 |  |  | 0.699 |
| walker |  | 2532 | 0 | python method at src/pluggy/_manager.py:395 |  |  | 0.699 |
| walker |  | 2532 | 0 | python method at src/pluggy/_manager.py:425 |  |  | 0.699 |
| walker |  | 2532 | 0 | python method at src/pluggy/_manager.py:430 |  |  | 0.699 |
| walker |  | 2532 | 0 | python method at src/pluggy/_manager.py:434 |  |  | 0.699 |
| walker |  | 2532 | 0 | python method at src/pluggy/_manager.py:487 |  |  | 0.699 |
| ns | 2533 |  | 186 | HookspecOpts TypedDict fields | 4.1 | 3.1 | 0.676 |
| walker |  | 2540 | 8 | python method at src/pluggy/_manager.py:100 |  |  | 0.676 |
| walker |  | 2551 | 11 | python method at src/pluggy/_manager.py:71 |  |  | 0.676 |
| walker |  | 2563 | 12 | python method doc at src/pluggy/_manager.py:300 |  |  | 0.676 |
| walker |  | 2589 | 26 | python method at src/pluggy/_manager.py:512 |  |  | 0.676 |
| walker |  | 2598 | 9 | python method body at src/pluggy/_manager.py:68 body 69 |  |  | 0.676 |
| walker |  | 2611 | 13 | python method doc at src/pluggy/_manager.py:238 |  |  | 0.676 |
| walker |  | 2624 | 13 | python method doc at src/pluggy/_manager.py:296 |  |  | 0.676 |
| walker |  | 2653 | 29 | python method at src/pluggy/_manager.py:278 |  |  | 0.676 |
| walker |  | 2682 | 29 | python method at src/pluggy/_manager.py:450 |  |  | 0.676 |
| walker |  | 2697 | 15 | python method doc at src/pluggy/_manager.py:319 |  |  | 0.676 |
| walker |  | 2713 | 16 | python method doc at src/pluggy/_manager.py:233 |  |  | 0.676 |
| walker |  | 2729 | 16 | python method doc at src/pluggy/_manager.py:315 |  |  | 0.676 |
| walker |  | 2762 | 33 | python method at src/pluggy/_manager.py:201 |  |  | 0.676 |
| walker |  | 2781 | 19 | python method doc at src/pluggy/_manager.py:430 |  |  | 0.647 |
| ns | 2781 |  | 248 | HookimplOpts TypedDict fields | 4.2 | 3.1 | 0.647 |
| walker |  | 2794 | 13 | python method body at src/pluggy/_manager.py:76 body 77 |  |  | 0.647 |
| walker |  | 2807 | 13 | python method body at src/pluggy/_manager.py:425 body 428 |  |  | 0.647 |
| walker |  | 2821 | 14 | python method body at src/pluggy/_manager.py:315 body 317 |  |  | 0.647 |
| walker |  | 2835 | 14 | python method body at src/pluggy/_manager.py:430 body 432 |  |  | 0.647 |
| walker |  | 2850 | 15 | python method body at src/pluggy/_manager.py:319 body 321 |  |  | 0.647 |
| walker |  | 2877 | 27 | python method doc at src/pluggy/_manager.py:323 |  |  | 0.647 |
| walker |  | 2905 | 28 | python method doc at src/pluggy/_manager.py:242 |  |  | 0.647 |
| walker |  | 2933 | 28 | python method doc at src/pluggy/_manager.py:425 |  |  | 0.647 |
| walker |  | 2952 | 19 | python method body at src/pluggy/_manager.py:79 body 80 |  |  | 0.647 |
| walker |  | 2972 | 20 | python method body at src/pluggy/_manager.py:300 body 302 |  |  | 0.647 |
| walker |  | 3007 | 35 | python method doc at src/pluggy/_manager.py:487 |  |  | 0.647 |
| walker |  | 3029 | 22 | python method body at src/pluggy/_manager.py:71 body 73 |  |  | 0.647 |
| ns | 3060 |  | 279 | docs/index.rst section heading map | 4.3 |  | 0.601 |
| walker |  | 3071 | 42 | python method doc at src/pluggy/_manager.py:379 |  |  | 0.601 |
| walker |  | 3095 | 24 | python method body at src/pluggy/_manager.py:238 body 240 |  |  | 0.601 |
| walker |  | 3119 | 24 | python method body at src/pluggy/_manager.py:296 body 298 |  |  | 0.601 |
| walker |  | 3168 | 49 | python method doc at src/pluggy/_result.py:80 |  |  | 0.601 |
| walker |  | 3218 | 50 | python method doc at src/pluggy/_manager.py:512 |  |  | 0.601 |
| ns | 3368 |  | 308 | HookCaller._add_hookimpl - the ordering algorithm | 4.4 |  | 0.576 |
| walker |  | 3399 | 181 | python decl names surface in src/pluggy/_hooks.py |  |  | 0.578 |
| walker |  | 3399 | 0 | python decl at src/pluggy/_hooks.py:40 |  |  | 0.578 |
| walker |  | 3399 | 0 | python decl at src/pluggy/_hooks.py:56 |  |  | 0.578 |
| walker |  | 3399 | 0 | python decl at src/pluggy/_hooks.py:281 |  |  | 0.578 |
| walker |  | 3406 | 7 | python decl at src/pluggy/_hooks.py:77 |  |  | 0.579 |
| walker |  | 3413 | 7 | python decl at src/pluggy/_hooks.py:164 |  |  | 0.581 |
| walker |  | 3423 | 10 | python decl doc at src/pluggy/_hooks.py:40 |  |  | 0.581 |
| walker |  | 3433 | 10 | python decl doc at src/pluggy/_hooks.py:56 |  |  | 0.582 |
| walker |  | 3447 | 14 | python class body at src/pluggy/_hooks.py:77 |  |  | 0.582 |
| walker |  | 3461 | 14 | python class body at src/pluggy/_hooks.py:164 |  |  | 0.582 |
| walker |  | 3687 | 226 | python class body at src/pluggy/_hooks.py:56 |  |  | 0.621 |
| walker |  | 3821 | 134 | python method sigs in src/pluggy/_hooks.py |  |  | 0.621 |
| walker |  | 3821 | 0 | python method at src/pluggy/_hooks.py:88 |  |  | 0.621 |
| walker |  | 3821 | 0 | python method at src/pluggy/_hooks.py:175 |  |  | 0.621 |
| walker |  | 3833 | 12 | python method body at src/pluggy/_hooks.py:88 body 89 |  |  | 0.621 |
| walker |  | 3845 | 12 | python method body at src/pluggy/_hooks.py:175 body 176 |  |  | 0.621 |
| walker |  | 4009 | 164 | python class body at src/pluggy/_hooks.py:40 |  |  | 0.649 |
| walker |  | 4086 | 77 | python decl doc at src/pluggy/_hooks.py:164 |  |  | 0.649 |
| walker |  | 4164 | 78 | python decl doc at src/pluggy/_hooks.py:77 |  |  | 0.649 |
| walker |  | 4200 | 36 | python decl at src/pluggy/_hooks.py:33 |  |  | 0.649 |
| walker |  | 4282 | 82 | python method at src/pluggy/_hooks.py:91 |  |  | 0.649 |
| walker |  | 4293 | 11 | python method body at src/pluggy/_hooks.py:91 body 99 |  |  | 0.649 |
| walker |  | 4349 | 56 | python method doc at src/pluggy/_manager.py:252 |  |  | 0.649 |
| ns | 4391 |  | 1023 | _multicall body - the call loop | 4.5 | 3.4 | 0.565 |
| walker |  | 4405 | 56 | python method doc at src/pluggy/_result.py:91 |  |  | 0.565 |
| walker |  | 4467 | 62 | listing of 'testing' |  |  | 0.565 |
| walker |  | 4525 | 58 | python method doc at src/pluggy/_manager.py:434 |  |  | 0.565 |
| walker |  | 4615 | 90 | python method at src/pluggy/_hooks.py:101 |  |  | 0.565 |
| walker |  | 4631 | 16 | python method body at src/pluggy/_hooks.py:101 body 109 |  |  | 0.565 |
| walker |  | 4686 | 55 | python decl names surface in src/pluggy/_tracing.py |  |  | 0.565 |
| walker |  | 4686 | 0 | python decl at src/pluggy/_tracing.py:16 |  |  | 0.565 |
| walker |  | 4686 | 0 | python decl at src/pluggy/_tracing.py:59 |  |  | 0.565 |
| walker |  | 4717 | 31 | python method body at src/pluggy/_manager.py:59 body 60 |  |  | 0.565 |
| walker |  | 4812 | 95 | python method at src/pluggy/_hooks.py:111 |  |  | 0.565 |
| walker |  | 4909 | 97 | python method at src/pluggy/_hooks.py:178 |  |  | 0.565 |
| walker |  | 4920 | 11 | python method body at src/pluggy/_hooks.py:178 body 188 |  |  | 0.565 |
| walker |  | 5025 | 105 | python method at src/pluggy/_hooks.py:190 |  |  | 0.565 |
| walker |  | 5041 | 16 | python method body at src/pluggy/_hooks.py:190 body 200 |  |  | 0.565 |
| ns | 5054 |  | 663 | PluginManager.register body | 4.6 | 3.2 | 0.531 |
| walker |  | 5109 | 68 | python decl names surface in src/pluggy/_callers.py |  |  | 0.538 |
| walker |  | 5138 | 29 | python decl at src/pluggy/_callers.py:27 |  |  | 0.538 |
| walker |  | 5169 | 31 | python decl doc at src/pluggy/_callers.py:27 |  |  | 0.538 |
| walker |  | 5196 | 27 | python decl at src/pluggy/_callers.py:70 |  |  | 0.538 |
| walker |  | 5228 | 32 | python decl at src/pluggy/_callers.py:60 |  |  | 0.538 |
| walker |  | 5236 | 8 | python method body at src/pluggy/_manager.py:242 body 250 |  |  | 0.538 |
| walker |  | 5244 | 8 | python method body at src/pluggy/_manager.py:323 body 329 |  |  | 0.538 |
| ns | 5262 |  | 208 | HookCaller.__call__ body | 4.7 | 3.1 | 0.528 |
| walker |  | 5321 | 77 | python method doc at src/pluggy/_manager.py:201 |  |  | 0.528 |
| walker |  | 5431 | 110 | python method at src/pluggy/_hooks.py:202 |  |  | 0.528 |
| ns | 5585 |  | 323 | HookCaller.call_historic body | 4.8 | 3.1 | 0.513 |
| walker |  | 5641 | 210 | python method sigs in src/pluggy/_tracing.py |  |  | 0.513 |
| walker |  | 5641 | 0 | python method at src/pluggy/_tracing.py:17 |  |  | 0.513 |
| walker |  | 5641 | 0 | python method at src/pluggy/_tracing.py:22 |  |  | 0.513 |
| walker |  | 5641 | 0 | python method at src/pluggy/_tracing.py:25 |  |  | 0.513 |
| walker |  | 5641 | 0 | python method at src/pluggy/_tracing.py:42 |  |  | 0.513 |
| walker |  | 5641 | 0 | python method at src/pluggy/_tracing.py:48 |  |  | 0.513 |
| walker |  | 5641 | 0 | python method at src/pluggy/_tracing.py:51 |  |  | 0.513 |
| walker |  | 5641 | 0 | python method at src/pluggy/_tracing.py:60 |  |  | 0.513 |
| walker |  | 5641 | 0 | python method at src/pluggy/_tracing.py:64 |  |  | 0.513 |
| walker |  | 5641 | 0 | python method at src/pluggy/_tracing.py:67 |  |  | 0.513 |
| walker |  | 5650 | 9 | python method body at src/pluggy/_tracing.py:48 body 49 |  |  | 0.513 |
| walker |  | 5662 | 12 | python method body at src/pluggy/_tracing.py:22 body 23 |  |  | 0.513 |
| walker |  | 5675 | 13 | python method body at src/pluggy/_tracing.py:64 body 65 |  |  | 0.513 |
| walker |  | 5692 | 17 | python method body at src/pluggy/_tracing.py:67 body 68 |  |  | 0.513 |
| walker |  | 5710 | 18 | python method body at src/pluggy/_tracing.py:60 body 61 |  |  | 0.513 |
| walker |  | 5714 | 4 | listing of '.claude' |  |  | 0.513 |
| walker |  | 5729 | 15 | listing of 'docs/examples' |  |  | 0.513 |
| walker |  | 5898 | 169 | python decl doc at src/pluggy/_manager.py:83 |  |  | 0.513 |
| ns | 6025 |  | 440 | HookCaller.call_extra body | 4.9 | 3.1 | 0.496 |
| walker |  | 6069 | 171 | python decl doc at src/pluggy/_warnings.py:10 |  |  | 0.496 |
| walker |  | 6182 | 113 | python decl names surface #1 in src/pluggy/_hooks.py |  |  | 0.503 |
| walker |  | 6182 | 0 | python decl at src/pluggy/_hooks.py:293 |  |  | 0.503 |
| walker |  | 6182 | 0 | python decl at src/pluggy/_hooks.py:382 |  |  | 0.503 |
| walker |  | 6182 | 0 | python decl at src/pluggy/_hooks.py:593 |  |  | 0.503 |
| walker |  | 6188 | 6 | python decl at src/pluggy/_hooks.py:358 |  |  | 0.505 |
| walker |  | 6194 | 6 | python decl at src/pluggy/_hooks.py:638 |  |  | 0.506 |
| walker |  | 6200 | 6 | python decl at src/pluggy/_hooks.py:696 |  |  | 0.506 |
| walker |  | 6215 | 15 | python decl doc at src/pluggy/_hooks.py:382 |  |  | 0.506 |
| walker |  | 6233 | 18 | python decl doc at src/pluggy/_hooks.py:638 |  |  | 0.506 |
| walker |  | 6264 | 31 | python decl doc at src/pluggy/_hooks.py:358 |  |  | 0.506 |
| walker |  | 6309 | 45 | python class body at src/pluggy/_hooks.py:358 |  |  | 0.506 |
| walker |  | 6421 | 112 | python class body at src/pluggy/_hooks.py:638 |  |  | 0.506 |
| walker |  | 6509 | 88 | python class body at src/pluggy/_hooks.py:696 |  |  | 0.506 |
| walker |  | 6597 | 88 | python class body at src/pluggy/_hooks.py:382 |  |  | 0.506 |
| walker |  | 6623 | 26 | python decl at src/pluggy/_hooks.py:377 |  |  | 0.506 |
| ns | 6694 |  | 669 | PluginManager._verify_hook body - validation rules | 4.10 |  | 0.484 |
| ns | 6748 |  | 54 | docs/examples listing (eggsample + eggsample-spam) | 5.1 |  | 0.480 |
| walker |  | 7007 | 384 | python method sigs #1 in src/pluggy/_hooks.py |  |  | 0.496 |
| walker |  | 7007 | 0 | python method at src/pluggy/_hooks.py:365 |  |  | 0.496 |
| walker |  | 7007 | 0 | python method at src/pluggy/_hooks.py:420 |  |  | 0.496 |
| walker |  | 7007 | 0 | python method at src/pluggy/_hooks.py:438 |  |  | 0.496 |
| walker |  | 7007 | 0 | python method at src/pluggy/_hooks.py:442 |  |  | 0.496 |
| walker |  | 7007 | 0 | python method at src/pluggy/_hooks.py:449 |  |  | 0.496 |
| walker |  | 7007 | 0 | python method at src/pluggy/_hooks.py:453 |  |  | 0.496 |
| walker |  | 7007 | 0 | python method at src/pluggy/_hooks.py:477 |  |  | 0.496 |
| walker |  | 7007 | 0 | python method at src/pluggy/_hooks.py:499 |  |  | 0.496 |
| walker |  | 7007 | 0 | python method at src/pluggy/_hooks.py:577 |  |  | 0.496 |
| walker |  | 7007 | 0 | python method at src/pluggy/_hooks.py:612 |  |  | 0.496 |
| walker |  | 7007 | 0 | python method at src/pluggy/_hooks.py:634 |  |  | 0.496 |
| walker |  | 7007 | 0 | python method at src/pluggy/_hooks.py:692 |  |  | 0.496 |
| walker |  | 7007 | 0 | python method at src/pluggy/_hooks.py:709 |  |  | 0.496 |
| ns | 7024 |  | 276 | eggsample hookspecs | 5.2 |  | 0.484 |
| walker |  | 7028 | 21 | python method at src/pluggy/_hooks.py:626 |  |  | 0.484 |
| walker |  | 7038 | 10 | python method doc at src/pluggy/_hooks.py:365 |  |  | 0.484 |
| walker |  | 7051 | 13 | python method doc at src/pluggy/_hooks.py:449 |  |  | 0.484 |
| walker |  | 7080 | 29 | python method at src/pluggy/_hooks.py:543 |  |  | 0.484 |
| walker |  | 7090 | 10 | python method body at src/pluggy/_hooks.py:420 body 421 |  |  | 0.484 |
| walker |  | 7101 | 11 | python method body at src/pluggy/_hooks.py:626 body 628 |  |  | 0.484 |
| walker |  | 7117 | 16 | python method doc at src/pluggy/_hooks.py:438 |  |  | 0.484 |
| walker |  | 7132 | 15 | python method at src/pluggy/_hooks.py:480 |  |  | 0.484 |
| walker |  | 7145 | 13 | python method body at src/pluggy/_hooks.py:449 body 451 |  |  | 0.484 |
| walker |  | 7184 | 39 | python method at src/pluggy/_hooks.py:424 |  |  | 0.484 |
| walker |  | 7200 | 16 | python method at src/pluggy/_hooks.py:618 |  |  | 0.484 |
| walker |  | 7214 | 14 | python method body at src/pluggy/_hooks.py:438 body 440 |  |  | 0.484 |
| walker |  | 7229 | 15 | python method body at src/pluggy/_hooks.py:477 body 478 |  |  | 0.484 |
| walker |  | 7241 | 12 | python method doc at src/pluggy/_hooks.py:453 |  |  | 0.484 |
| walker |  | 7258 | 17 | python method body at src/pluggy/_hooks.py:634 body 635 |  |  | 0.484 |
| ns | 7282 |  | 258 | eggsample host wiring + main() | 5.3 | 5.2 | 0.473 |
| walker |  | 7308 | 50 | python method at src/pluggy/_hooks.py:516 |  |  | 0.474 |
| walker |  | 7332 | 24 | python method at src/pluggy/_hooks.py:630 |  |  | 0.474 |
| walker |  | 7362 | 30 | python decl doc at src/pluggy/_hooks.py:593 |  |  | 0.474 |
| walker |  | 7423 | 61 | python method at src/pluggy/_hooks.py:656 |  |  | 0.474 |
| walker |  | 7435 | 12 | python method doc at src/pluggy/_hooks.py:656 |  |  | 0.474 |
| walker |  | 7507 | 72 | python decl doc at src/pluggy/_hooks.py:293 |  |  | 0.474 |
| ns | 7530 |  | 248 | eggsample-spam plugin code | 5.4 |  | 0.465 |
| walker |  | 7575 | 68 | python method at src/pluggy/_hooks.py:393 |  |  | 0.466 |
| walker |  | 7587 | 12 | python method doc at src/pluggy/_hooks.py:393 |  |  | 0.466 |
| walker |  | 7606 | 19 | python method doc at src/pluggy/_hooks.py:577 |  |  | 0.466 |
| walker |  | 7651 | 45 | python method doc at src/pluggy/_hooks.py:543 |  |  | 0.467 |
| walker |  | 7676 | 25 | python method body at src/pluggy/_hooks.py:692 body 693 |  |  | 0.467 |
| walker |  | 7689 | 13 | python method body at src/pluggy/_hooks.py:630 body 632 |  |  | 0.467 |
| ns | 7723 |  | 193 | eggsample + eggsample-spam setup.py: entry-point wiring | 5.5 |  | 0.460 |
| walker |  | 7754 | 65 | python method doc at src/pluggy/_hooks.py:499 |  |  | 0.463 |
| ns | 7785 |  | 62 | testing/ directory listing | 5.6 |  | 0.472 |
| walker |  | 7829 | 75 | python decl body at src/pluggy/_hooks.py:281 body 282 |  |  | 0.472 |
| walker |  | 7916 | 87 | python method doc at src/pluggy/_result.py:67 |  |  | 0.472 |
| walker |  | 8006 | 90 | python method doc at src/pluggy/_manager.py:304 |  |  | 0.472 |
| ns | 8033 |  | 248 | Shared test fixtures (conftest.py) | 5.7 |  | 0.463 |
| ns | 8065 |  | 32 | docs/ listing | 5.8 |  | 0.468 |
| walker |  | 8097 | 91 | python method doc at src/pluggy/_manager.py:395 |  |  | 0.468 |
| walker |  | 8190 | 93 | python method doc at src/pluggy/_hooks.py:516 |  |  | 0.474 |
| walker |  | 8200 | 10 | python method body at src/pluggy/_manager.py:233 body 235 |  |  | 0.474 |
| walker |  | 8413 | 213 | python class body at src/pluggy/_hooks.py:593 |  |  | 0.467 |
| ns | 8413 |  | 348 | HookCaller.__init__ + the hookimpls-list layout comment | 5.9 | 3.1 | 0.467 |
| walker |  | 8470 | 57 | python decl at src/pluggy/_callers.py:82 |  |  | 0.467 |
| walker |  | 8481 | 11 | python method body at src/pluggy/_result.py:42 body 45 |  |  | 0.467 |
| walker |  | 8601 | 120 | python method doc at src/pluggy/_manager.py:125 |  |  | 0.469 |
| walker |  | 8724 | 123 | python method doc at src/pluggy/_manager.py:176 |  |  | 0.469 |
| walker |  | 8775 | 51 | python decl doc at src/pluggy/_callers.py:82 |  |  | 0.470 |
| ns | 8779 |  | 366 | test_pluginmanager.py: test function name list | 5.10 |  | 0.459 |
| walker |  | 8876 | 101 | python imports in src/pluggy/_result.py |  |  | 0.459 |
| walker |  | 8888 | 12 | python method body at src/pluggy/_manager.py:304 body 313 |  |  | 0.459 |
| ns | 8916 |  | 137 | CHANGELOG.rst recent-release headers | 6.1 |  | 0.457 |
| walker |  | 8947 | 59 | python imports in src/pluggy/_tracing.py |  |  | 0.457 |
| walker |  | 9083 | 136 | python method doc at src/pluggy/_manager.py:278 |  |  | 0.453 |
| ns | 9083 |  | 167 | Test files: per-file test-fn name lists (smaller files) | 6.2 |  | 0.453 |
| walker |  | 9145 | 62 | python method body at src/pluggy/_hooks.py:612 body 613 |  |  | 0.453 |
| walker |  | 9153 | 8 | listing of 'docs/examples/eggsample' |  |  | 0.454 |
| walker |  | 9173 | 20 | listing of 'docs/examples/eggsample/eggsample' |  |  | 0.460 |
| walker |  | 9181 | 8 | python imports in docs/examples/eggsample/eggsample/__init__.py |  |  | 0.460 |
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
