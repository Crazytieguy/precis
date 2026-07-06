Score(3000)=0.615 I=0.712 C=0.531 ns_rows≤3K=16/40 (reached=6 partial=1 missing=9)

| source | ns_cum | walker_cum | marginal | descriptor | id | predecessor | Score(B=cum) |
|:-------|-------:|-----------:|---------:|:-----------|:---|:------------|-------------:|
| ns | 28 |  | 28 | README title + tagline | 1.1 |  | 0.000 |
| walker |  | 96 | 96 | listing of '.' |  |  | 0.000 |
| walker |  | 100 | 4 | listing of 'src' |  |  | 0.000 |
| ns | 114 |  | 86 | README badges line + who depends on pluggy + doc pointer | 1.2 |  | 0.000 |
| ns | 200 |  | 86 | Directory listing: docs/ and its worked-example subtree | 1.3 |  | 0.000 |
| walker |  | 219 | 119 | README headline in README.rst |  |  | 0.482 |
| walker |  | 262 | 43 | listing of 'src/pluggy' |  |  | 0.493 |
| ns | 293 |  | 93 | Directory listing: changelog, CI, downstream smoke-tests, scripts | 1.4 |  | 0.356 |
| ns | 445 |  | 152 | pluggy/__init__.py: __all__ (the literal public export list) | 1.5 |  | 0.295 |
| walker |  | 580 | 318 | python imports in src/pluggy/__init__.py |  |  | 0.541 |
| walker |  | 596 | 16 | python decl names surface in src/pluggy/__init__.py |  |  | 0.542 |
| walker |  | 596 | 0 | python decl at src/pluggy/__init__.py:32 |  |  | 0.542 |
| walker |  | 615 | 19 | listing of 'changelog' |  |  | 0.549 |
| ns | 646 |  | 201 | Directory listing: repo root, src/pluggy, testing | 1.6 |  | 0.560 |
| walker |  | 647 | 32 | listing of 'docs' |  |  | 0.594 |
| walker |  | 690 | 43 | listing of 'downstream' |  |  | 0.654 |
| walker |  | 743 | 53 | [package] in pyproject.toml |  |  | 0.654 |
| walker |  | 767 | 24 | python decl names surface in src/pluggy/_warnings.py |  |  | 0.654 |
| walker |  | 767 | 0 | python decl at src/pluggy/_warnings.py:4 |  |  | 0.654 |
| walker |  | 783 | 16 | python decl at src/pluggy/_warnings.py:10 |  |  | 0.654 |
| walker |  | 796 | 13 | python class body at src/pluggy/_warnings.py:4 |  |  | 0.654 |
| walker |  | 809 | 13 | python class body at src/pluggy/_warnings.py:10 |  |  | 0.654 |
| walker |  | 823 | 14 | python decl doc at src/pluggy/_warnings.py:4 |  |  | 0.654 |
| walker |  | 830 | 7 | python imports in src/pluggy/_warnings.py |  |  | 0.654 |
| walker |  | 833 | 3 | listing of 'docs/_static' |  |  | 0.654 |
| ns | 904 |  | 258 | pluggy/__init__.py: re-export imports + dynamic __version__ | 1.7 | 1.5 | 0.632 |
| ns | 1237 |  | 333 | README definitive example, part 1: markers + spec/impl namespaces | 1.8 |  | 0.549 |
| walker |  | 1379 | 546 | README.rst section #0 |  |  | 0.699 |
| walker |  | 1383 | 4 | listing of 'docs/_static/img' |  |  | 0.699 |
| walker |  | 1430 | 47 | plaintext config docs/requirements.txt |  |  | 0.699 |
| ns | 1445 |  | 208 | README definitive example, part 2: manager wiring + call + real output | 1.9 | 1.8 | 0.713 |
| ns | 1497 |  | 52 | _callers.py: every def location (signatures only) | 2.1 |  | 0.703 |
| walker |  | 1511 | 81 | python decl names surface in src/pluggy/_result.py |  |  | 0.704 |
| walker |  | 1511 | 0 | python decl at src/pluggy/_result.py:20 |  |  | 0.704 |
| walker |  | 1522 | 11 | python decl at src/pluggy/_result.py:24 |  |  | 0.704 |
| walker |  | 1531 | 9 | python decl doc at src/pluggy/_result.py:20 |  |  | 0.704 |
| walker |  | 1553 | 22 | python class body at src/pluggy/_result.py:24 |  |  | 0.704 |
| walker |  | 1585 | 32 | python decl doc at src/pluggy/_result.py:24 |  |  | 0.704 |
| walker |  | 1680 | 95 | python method sigs in src/pluggy/_result.py |  |  | 0.706 |
| walker |  | 1680 | 0 | python method at src/pluggy/_result.py:67 |  |  | 0.706 |
| walker |  | 1680 | 0 | python method at src/pluggy/_result.py:80 |  |  | 0.706 |
| walker |  | 1680 | 0 | python method at src/pluggy/_result.py:91 |  |  | 0.706 |
| walker |  | 1695 | 15 | python method at src/pluggy/_result.py:51 |  |  | 0.706 |
| walker |  | 1712 | 17 | python method at src/pluggy/_result.py:42 |  |  | 0.707 |
| walker |  | 1736 | 24 | python method at src/pluggy/_result.py:56 |  |  | 0.708 |
| walker |  | 1748 | 12 | python method doc at src/pluggy/_result.py:42 |  |  | 0.708 |
| walker |  | 1760 | 12 | python method doc at src/pluggy/_result.py:51 |  |  | 0.708 |
| walker |  | 1768 | 8 | python method body at src/pluggy/_result.py:51 body 54 |  |  | 0.708 |
| walker |  | 1780 | 12 | python method doc at src/pluggy/_result.py:56 |  |  | 0.708 |
| ns | 1815 |  | 318 | _result.py / _tracing.py / _warnings.py: every class/def location | 2.2 |  | 0.682 |
| walker |  | 1820 | 40 | python method at src/pluggy/_result.py:31 |  |  | 0.682 |
| walker |  | 1832 | 12 | python method doc at src/pluggy/_result.py:31 |  |  | 0.682 |
| walker |  | 1898 | 66 | python decl body at src/pluggy/__init__.py:32 body 33 |  |  | 0.713 |
| ns | 1951 |  | 136 | _hooks.py: every class/def location, part 1 (markers + helpers) | 2.3 |  | 0.691 |
| walker |  | 2018 | 120 | python decl names surface in src/pluggy/_manager.py |  |  | 0.691 |
| walker |  | 2018 | 0 | python decl at src/pluggy/_manager.py:42 |  |  | 0.691 |
| walker |  | 2018 | 0 | python decl at src/pluggy/_manager.py:52 |  |  | 0.691 |
| walker |  | 2018 | 0 | python decl at src/pluggy/_manager.py:65 |  |  | 0.691 |
| walker |  | 2018 | 0 | python decl at src/pluggy/_manager.py:83 |  |  | 0.691 |
| walker |  | 2018 | 0 | python decl at src/pluggy/_manager.py:525 |  |  | 0.691 |
| walker |  | 2031 | 13 | python decl doc at src/pluggy/_manager.py:65 |  |  | 0.691 |
| walker |  | 2079 | 48 | python decl doc at src/pluggy/_manager.py:52 |  |  | 0.691 |
| walker |  | 2105 | 26 | python decl at src/pluggy/_manager.py:37 |  |  | 0.691 |
| walker |  | 2121 | 16 | python decl body at src/pluggy/_manager.py:525 body 526 |  |  | 0.691 |
| ns | 2144 |  | 193 | _hooks.py: every class/def location, part 2 (HookRelay + the full HookCaller method list) | 2.4 |  | 0.663 |
| walker |  | 2281 | 160 | python decl names surface in src/pluggy/_hooks.py |  |  | 0.672 |
| walker |  | 2281 | 0 | python decl at src/pluggy/_hooks.py:40 |  |  | 0.672 |
| walker |  | 2281 | 0 | python decl at src/pluggy/_hooks.py:56 |  |  | 0.672 |
| walker |  | 2281 | 0 | python decl at src/pluggy/_hooks.py:281 |  |  | 0.672 |
| walker |  | 2281 | 0 | python decl at src/pluggy/_hooks.py:293 |  |  | 0.672 |
| walker |  | 2281 | 0 | python decl at src/pluggy/_hooks.py:382 |  |  | 0.672 |
| walker |  | 2281 | 0 | python decl at src/pluggy/_hooks.py:593 |  |  | 0.672 |
| walker |  | 2289 | 8 | python decl at src/pluggy/_hooks.py:358 |  |  | 0.673 |
| walker |  | 2297 | 8 | python decl at src/pluggy/_hooks.py:638 |  |  | 0.673 |
| walker |  | 2305 | 8 | python decl at src/pluggy/_hooks.py:696 |  |  | 0.673 |
| ns | 2306 |  | 162 | _hooks.py: every class/def location, part 3 (_SubsetHookCaller/HookImpl/HookSpec) | 2.5 |  | 0.654 |
| walker |  | 2314 | 9 | python decl at src/pluggy/_hooks.py:77 |  |  | 0.657 |
| walker |  | 2323 | 9 | python decl at src/pluggy/_hooks.py:164 |  |  | 0.660 |
| walker |  | 2335 | 12 | python decl doc at src/pluggy/_hooks.py:40 |  |  | 0.660 |
| walker |  | 2347 | 12 | python decl doc at src/pluggy/_hooks.py:56 |  |  | 0.660 |
| walker |  | 2363 | 16 | python class body at src/pluggy/_hooks.py:77 |  |  | 0.660 |
| walker |  | 2379 | 16 | python class body at src/pluggy/_hooks.py:164 |  |  | 0.660 |
| walker |  | 2396 | 17 | python decl doc at src/pluggy/_hooks.py:382 |  |  | 0.663 |
| walker |  | 2414 | 18 | python decl doc at src/pluggy/_hooks.py:638 |  |  | 0.663 |
| walker |  | 2445 | 31 | python decl doc at src/pluggy/_hooks.py:358 |  |  | 0.663 |
| walker |  | 2557 | 112 | python class body at src/pluggy/_hooks.py:638 |  |  | 0.663 |
| walker |  | 2609 | 52 | python class body at src/pluggy/_hooks.py:358 |  |  | 0.627 |
| ns | 2609 |  | 303 | _manager.py: every class/def location, part 1 (registration + introspection) | 2.6 |  | 0.627 |
| walker |  | 2697 | 88 | python class body at src/pluggy/_hooks.py:696 |  |  | 0.627 |
| ns | 2734 |  | 125 | _manager.py: every class/def location, part 2 (validation, pending, plugins, tracing) | 2.7 |  | 0.615 |
| walker |  | 2789 | 92 | python class body at src/pluggy/_hooks.py:382 |  |  | 0.615 |
| walker |  | 3013 | 224 | python class body at src/pluggy/_hooks.py:56 |  |  | 0.615 |
| walker |  | 3175 | 162 | python class body at src/pluggy/_hooks.py:40 |  |  | 0.615 |
| walker |  | 3351 | 176 | python method sigs in src/pluggy/_hooks.py |  |  | 0.634 |
| walker |  | 3351 | 0 | python method at src/pluggy/_hooks.py:88 |  |  | 0.634 |
| walker |  | 3351 | 0 | python method at src/pluggy/_hooks.py:175 |  |  | 0.634 |
| walker |  | 3351 | 0 | python method at src/pluggy/_hooks.py:365 |  |  | 0.634 |
| walker |  | 3351 | 0 | python method at src/pluggy/_hooks.py:420 |  |  | 0.634 |
| walker |  | 3361 | 10 | python method doc at src/pluggy/_hooks.py:365 |  |  | 0.634 |
| walker |  | 3371 | 10 | python method body at src/pluggy/_hooks.py:420 body 421 |  |  | 0.634 |
| ns | 3382 |  | 648 | HookspecMarker.__call__: full option docs (firstresult/historic/warn_on_impl) | 3.1 | 2.3 | 0.577 |
| walker |  | 3383 | 12 | python method body at src/pluggy/_hooks.py:88 body 89 |  |  | 0.577 |
| walker |  | 3395 | 12 | python method body at src/pluggy/_hooks.py:175 body 176 |  |  | 0.577 |
| walker |  | 3436 | 41 | python method at src/pluggy/_hooks.py:424 |  |  | 0.577 |
| walker |  | 3468 | 32 | python decl doc at src/pluggy/_hooks.py:593 |  |  | 0.581 |
| walker |  | 3548 | 80 | python decl doc at src/pluggy/_hooks.py:164 |  |  | 0.581 |
| walker |  | 3629 | 81 | python decl doc at src/pluggy/_hooks.py:77 |  |  | 0.581 |
| walker |  | 3699 | 70 | python method at src/pluggy/_hooks.py:393 |  |  | 0.581 |
| walker |  | 3711 | 12 | python method doc at src/pluggy/_hooks.py:393 |  |  | 0.581 |
| walker |  | 3790 | 79 | python decl doc at src/pluggy/_hooks.py:293 |  |  | 0.581 |
| ns | 3795 |  | 413 | HookCaller: hookimpl call-order algorithm (_add_hookimpl + its ordering comment) | 3.2 | 2.4 | 0.552 |
| walker |  | 4033 | 243 | python method sigs in src/pluggy/_manager.py |  |  | 0.569 |
| walker |  | 4033 | 0 | python method at src/pluggy/_manager.py:59 |  |  | 0.569 |
| walker |  | 4033 | 0 | python method at src/pluggy/_manager.py:68 |  |  | 0.569 |
| walker |  | 4033 | 0 | python method at src/pluggy/_manager.py:76 |  |  | 0.569 |
| walker |  | 4033 | 0 | python method at src/pluggy/_manager.py:79 |  |  | 0.569 |
| walker |  | 4033 | 0 | python method at src/pluggy/_manager.py:125 |  |  | 0.569 |
| walker |  | 4033 | 0 | python method at src/pluggy/_manager.py:176 |  |  | 0.569 |
| walker |  | 4033 | 0 | python method at src/pluggy/_manager.py:233 |  |  | 0.569 |
| walker |  | 4033 | 0 | python method at src/pluggy/_manager.py:238 |  |  | 0.569 |
| walker |  | 4043 | 10 | python method at src/pluggy/_manager.py:100 |  |  | 0.569 |
| walker |  | 4056 | 13 | python method at src/pluggy/_manager.py:71 |  |  | 0.572 |
| walker |  | 4065 | 9 | python method body at src/pluggy/_manager.py:68 body 69 |  |  | 0.572 |
| walker |  | 4080 | 15 | python method doc at src/pluggy/_manager.py:238 |  |  | 0.572 |
| walker |  | 4115 | 35 | python method at src/pluggy/_manager.py:201 |  |  | 0.572 |
| walker |  | 4133 | 18 | python method doc at src/pluggy/_manager.py:233 |  |  | 0.572 |
| walker |  | 4146 | 13 | python method body at src/pluggy/_manager.py:76 body 77 |  |  | 0.572 |
| walker |  | 4165 | 19 | python method body at src/pluggy/_manager.py:79 body 80 |  |  | 0.572 |
| walker |  | 4185 | 20 | python method body at src/pluggy/_manager.py:71 body 73 |  |  | 0.572 |
| walker |  | 4209 | 24 | python method body at src/pluggy/_manager.py:238 body 240 |  |  | 0.572 |
| walker |  | 4271 | 62 | listing of 'testing' |  |  | 0.617 |
| walker |  | 4355 | 84 | python method at src/pluggy/_hooks.py:91 |  |  | 0.617 |
| walker |  | 4364 | 9 | python method body at src/pluggy/_hooks.py:91 body 99 |  |  | 0.617 |
| walker |  | 4456 | 92 | python method at src/pluggy/_hooks.py:101 |  |  | 0.617 |
| walker |  | 4470 | 14 | python method body at src/pluggy/_hooks.py:101 body 109 |  |  | 0.617 |
| walker |  | 4531 | 61 | python method doc at src/pluggy/_result.py:80 |  |  | 0.617 |
| walker |  | 4628 | 97 | python method at src/pluggy/_hooks.py:111 |  |  | 0.619 |
| walker |  | 4691 | 63 | python method doc at src/pluggy/_result.py:91 |  |  | 0.619 |
| walker |  | 4790 | 99 | python method at src/pluggy/_hooks.py:178 |  |  | 0.619 |
| walker |  | 4799 | 9 | python method body at src/pluggy/_hooks.py:178 body 188 |  |  | 0.619 |
| ns | 4819 |  | 1024 | HookimplMarker.__call__: full option docs (wrapper/hookwrapper/optionalhook/tryfirst/trylast/specname) | 3.3 | 2.3 | 0.556 |
| walker |  | 4830 | 31 | python method body at src/pluggy/_manager.py:59 body 60 |  |  | 0.556 |
| walker |  | 4897 | 67 | python decl names surface in src/pluggy/_tracing.py |  |  | 0.560 |
| walker |  | 4897 | 0 | python decl at src/pluggy/_tracing.py:16 |  |  | 0.560 |
| walker |  | 4897 | 0 | python decl at src/pluggy/_tracing.py:59 |  |  | 0.560 |
| walker |  | 5004 | 107 | python method at src/pluggy/_hooks.py:190 |  |  | 0.560 |
| walker |  | 5018 | 14 | python method body at src/pluggy/_hooks.py:190 body 200 |  |  | 0.560 |
| walker |  | 5033 | 15 | listing of 'docs/examples' |  |  | 0.575 |
| walker |  | 5243 | 210 | python method sigs in src/pluggy/_tracing.py |  |  | 0.598 |
| walker |  | 5243 | 0 | python method at src/pluggy/_tracing.py:17 |  |  | 0.598 |
| walker |  | 5243 | 0 | python method at src/pluggy/_tracing.py:22 |  |  | 0.598 |
| walker |  | 5243 | 0 | python method at src/pluggy/_tracing.py:25 |  |  | 0.598 |
| walker |  | 5243 | 0 | python method at src/pluggy/_tracing.py:42 |  |  | 0.598 |
| walker |  | 5243 | 0 | python method at src/pluggy/_tracing.py:48 |  |  | 0.598 |
| walker |  | 5243 | 0 | python method at src/pluggy/_tracing.py:51 |  |  | 0.598 |
| walker |  | 5243 | 0 | python method at src/pluggy/_tracing.py:60 |  |  | 0.598 |
| walker |  | 5243 | 0 | python method at src/pluggy/_tracing.py:64 |  |  | 0.598 |
| walker |  | 5243 | 0 | python method at src/pluggy/_tracing.py:67 |  |  | 0.598 |
| walker |  | 5252 | 9 | python method body at src/pluggy/_tracing.py:48 body 49 |  |  | 0.598 |
| walker |  | 5264 | 12 | python method body at src/pluggy/_tracing.py:22 body 23 |  |  | 0.598 |
| walker |  | 5277 | 13 | python method body at src/pluggy/_tracing.py:64 body 65 |  |  | 0.598 |
| walker |  | 5294 | 17 | python method body at src/pluggy/_tracing.py:67 body 68 |  |  | 0.598 |
| walker |  | 5312 | 18 | python method body at src/pluggy/_tracing.py:60 body 61 |  |  | 0.598 |
| walker |  | 5316 | 4 | listing of '.claude' |  |  | 0.598 |
| walker |  | 5428 | 112 | python method at src/pluggy/_hooks.py:202 |  |  | 0.599 |
| walker |  | 5508 | 80 | python decl names surface in src/pluggy/_callers.py |  |  | 0.606 |
| walker |  | 5539 | 31 | python decl at src/pluggy/_callers.py:27 |  |  | 0.606 |
| walker |  | 5570 | 31 | python decl doc at src/pluggy/_callers.py:27 |  |  | 0.606 |
| walker |  | 5599 | 29 | python decl at src/pluggy/_callers.py:70 |  |  | 0.606 |
| walker |  | 5633 | 34 | python decl at src/pluggy/_callers.py:60 |  |  | 0.606 |
| ns | 5840 |  | 1021 | _multicall(): full body -- the actual call loop | 3.4 | 2.1 | 0.544 |
| walker |  | 5869 | 236 | python method sigs #1 in src/pluggy/_hooks.py |  |  | 0.564 |
| walker |  | 5869 | 0 | python method at src/pluggy/_hooks.py:438 |  |  | 0.564 |
| walker |  | 5869 | 0 | python method at src/pluggy/_hooks.py:442 |  |  | 0.564 |
| walker |  | 5869 | 0 | python method at src/pluggy/_hooks.py:449 |  |  | 0.564 |
| walker |  | 5869 | 0 | python method at src/pluggy/_hooks.py:453 |  |  | 0.564 |
| walker |  | 5869 | 0 | python method at src/pluggy/_hooks.py:477 |  |  | 0.564 |
| walker |  | 5869 | 0 | python method at src/pluggy/_hooks.py:499 |  |  | 0.564 |
| walker |  | 5869 | 0 | python method at src/pluggy/_hooks.py:577 |  |  | 0.564 |
| walker |  | 5869 | 0 | python method at src/pluggy/_hooks.py:612 |  |  | 0.564 |
| walker |  | 5900 | 31 | python method at src/pluggy/_hooks.py:543 |  |  | 0.564 |
| walker |  | 5915 | 15 | python method doc at src/pluggy/_hooks.py:449 |  |  | 0.564 |
| walker |  | 5926 | 11 | python method body at src/pluggy/_hooks.py:449 body 451 |  |  | 0.564 |
| walker |  | 5944 | 18 | python method doc at src/pluggy/_hooks.py:438 |  |  | 0.564 |
| walker |  | 5956 | 12 | python method body at src/pluggy/_hooks.py:438 body 440 |  |  | 0.564 |
| walker |  | 5973 | 17 | python method at src/pluggy/_hooks.py:480 |  |  | 0.564 |
| walker |  | 5991 | 18 | python method at src/pluggy/_hooks.py:618 |  |  | 0.566 |
| walker |  | 6006 | 15 | python method body at src/pluggy/_hooks.py:477 body 478 |  |  | 0.566 |
| walker |  | 6058 | 52 | python method at src/pluggy/_hooks.py:516 |  |  | 0.566 |
| walker |  | 6072 | 14 | python method doc at src/pluggy/_hooks.py:453 |  |  | 0.566 |
| walker |  | 6093 | 21 | python method doc at src/pluggy/_hooks.py:577 |  |  | 0.566 |
| ns | 6113 |  | 273 | PluggyWarning / PluggyTeardownRaisedWarning: full body | 3.5 | 2.2 | 0.552 |
| walker |  | 6138 | 45 | python method doc at src/pluggy/_hooks.py:543 |  |  | 0.552 |
| walker |  | 6215 | 77 | python method doc at src/pluggy/_hooks.py:499 |  |  | 0.552 |
| walker |  | 6302 | 87 | python method doc at src/pluggy/_manager.py:201 |  |  | 0.552 |
| ns | 6545 |  | 432 | Test roster: test_pluginmanager.py (32 tests) | 4.1 |  | 0.535 |
| walker |  | 6547 | 245 | python method sigs #1 in src/pluggy/_manager.py |  |  | 0.557 |
| walker |  | 6547 | 0 | python method at src/pluggy/_manager.py:242 |  |  | 0.557 |
| walker |  | 6547 | 0 | python method at src/pluggy/_manager.py:252 |  |  | 0.557 |
| walker |  | 6547 | 0 | python method at src/pluggy/_manager.py:296 |  |  | 0.557 |
| walker |  | 6547 | 0 | python method at src/pluggy/_manager.py:300 |  |  | 0.557 |
| walker |  | 6547 | 0 | python method at src/pluggy/_manager.py:304 |  |  | 0.557 |
| walker |  | 6547 | 0 | python method at src/pluggy/_manager.py:315 |  |  | 0.557 |
| walker |  | 6547 | 0 | python method at src/pluggy/_manager.py:319 |  |  | 0.557 |
| walker |  | 6547 | 0 | python method at src/pluggy/_manager.py:323 |  |  | 0.557 |
| walker |  | 6547 | 0 | python method at src/pluggy/_manager.py:331 |  |  | 0.557 |
| walker |  | 6547 | 0 | python method at src/pluggy/_manager.py:379 |  |  | 0.557 |
| walker |  | 6547 | 0 | python method at src/pluggy/_manager.py:395 |  |  | 0.557 |
| walker |  | 6561 | 14 | python method doc at src/pluggy/_manager.py:300 |  |  | 0.557 |
| walker |  | 6592 | 31 | python method at src/pluggy/_manager.py:278 |  |  | 0.557 |
| walker |  | 6607 | 15 | python method doc at src/pluggy/_manager.py:296 |  |  | 0.557 |
| walker |  | 6624 | 17 | python method doc at src/pluggy/_manager.py:319 |  |  | 0.557 |
| walker |  | 6642 | 18 | python method doc at src/pluggy/_manager.py:315 |  |  | 0.557 |
| walker |  | 6654 | 12 | python method body at src/pluggy/_manager.py:315 body 317 |  |  | 0.557 |
| walker |  | 6667 | 13 | python method body at src/pluggy/_manager.py:319 body 321 |  |  | 0.557 |
| ns | 6694 |  | 149 | Test roster: test_details.py (10 tests) | 4.2 |  | 0.552 |
| walker |  | 6696 | 29 | python method doc at src/pluggy/_manager.py:323 |  |  | 0.552 |
| walker |  | 6714 | 18 | python method body at src/pluggy/_manager.py:300 body 302 |  |  | 0.552 |
| walker |  | 6749 | 35 | python method doc at src/pluggy/_manager.py:242 |  |  | 0.552 |
| walker |  | 6771 | 22 | python method body at src/pluggy/_manager.py:296 body 298 |  |  | 0.552 |
| walker |  | 6815 | 44 | python method doc at src/pluggy/_manager.py:379 |  |  | 0.552 |
| walker |  | 6878 | 63 | python method doc at src/pluggy/_manager.py:252 |  |  | 0.552 |
| walker |  | 6886 | 8 | python method body at src/pluggy/_manager.py:242 body 250 |  |  | 0.552 |
| walker |  | 6894 | 8 | python method body at src/pluggy/_manager.py:323 body 329 |  |  | 0.552 |
| ns | 6913 |  | 219 | Test roster: test_hookcaller.py (15 tests) | 4.3 |  | 0.544 |
| walker |  | 6971 | 77 | python decl body at src/pluggy/_hooks.py:281 body 282 |  |  | 0.544 |
| ns | 7085 |  | 172 | Test roster: test_invocations.py (13 tests) | 4.4 |  | 0.538 |
| walker |  | 7160 | 189 | python decl doc at src/pluggy/_manager.py:83 |  |  | 0.538 |
| walker |  | 7349 | 189 | python decl doc at src/pluggy/_warnings.py:10 |  |  | 0.560 |
| ns | 7372 |  | 287 | Test roster: test_multicall.py (21 tests) | 4.5 |  | 0.549 |
| walker |  | 7446 | 97 | python method doc at src/pluggy/_manager.py:304 |  |  | 0.549 |
| walker |  | 7544 | 98 | python method doc at src/pluggy/_hooks.py:516 |  |  | 0.549 |
| ns | 7620 |  | 248 | testing/conftest.py: full body (shared pm / he_pm fixtures) | 4.6 |  | 0.538 |
| walker |  | 7643 | 99 | python method doc at src/pluggy/_result.py:67 |  |  | 0.538 |
| walker |  | 7746 | 103 | python method doc at src/pluggy/_manager.py:395 |  |  | 0.538 |
| ns | 7848 |  | 228 | docs/api_reference.rst: autodoc roster (which classes are documented, in order) | 5.1 |  | 0.531 |
| walker |  | 7962 | 216 | python class body at src/pluggy/_hooks.py:593 |  |  | 0.531 |
| walker |  | 8021 | 59 | python decl at src/pluggy/_callers.py:82 |  |  | 0.531 |
| ns | 8153 |  | 305 | docs/index.rst: 'How does it work?' overview | 5.2 |  | 0.523 |
| walker |  | 8174 | 153 | python decl names surface #1 in src/pluggy/_hooks.py |  |  | 0.523 |
| walker |  | 8198 | 24 | python decl at src/pluggy/_hooks.py:377 |  |  | 0.523 |
| walker |  | 8232 | 34 | python decl at src/pluggy/_hooks.py:33 |  |  | 0.523 |
| walker |  | 8242 | 10 | python method body at src/pluggy/_manager.py:233 body 235 |  |  | 0.523 |
| ns | 8302 |  | 149 | pyproject.toml: every distinct [section] header (config-key roster) | 6.1 |  | 0.518 |
| walker |  | 8332 | 90 | python method sigs #2 in src/pluggy/_hooks.py |  |  | 0.524 |
| walker |  | 8332 | 0 | python method at src/pluggy/_hooks.py:634 |  |  | 0.524 |
| walker |  | 8332 | 0 | python method at src/pluggy/_hooks.py:692 |  |  | 0.524 |
| walker |  | 8332 | 0 | python method at src/pluggy/_hooks.py:709 |  |  | 0.524 |
| walker |  | 8355 | 23 | python method at src/pluggy/_hooks.py:626 |  |  | 0.526 |
| walker |  | 8364 | 9 | python method body at src/pluggy/_hooks.py:626 body 628 |  |  | 0.526 |
| walker |  | 8381 | 17 | python method body at src/pluggy/_hooks.py:634 body 635 |  |  | 0.526 |
| walker |  | 8444 | 63 | python method at src/pluggy/_hooks.py:656 |  |  | 0.526 |
| walker |  | 8456 | 12 | python method doc at src/pluggy/_hooks.py:656 |  |  | 0.526 |
| walker |  | 8482 | 26 | python method at src/pluggy/_hooks.py:630 |  |  | 0.528 |
| walker |  | 8493 | 11 | python method body at src/pluggy/_hooks.py:630 body 632 |  |  | 0.528 |
| ns | 8496 |  | 194 | tox.ini: [tox] envlist + [testenv] | 6.2 |  | 0.522 |
| walker |  | 8518 | 25 | python method body at src/pluggy/_hooks.py:692 body 693 |  |  | 0.522 |
| walker |  | 8529 | 11 | python method body at src/pluggy/_result.py:42 body 45 |  |  | 0.522 |
| walker |  | 8585 | 56 | python decl doc at src/pluggy/_callers.py:82 |  |  | 0.523 |
| ns | 8674 |  | 178 | tox.ini: [testenv:docs] + [pytest] | 6.3 | 6.2 | 0.517 |
| walker |  | 8694 | 109 | python imports in src/pluggy/_result.py |  |  | 0.517 |
| walker |  | 8702 | 8 | listing of 'docs/examples/eggsample' |  |  | 0.527 |
| walker |  | 8722 | 20 | listing of 'docs/examples/eggsample/eggsample' |  |  | 0.550 |
| walker |  | 8732 | 10 | python imports in docs/examples/eggsample/eggsample/__init__.py |  |  | 0.550 |
| ns | 8786 |  | 112 | tox.ini: [testenv:release] | 6.4 | 6.2 | 0.546 |
| walker |  | 8867 | 135 | python method doc at src/pluggy/_manager.py:176 |  |  | 0.546 |
| walker |  | 8879 | 12 | python method body at src/pluggy/_manager.py:304 body 313 |  |  | 0.546 |
| ns | 8988 |  | 202 | .pre-commit-config.yaml: repo + hook-id roster | 6.5 |  | 0.541 |
| walker |  | 9016 | 137 | python method doc at src/pluggy/_manager.py:125 |  |  | 0.541 |
| ns | 9039 |  | 51 | .coveragerc: [run] section (what coverage.py tracks) | 6.6 |  | 0.538 |
| ns | 9102 |  | 63 | MANIFEST.in: full (sdist inclusion rules) | 6.7 |  | 0.535 |
| walker |  | 9136 | 120 | python method sigs #2 in src/pluggy/_manager.py |  |  | 0.544 |
| walker |  | 9136 | 0 | python method at src/pluggy/_manager.py:425 |  |  | 0.544 |
| walker |  | 9136 | 0 | python method at src/pluggy/_manager.py:430 |  |  | 0.544 |
| walker |  | 9136 | 0 | python method at src/pluggy/_manager.py:434 |  |  | 0.544 |
| walker |  | 9136 | 0 | python method at src/pluggy/_manager.py:487 |  |  | 0.544 |
| walker |  | 9164 | 28 | python method at src/pluggy/_manager.py:512 |  |  | 0.544 |
| walker |  | 9195 | 31 | python method at src/pluggy/_manager.py:450 |  |  | 0.544 |
| walker |  | 9208 | 13 | python method body at src/pluggy/_manager.py:425 body 428 |  |  | 0.544 |
| ns | 9212 |  | 110 | CHANGELOG.rst: pluggy 1.6.0 -- version header + deprecations | 7.1 |  | 0.541 |
| walker |  | 9229 | 21 | python method doc at src/pluggy/_manager.py:430 |  |  | 0.541 |
| walker |  | 9241 | 12 | python method body at src/pluggy/_manager.py:430 body 432 |  |  | 0.541 |
| walker |  | 9269 | 28 | python method doc at src/pluggy/_manager.py:425 |  |  | 0.541 |
| walker |  | 9311 | 42 | python method doc at src/pluggy/_manager.py:487 |  |  | 0.541 |
| walker |  | 9361 | 50 | python method doc at src/pluggy/_manager.py:512 |  |  | 0.541 |
| walker |  | 9426 | 65 | python method doc at src/pluggy/_manager.py:434 |  |  | 0.541 |
| ns | 9443 |  | 231 | CHANGELOG.rst: pluggy 1.6.0 -- bug fixes | 7.2 | 7.1 | 0.536 |
| walker |  | 9572 | 146 | python method doc at src/pluggy/_manager.py:278 |  |  | 0.536 |
| walker |  | 9639 | 67 | python imports in src/pluggy/_tracing.py |  |  | 0.536 |
| ns | 9646 |  | 203 | changelog/README.rst: newsfragment type taxonomy + naming convention | 7.3 |  | 0.532 |
| walker |  | 9652 | 13 | python method body at src/pluggy/_manager.py:233 body 236 |  |  | 0.532 |
| ns | 9971 |  | 325 | RELEASING.rst: full release procedure | 7.4 |  | 0.522 |
