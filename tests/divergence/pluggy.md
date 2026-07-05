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
| walker |  | 714 | 24 | python decl names surface in src/pluggy/_warnings.py |  |  | 0.654 |
| walker |  | 714 | 0 | python decl at src/pluggy/_warnings.py:4 |  |  | 0.654 |
| walker |  | 730 | 16 | python decl at src/pluggy/_warnings.py:10 |  |  | 0.654 |
| walker |  | 743 | 13 | python class body at src/pluggy/_warnings.py:4 |  |  | 0.654 |
| walker |  | 756 | 13 | python class body at src/pluggy/_warnings.py:10 |  |  | 0.654 |
| walker |  | 770 | 14 | python decl doc at src/pluggy/_warnings.py:4 |  |  | 0.654 |
| walker |  | 777 | 7 | python imports in src/pluggy/_warnings.py |  |  | 0.654 |
| walker |  | 830 | 53 | [package] in pyproject.toml |  |  | 0.654 |
| ns | 904 |  | 258 | pluggy/__init__.py: re-export imports + dynamic __version__ | 1.7 | 1.5 | 0.632 |
| ns | 1237 |  | 333 | README definitive example, part 1: markers + spec/impl namespaces | 1.8 |  | 0.549 |
| walker |  | 1376 | 546 | README.rst section #0 |  |  | 0.699 |
| walker |  | 1379 | 3 | listing of 'docs/_static' |  |  | 0.699 |
| walker |  | 1383 | 4 | listing of 'docs/_static/img' |  |  | 0.699 |
| ns | 1445 |  | 208 | README definitive example, part 2: manager wiring + call + real output | 1.9 | 1.8 | 0.713 |
| walker |  | 1449 | 66 | python decl body at src/pluggy/__init__.py:32 body 33 |  |  | 0.747 |
| ns | 1497 |  | 52 | _callers.py: every def location (signatures only) | 2.1 |  | 0.738 |
| walker |  | 1530 | 81 | python decl names surface in src/pluggy/_result.py |  |  | 0.738 |
| walker |  | 1530 | 0 | python decl at src/pluggy/_result.py:20 |  |  | 0.738 |
| walker |  | 1541 | 11 | python decl at src/pluggy/_result.py:24 |  |  | 0.738 |
| walker |  | 1550 | 9 | python decl doc at src/pluggy/_result.py:20 |  |  | 0.738 |
| walker |  | 1572 | 22 | python class body at src/pluggy/_result.py:24 |  |  | 0.738 |
| walker |  | 1604 | 32 | python decl doc at src/pluggy/_result.py:24 |  |  | 0.738 |
| walker |  | 1699 | 95 | python method sigs in src/pluggy/_result.py |  |  | 0.740 |
| walker |  | 1699 | 0 | python method at src/pluggy/_result.py:67 |  |  | 0.740 |
| walker |  | 1699 | 0 | python method at src/pluggy/_result.py:80 |  |  | 0.740 |
| walker |  | 1699 | 0 | python method at src/pluggy/_result.py:91 |  |  | 0.740 |
| walker |  | 1714 | 15 | python method at src/pluggy/_result.py:51 |  |  | 0.740 |
| walker |  | 1731 | 17 | python method at src/pluggy/_result.py:42 |  |  | 0.741 |
| walker |  | 1755 | 24 | python method at src/pluggy/_result.py:56 |  |  | 0.742 |
| walker |  | 1767 | 12 | python method doc at src/pluggy/_result.py:42 |  |  | 0.742 |
| walker |  | 1779 | 12 | python method doc at src/pluggy/_result.py:51 |  |  | 0.742 |
| walker |  | 1787 | 8 | python method body at src/pluggy/_result.py:51 body 54 |  |  | 0.742 |
| walker |  | 1799 | 12 | python method doc at src/pluggy/_result.py:56 |  |  | 0.742 |
| ns | 1815 |  | 318 | _result.py / _tracing.py / _warnings.py: every class/def location | 2.2 |  | 0.713 |
| walker |  | 1839 | 40 | python method at src/pluggy/_result.py:31 |  |  | 0.713 |
| walker |  | 1851 | 12 | python method doc at src/pluggy/_result.py:31 |  |  | 0.713 |
| ns | 1951 |  | 136 | _hooks.py: every class/def location, part 1 (markers + helpers) | 2.3 |  | 0.691 |
| walker |  | 1971 | 120 | python decl names surface in src/pluggy/_manager.py |  |  | 0.691 |
| walker |  | 1971 | 0 | python decl at src/pluggy/_manager.py:42 |  |  | 0.691 |
| walker |  | 1971 | 0 | python decl at src/pluggy/_manager.py:52 |  |  | 0.691 |
| walker |  | 1971 | 0 | python decl at src/pluggy/_manager.py:65 |  |  | 0.691 |
| walker |  | 1971 | 0 | python decl at src/pluggy/_manager.py:83 |  |  | 0.691 |
| walker |  | 1971 | 0 | python decl at src/pluggy/_manager.py:525 |  |  | 0.691 |
| walker |  | 1984 | 13 | python decl doc at src/pluggy/_manager.py:65 |  |  | 0.691 |
| walker |  | 2032 | 48 | python decl doc at src/pluggy/_manager.py:52 |  |  | 0.691 |
| walker |  | 2058 | 26 | python decl at src/pluggy/_manager.py:37 |  |  | 0.691 |
| walker |  | 2074 | 16 | python decl body at src/pluggy/_manager.py:525 body 526 |  |  | 0.691 |
| ns | 2144 |  | 193 | _hooks.py: every class/def location, part 2 (HookRelay + the full HookCaller method list) | 2.4 |  | 0.663 |
| walker |  | 2234 | 160 | python decl names surface in src/pluggy/_hooks.py |  |  | 0.672 |
| walker |  | 2234 | 0 | python decl at src/pluggy/_hooks.py:40 |  |  | 0.672 |
| walker |  | 2234 | 0 | python decl at src/pluggy/_hooks.py:56 |  |  | 0.672 |
| walker |  | 2234 | 0 | python decl at src/pluggy/_hooks.py:281 |  |  | 0.672 |
| walker |  | 2234 | 0 | python decl at src/pluggy/_hooks.py:293 |  |  | 0.672 |
| walker |  | 2234 | 0 | python decl at src/pluggy/_hooks.py:382 |  |  | 0.672 |
| walker |  | 2234 | 0 | python decl at src/pluggy/_hooks.py:593 |  |  | 0.672 |
| walker |  | 2242 | 8 | python decl at src/pluggy/_hooks.py:358 |  |  | 0.673 |
| walker |  | 2250 | 8 | python decl at src/pluggy/_hooks.py:638 |  |  | 0.673 |
| walker |  | 2258 | 8 | python decl at src/pluggy/_hooks.py:696 |  |  | 0.673 |
| walker |  | 2267 | 9 | python decl at src/pluggy/_hooks.py:77 |  |  | 0.676 |
| walker |  | 2276 | 9 | python decl at src/pluggy/_hooks.py:164 |  |  | 0.680 |
| walker |  | 2288 | 12 | python decl doc at src/pluggy/_hooks.py:40 |  |  | 0.680 |
| walker |  | 2300 | 12 | python decl doc at src/pluggy/_hooks.py:56 |  |  | 0.680 |
| ns | 2306 |  | 162 | _hooks.py: every class/def location, part 3 (_SubsetHookCaller/HookImpl/HookSpec) | 2.5 |  | 0.660 |
| walker |  | 2316 | 16 | python class body at src/pluggy/_hooks.py:77 |  |  | 0.660 |
| walker |  | 2332 | 16 | python class body at src/pluggy/_hooks.py:164 |  |  | 0.660 |
| walker |  | 2349 | 17 | python decl doc at src/pluggy/_hooks.py:382 |  |  | 0.663 |
| walker |  | 2367 | 18 | python decl doc at src/pluggy/_hooks.py:638 |  |  | 0.663 |
| walker |  | 2398 | 31 | python decl doc at src/pluggy/_hooks.py:358 |  |  | 0.663 |
| walker |  | 2510 | 112 | python class body at src/pluggy/_hooks.py:638 |  |  | 0.663 |
| walker |  | 2562 | 52 | python class body at src/pluggy/_hooks.py:358 |  |  | 0.663 |
| ns | 2609 |  | 303 | _manager.py: every class/def location, part 1 (registration + introspection) | 2.6 |  | 0.627 |
| walker |  | 2650 | 88 | python class body at src/pluggy/_hooks.py:696 |  |  | 0.627 |
| ns | 2734 |  | 125 | _manager.py: every class/def location, part 2 (validation, pending, plugins, tracing) | 2.7 |  | 0.615 |
| walker |  | 2742 | 92 | python class body at src/pluggy/_hooks.py:382 |  |  | 0.615 |
| walker |  | 2966 | 224 | python class body at src/pluggy/_hooks.py:56 |  |  | 0.615 |
| walker |  | 3128 | 162 | python class body at src/pluggy/_hooks.py:40 |  |  | 0.615 |
| walker |  | 3304 | 176 | python method sigs in src/pluggy/_hooks.py |  |  | 0.634 |
| walker |  | 3304 | 0 | python method at src/pluggy/_hooks.py:88 |  |  | 0.634 |
| walker |  | 3304 | 0 | python method at src/pluggy/_hooks.py:175 |  |  | 0.634 |
| walker |  | 3304 | 0 | python method at src/pluggy/_hooks.py:365 |  |  | 0.634 |
| walker |  | 3304 | 0 | python method at src/pluggy/_hooks.py:420 |  |  | 0.634 |
| walker |  | 3314 | 10 | python method doc at src/pluggy/_hooks.py:365 |  |  | 0.634 |
| walker |  | 3324 | 10 | python method body at src/pluggy/_hooks.py:420 body 421 |  |  | 0.634 |
| walker |  | 3336 | 12 | python method body at src/pluggy/_hooks.py:88 body 89 |  |  | 0.634 |
| walker |  | 3348 | 12 | python method body at src/pluggy/_hooks.py:175 body 176 |  |  | 0.634 |
| ns | 3382 |  | 648 | HookspecMarker.__call__: full option docs (firstresult/historic/warn_on_impl) | 3.1 | 2.3 | 0.577 |
| walker |  | 3389 | 41 | python method at src/pluggy/_hooks.py:424 |  |  | 0.577 |
| walker |  | 3421 | 32 | python decl doc at src/pluggy/_hooks.py:593 |  |  | 0.581 |
| walker |  | 3501 | 80 | python decl doc at src/pluggy/_hooks.py:164 |  |  | 0.581 |
| walker |  | 3582 | 81 | python decl doc at src/pluggy/_hooks.py:77 |  |  | 0.581 |
| walker |  | 3652 | 70 | python method at src/pluggy/_hooks.py:393 |  |  | 0.581 |
| walker |  | 3664 | 12 | python method doc at src/pluggy/_hooks.py:393 |  |  | 0.581 |
| walker |  | 3743 | 79 | python decl doc at src/pluggy/_hooks.py:293 |  |  | 0.581 |
| ns | 3795 |  | 413 | HookCaller: hookimpl call-order algorithm (_add_hookimpl + its ordering comment) | 3.2 | 2.4 | 0.552 |
| walker |  | 3986 | 243 | python method sigs in src/pluggy/_manager.py |  |  | 0.569 |
| walker |  | 3986 | 0 | python method at src/pluggy/_manager.py:59 |  |  | 0.569 |
| walker |  | 3986 | 0 | python method at src/pluggy/_manager.py:68 |  |  | 0.569 |
| walker |  | 3986 | 0 | python method at src/pluggy/_manager.py:76 |  |  | 0.569 |
| walker |  | 3986 | 0 | python method at src/pluggy/_manager.py:79 |  |  | 0.569 |
| walker |  | 3986 | 0 | python method at src/pluggy/_manager.py:125 |  |  | 0.569 |
| walker |  | 3986 | 0 | python method at src/pluggy/_manager.py:176 |  |  | 0.569 |
| walker |  | 3986 | 0 | python method at src/pluggy/_manager.py:233 |  |  | 0.569 |
| walker |  | 3986 | 0 | python method at src/pluggy/_manager.py:238 |  |  | 0.569 |
| walker |  | 3996 | 10 | python method at src/pluggy/_manager.py:100 |  |  | 0.569 |
| walker |  | 4009 | 13 | python method at src/pluggy/_manager.py:71 |  |  | 0.572 |
| walker |  | 4018 | 9 | python method body at src/pluggy/_manager.py:68 body 69 |  |  | 0.572 |
| walker |  | 4033 | 15 | python method doc at src/pluggy/_manager.py:238 |  |  | 0.572 |
| walker |  | 4068 | 35 | python method at src/pluggy/_manager.py:201 |  |  | 0.572 |
| walker |  | 4086 | 18 | python method doc at src/pluggy/_manager.py:233 |  |  | 0.572 |
| walker |  | 4099 | 13 | python method body at src/pluggy/_manager.py:76 body 77 |  |  | 0.572 |
| walker |  | 4118 | 19 | python method body at src/pluggy/_manager.py:79 body 80 |  |  | 0.572 |
| walker |  | 4138 | 20 | python method body at src/pluggy/_manager.py:71 body 73 |  |  | 0.572 |
| walker |  | 4162 | 24 | python method body at src/pluggy/_manager.py:238 body 240 |  |  | 0.572 |
| walker |  | 4246 | 84 | python method at src/pluggy/_hooks.py:91 |  |  | 0.572 |
| walker |  | 4255 | 9 | python method body at src/pluggy/_hooks.py:91 body 99 |  |  | 0.572 |
| walker |  | 4317 | 62 | listing of 'testing' |  |  | 0.617 |
| walker |  | 4348 | 31 | python method body at src/pluggy/_manager.py:59 body 60 |  |  | 0.617 |
| walker |  | 4409 | 61 | python method doc at src/pluggy/_result.py:80 |  |  | 0.617 |
| walker |  | 4501 | 92 | python method at src/pluggy/_hooks.py:101 |  |  | 0.617 |
| walker |  | 4515 | 14 | python method body at src/pluggy/_hooks.py:101 body 109 |  |  | 0.617 |
| walker |  | 4578 | 63 | python method doc at src/pluggy/_result.py:91 |  |  | 0.617 |
| walker |  | 4675 | 97 | python method at src/pluggy/_hooks.py:111 |  |  | 0.619 |
| walker |  | 4774 | 99 | python method at src/pluggy/_hooks.py:178 |  |  | 0.619 |
| walker |  | 4783 | 9 | python method body at src/pluggy/_hooks.py:178 body 188 |  |  | 0.619 |
| ns | 4819 |  | 1024 | HookimplMarker.__call__: full option docs (wrapper/hookwrapper/optionalhook/tryfirst/trylast/specname) | 3.3 | 2.3 | 0.556 |
| walker |  | 4850 | 67 | python decl names surface in src/pluggy/_tracing.py |  |  | 0.560 |
| walker |  | 4850 | 0 | python decl at src/pluggy/_tracing.py:16 |  |  | 0.560 |
| walker |  | 4850 | 0 | python decl at src/pluggy/_tracing.py:59 |  |  | 0.560 |
| walker |  | 4957 | 107 | python method at src/pluggy/_hooks.py:190 |  |  | 0.560 |
| walker |  | 4971 | 14 | python method body at src/pluggy/_hooks.py:190 body 200 |  |  | 0.560 |
| walker |  | 5083 | 112 | python method at src/pluggy/_hooks.py:202 |  |  | 0.561 |
| walker |  | 5293 | 210 | python method sigs in src/pluggy/_tracing.py |  |  | 0.583 |
| walker |  | 5293 | 0 | python method at src/pluggy/_tracing.py:17 |  |  | 0.583 |
| walker |  | 5293 | 0 | python method at src/pluggy/_tracing.py:22 |  |  | 0.583 |
| walker |  | 5293 | 0 | python method at src/pluggy/_tracing.py:25 |  |  | 0.583 |
| walker |  | 5293 | 0 | python method at src/pluggy/_tracing.py:42 |  |  | 0.583 |
| walker |  | 5293 | 0 | python method at src/pluggy/_tracing.py:48 |  |  | 0.583 |
| walker |  | 5293 | 0 | python method at src/pluggy/_tracing.py:51 |  |  | 0.583 |
| walker |  | 5293 | 0 | python method at src/pluggy/_tracing.py:60 |  |  | 0.583 |
| walker |  | 5293 | 0 | python method at src/pluggy/_tracing.py:64 |  |  | 0.583 |
| walker |  | 5293 | 0 | python method at src/pluggy/_tracing.py:67 |  |  | 0.583 |
| walker |  | 5302 | 9 | python method body at src/pluggy/_tracing.py:48 body 49 |  |  | 0.583 |
| walker |  | 5314 | 12 | python method body at src/pluggy/_tracing.py:22 body 23 |  |  | 0.583 |
| walker |  | 5327 | 13 | python method body at src/pluggy/_tracing.py:64 body 65 |  |  | 0.583 |
| walker |  | 5344 | 17 | python method body at src/pluggy/_tracing.py:67 body 68 |  |  | 0.583 |
| walker |  | 5362 | 18 | python method body at src/pluggy/_tracing.py:60 body 61 |  |  | 0.583 |
| walker |  | 5366 | 4 | listing of '.claude' |  |  | 0.583 |
| walker |  | 5381 | 15 | listing of 'docs/examples' |  |  | 0.599 |
| walker |  | 5458 | 77 | python decl body at src/pluggy/_hooks.py:281 body 282 |  |  | 0.599 |
| walker |  | 5545 | 87 | python method doc at src/pluggy/_manager.py:201 |  |  | 0.599 |
| walker |  | 5625 | 80 | python decl names surface in src/pluggy/_callers.py |  |  | 0.606 |
| walker |  | 5656 | 31 | python decl at src/pluggy/_callers.py:27 |  |  | 0.606 |
| walker |  | 5687 | 31 | python decl doc at src/pluggy/_callers.py:27 |  |  | 0.606 |
| walker |  | 5716 | 29 | python decl at src/pluggy/_callers.py:70 |  |  | 0.606 |
| walker |  | 5750 | 34 | python decl at src/pluggy/_callers.py:60 |  |  | 0.606 |
| ns | 5840 |  | 1021 | _multicall(): full body -- the actual call loop | 3.4 | 2.1 | 0.544 |
| walker |  | 5939 | 189 | python decl doc at src/pluggy/_manager.py:83 |  |  | 0.544 |
| ns | 6113 |  | 273 | PluggyWarning / PluggyTeardownRaisedWarning: full body | 3.5 | 2.2 | 0.531 |
| walker |  | 6128 | 189 | python decl doc at src/pluggy/_warnings.py:10 |  |  | 0.556 |
| walker |  | 6364 | 236 | python method sigs #1 in src/pluggy/_hooks.py |  |  | 0.575 |
| walker |  | 6364 | 0 | python method at src/pluggy/_hooks.py:438 |  |  | 0.575 |
| walker |  | 6364 | 0 | python method at src/pluggy/_hooks.py:442 |  |  | 0.575 |
| walker |  | 6364 | 0 | python method at src/pluggy/_hooks.py:449 |  |  | 0.575 |
| walker |  | 6364 | 0 | python method at src/pluggy/_hooks.py:453 |  |  | 0.575 |
| walker |  | 6364 | 0 | python method at src/pluggy/_hooks.py:477 |  |  | 0.575 |
| walker |  | 6364 | 0 | python method at src/pluggy/_hooks.py:499 |  |  | 0.575 |
| walker |  | 6364 | 0 | python method at src/pluggy/_hooks.py:577 |  |  | 0.575 |
| walker |  | 6364 | 0 | python method at src/pluggy/_hooks.py:612 |  |  | 0.575 |
| walker |  | 6379 | 15 | python method doc at src/pluggy/_hooks.py:449 |  |  | 0.575 |
| walker |  | 6410 | 31 | python method at src/pluggy/_hooks.py:543 |  |  | 0.575 |
| walker |  | 6421 | 11 | python method body at src/pluggy/_hooks.py:449 body 451 |  |  | 0.575 |
| walker |  | 6439 | 18 | python method doc at src/pluggy/_hooks.py:438 |  |  | 0.575 |
| walker |  | 6451 | 12 | python method body at src/pluggy/_hooks.py:438 body 440 |  |  | 0.575 |
| walker |  | 6468 | 17 | python method at src/pluggy/_hooks.py:480 |  |  | 0.575 |
| walker |  | 6483 | 15 | python method body at src/pluggy/_hooks.py:477 body 478 |  |  | 0.575 |
| walker |  | 6501 | 18 | python method at src/pluggy/_hooks.py:618 |  |  | 0.576 |
| ns | 6545 |  | 432 | Test roster: test_pluginmanager.py (32 tests) | 4.1 |  | 0.558 |
| walker |  | 6553 | 52 | python method at src/pluggy/_hooks.py:516 |  |  | 0.558 |
| walker |  | 6567 | 14 | python method doc at src/pluggy/_hooks.py:453 |  |  | 0.558 |
| walker |  | 6612 | 45 | python method doc at src/pluggy/_hooks.py:543 |  |  | 0.558 |
| walker |  | 6633 | 21 | python method doc at src/pluggy/_hooks.py:577 |  |  | 0.558 |
| ns | 6694 |  | 149 | Test roster: test_details.py (10 tests) | 4.2 |  | 0.553 |
| walker |  | 6710 | 77 | python method doc at src/pluggy/_hooks.py:499 |  |  | 0.553 |
| walker |  | 6808 | 98 | python method doc at src/pluggy/_hooks.py:516 |  |  | 0.553 |
| ns | 6913 |  | 219 | Test roster: test_hookcaller.py (15 tests) | 4.3 |  | 0.546 |
| walker |  | 7053 | 245 | python method sigs #1 in src/pluggy/_manager.py |  |  | 0.566 |
| walker |  | 7053 | 0 | python method at src/pluggy/_manager.py:242 |  |  | 0.566 |
| walker |  | 7053 | 0 | python method at src/pluggy/_manager.py:252 |  |  | 0.566 |
| walker |  | 7053 | 0 | python method at src/pluggy/_manager.py:296 |  |  | 0.566 |
| walker |  | 7053 | 0 | python method at src/pluggy/_manager.py:300 |  |  | 0.566 |
| walker |  | 7053 | 0 | python method at src/pluggy/_manager.py:304 |  |  | 0.566 |
| walker |  | 7053 | 0 | python method at src/pluggy/_manager.py:315 |  |  | 0.566 |
| walker |  | 7053 | 0 | python method at src/pluggy/_manager.py:319 |  |  | 0.566 |
| walker |  | 7053 | 0 | python method at src/pluggy/_manager.py:323 |  |  | 0.566 |
| walker |  | 7053 | 0 | python method at src/pluggy/_manager.py:331 |  |  | 0.566 |
| walker |  | 7053 | 0 | python method at src/pluggy/_manager.py:379 |  |  | 0.566 |
| walker |  | 7053 | 0 | python method at src/pluggy/_manager.py:395 |  |  | 0.566 |
| walker |  | 7067 | 14 | python method doc at src/pluggy/_manager.py:300 |  |  | 0.566 |
| walker |  | 7082 | 15 | python method doc at src/pluggy/_manager.py:296 |  |  | 0.566 |
| ns | 7085 |  | 172 | Test roster: test_invocations.py (13 tests) | 4.4 |  | 0.560 |
| walker |  | 7113 | 31 | python method at src/pluggy/_manager.py:278 |  |  | 0.560 |
| walker |  | 7130 | 17 | python method doc at src/pluggy/_manager.py:319 |  |  | 0.560 |
| walker |  | 7148 | 18 | python method doc at src/pluggy/_manager.py:315 |  |  | 0.560 |
| walker |  | 7160 | 12 | python method body at src/pluggy/_manager.py:315 body 317 |  |  | 0.560 |
| walker |  | 7173 | 13 | python method body at src/pluggy/_manager.py:319 body 321 |  |  | 0.560 |
| walker |  | 7202 | 29 | python method doc at src/pluggy/_manager.py:323 |  |  | 0.560 |
| walker |  | 7220 | 18 | python method body at src/pluggy/_manager.py:300 body 302 |  |  | 0.560 |
| walker |  | 7255 | 35 | python method doc at src/pluggy/_manager.py:242 |  |  | 0.560 |
| walker |  | 7277 | 22 | python method body at src/pluggy/_manager.py:296 body 298 |  |  | 0.560 |
| walker |  | 7321 | 44 | python method doc at src/pluggy/_manager.py:379 |  |  | 0.560 |
| ns | 7372 |  | 287 | Test roster: test_multicall.py (21 tests) | 4.5 |  | 0.549 |
| walker |  | 7384 | 63 | python method doc at src/pluggy/_manager.py:252 |  |  | 0.549 |
| walker |  | 7392 | 8 | python method body at src/pluggy/_manager.py:242 body 250 |  |  | 0.549 |
| walker |  | 7400 | 8 | python method body at src/pluggy/_manager.py:323 body 329 |  |  | 0.549 |
| walker |  | 7497 | 97 | python method doc at src/pluggy/_manager.py:304 |  |  | 0.549 |
| walker |  | 7596 | 99 | python method doc at src/pluggy/_result.py:67 |  |  | 0.549 |
| walker |  | 7606 | 10 | python method body at src/pluggy/_manager.py:233 body 235 |  |  | 0.549 |
| ns | 7620 |  | 248 | testing/conftest.py: full body (shared pm / he_pm fixtures) | 4.6 |  | 0.538 |
| walker |  | 7709 | 103 | python method doc at src/pluggy/_manager.py:395 |  |  | 0.538 |
| ns | 7848 |  | 228 | docs/api_reference.rst: autodoc roster (which classes are documented, in order) | 5.1 |  | 0.531 |
| walker |  | 7925 | 216 | python class body at src/pluggy/_hooks.py:593 |  |  | 0.531 |
| walker |  | 8078 | 153 | python decl names surface #1 in src/pluggy/_hooks.py |  |  | 0.531 |
| walker |  | 8102 | 24 | python decl at src/pluggy/_hooks.py:377 |  |  | 0.531 |
| walker |  | 8136 | 34 | python decl at src/pluggy/_hooks.py:33 |  |  | 0.531 |
| ns | 8153 |  | 305 | docs/index.rst: 'How does it work?' overview | 5.2 |  | 0.523 |
| walker |  | 8195 | 59 | python decl at src/pluggy/_callers.py:82 |  |  | 0.523 |
| walker |  | 8206 | 11 | python method body at src/pluggy/_result.py:42 body 45 |  |  | 0.523 |
| walker |  | 8296 | 90 | python method sigs #2 in src/pluggy/_hooks.py |  |  | 0.529 |
| walker |  | 8296 | 0 | python method at src/pluggy/_hooks.py:634 |  |  | 0.529 |
| walker |  | 8296 | 0 | python method at src/pluggy/_hooks.py:692 |  |  | 0.529 |
| walker |  | 8296 | 0 | python method at src/pluggy/_hooks.py:709 |  |  | 0.529 |
| ns | 8302 |  | 149 | pyproject.toml: every distinct [section] header (config-key roster) | 6.1 |  | 0.524 |
| walker |  | 8319 | 23 | python method at src/pluggy/_hooks.py:626 |  |  | 0.526 |
| walker |  | 8328 | 9 | python method body at src/pluggy/_hooks.py:626 body 628 |  |  | 0.526 |
| walker |  | 8345 | 17 | python method body at src/pluggy/_hooks.py:634 body 635 |  |  | 0.526 |
| walker |  | 8408 | 63 | python method at src/pluggy/_hooks.py:656 |  |  | 0.526 |
| walker |  | 8420 | 12 | python method doc at src/pluggy/_hooks.py:656 |  |  | 0.526 |
| walker |  | 8446 | 26 | python method at src/pluggy/_hooks.py:630 |  |  | 0.528 |
| walker |  | 8457 | 11 | python method body at src/pluggy/_hooks.py:630 body 632 |  |  | 0.528 |
| walker |  | 8482 | 25 | python method body at src/pluggy/_hooks.py:692 body 693 |  |  | 0.528 |
| walker |  | 8494 | 12 | python method body at src/pluggy/_manager.py:304 body 313 |  |  | 0.528 |
| ns | 8496 |  | 194 | tox.ini: [tox] envlist + [testenv] | 6.2 |  | 0.522 |
| walker |  | 8603 | 109 | python imports in src/pluggy/_result.py |  |  | 0.522 |
| ns | 8674 |  | 178 | tox.ini: [testenv:docs] + [pytest] | 6.3 | 6.2 | 0.517 |
| walker |  | 8738 | 135 | python method doc at src/pluggy/_manager.py:176 |  |  | 0.517 |
| ns | 8786 |  | 112 | tox.ini: [testenv:release] | 6.4 | 6.2 | 0.512 |
| walker |  | 8794 | 56 | python decl doc at src/pluggy/_callers.py:82 |  |  | 0.513 |
| walker |  | 8931 | 137 | python method doc at src/pluggy/_manager.py:125 |  |  | 0.513 |
| walker |  | 8944 | 13 | python method body at src/pluggy/_manager.py:233 body 236 |  |  | 0.513 |
| ns | 8988 |  | 202 | .pre-commit-config.yaml: repo + hook-id roster | 6.5 |  | 0.508 |
| ns | 9039 |  | 51 | .coveragerc: [run] section (what coverage.py tracks) | 6.6 |  | 0.506 |
| walker |  | 9090 | 146 | python method doc at src/pluggy/_manager.py:278 |  |  | 0.506 |
| ns | 9102 |  | 63 | MANIFEST.in: full (sdist inclusion rules) | 6.7 |  | 0.503 |
| walker |  | 9152 | 62 | python method body at src/pluggy/_hooks.py:612 body 613 |  |  | 0.503 |
| ns | 9212 |  | 110 | CHANGELOG.rst: pluggy 1.6.0 -- version header + deprecations | 7.1 |  | 0.500 |
| walker |  | 9219 | 67 | python imports in src/pluggy/_tracing.py |  |  | 0.500 |
| walker |  | 9227 | 8 | listing of 'docs/examples/eggsample' |  |  | 0.509 |
| walker |  | 9247 | 20 | listing of 'docs/examples/eggsample/eggsample' |  |  | 0.532 |
| walker |  | 9257 | 10 | python imports in docs/examples/eggsample/eggsample/__init__.py |  |  | 0.532 |
| walker |  | 9377 | 120 | python method sigs #2 in src/pluggy/_manager.py |  |  | 0.541 |
| walker |  | 9377 | 0 | python method at src/pluggy/_manager.py:425 |  |  | 0.541 |
| walker |  | 9377 | 0 | python method at src/pluggy/_manager.py:430 |  |  | 0.541 |
| walker |  | 9377 | 0 | python method at src/pluggy/_manager.py:434 |  |  | 0.541 |
| walker |  | 9377 | 0 | python method at src/pluggy/_manager.py:487 |  |  | 0.541 |
| walker |  | 9405 | 28 | python method at src/pluggy/_manager.py:512 |  |  | 0.541 |
| walker |  | 9436 | 31 | python method at src/pluggy/_manager.py:450 |  |  | 0.541 |
| ns | 9443 |  | 231 | CHANGELOG.rst: pluggy 1.6.0 -- bug fixes | 7.2 | 7.1 | 0.536 |
| walker |  | 9449 | 13 | python method body at src/pluggy/_manager.py:425 body 428 |  |  | 0.536 |
| walker |  | 9470 | 21 | python method doc at src/pluggy/_manager.py:430 |  |  | 0.536 |
| walker |  | 9482 | 12 | python method body at src/pluggy/_manager.py:430 body 432 |  |  | 0.536 |
| walker |  | 9510 | 28 | python method doc at src/pluggy/_manager.py:425 |  |  | 0.536 |
| walker |  | 9552 | 42 | python method doc at src/pluggy/_manager.py:487 |  |  | 0.536 |
| walker |  | 9602 | 50 | python method doc at src/pluggy/_manager.py:512 |  |  | 0.536 |
| ns | 9646 |  | 203 | changelog/README.rst: newsfragment type taxonomy + naming convention | 7.3 |  | 0.532 |
| walker |  | 9667 | 65 | python method doc at src/pluggy/_manager.py:434 |  |  | 0.532 |
| walker |  | 9846 | 179 | python method doc at src/pluggy/_manager.py:450 |  |  | 0.532 |
| walker |  | 9895 | 49 | python method body at src/pluggy/_tracing.py:17 body 18 |  |  | 0.532 |
| walker |  | 9912 | 17 | python method body at src/pluggy/_manager.py:304 body 312 |  |  | 0.532 |
| walker |  | 9930 | 18 | python decl names surface in docs/examples/eggsample/eggsample/__init__.py |  |  | 0.532 |
| ns | 9971 |  | 325 | RELEASING.rst: full release procedure | 7.4 |  | 0.522 |
