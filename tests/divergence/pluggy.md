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
| walker |  | 1712 | 127 | python method sigs in src/pluggy/_result.py |  |  | 0.707 |
| walker |  | 1712 | 0 | python method at src/pluggy/_result.py:67 |  |  | 0.707 |
| walker |  | 1712 | 0 | python method at src/pluggy/_result.py:80 |  |  | 0.707 |
| walker |  | 1712 | 0 | python method at src/pluggy/_result.py:91 |  |  | 0.707 |
| walker |  | 1720 | 8 | python method at src/pluggy/_result.py:42 |  |  | 0.707 |
| walker |  | 1728 | 8 | python method at src/pluggy/_result.py:51 |  |  | 0.707 |
| walker |  | 1736 | 8 | python method at src/pluggy/_result.py:56 |  |  | 0.708 |
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
| walker |  | 3207 | 32 | python decl doc at src/pluggy/_hooks.py:593 |  |  | 0.619 |
| walker |  | 3287 | 80 | python decl doc at src/pluggy/_hooks.py:164 |  |  | 0.619 |
| walker |  | 3368 | 81 | python decl doc at src/pluggy/_hooks.py:77 |  |  | 0.619 |
| ns | 3382 |  | 648 | HookspecMarker.__call__: full option docs (firstresult/historic/warn_on_impl) | 3.1 | 2.3 | 0.563 |
| ns | 3795 |  | 413 | HookCaller: hookimpl call-order algorithm (_add_hookimpl + its ordering comment) | 3.2 | 2.4 | 0.536 |
| walker |  | 3908 | 540 | python method sigs roster in src/pluggy/_hooks.py |  |  | 0.597 |
| walker |  | 3908 | 0 | python method sigs in src/pluggy/_hooks.py |  |  | 0.597 |
| walker |  | 3908 | 0 | python method sigs #1 in src/pluggy/_hooks.py |  |  | 0.597 |
| walker |  | 3908 | 0 | python method sigs #2 in src/pluggy/_hooks.py |  |  | 0.597 |
| walker |  | 3908 | 0 | python method at src/pluggy/_hooks.py:88 |  |  | 0.597 |
| walker |  | 3908 | 0 | python method at src/pluggy/_hooks.py:175 |  |  | 0.597 |
| walker |  | 3908 | 0 | python method at src/pluggy/_hooks.py:365 |  |  | 0.597 |
| walker |  | 3908 | 0 | python method at src/pluggy/_hooks.py:420 |  |  | 0.597 |
| walker |  | 3908 | 0 | python method at src/pluggy/_hooks.py:438 |  |  | 0.597 |
| walker |  | 3908 | 0 | python method at src/pluggy/_hooks.py:442 |  |  | 0.597 |
| walker |  | 3908 | 0 | python method at src/pluggy/_hooks.py:449 |  |  | 0.597 |
| walker |  | 3908 | 0 | python method at src/pluggy/_hooks.py:453 |  |  | 0.597 |
| walker |  | 3908 | 0 | python method at src/pluggy/_hooks.py:477 |  |  | 0.597 |
| walker |  | 3908 | 0 | python method at src/pluggy/_hooks.py:499 |  |  | 0.597 |
| walker |  | 3908 | 0 | python method at src/pluggy/_hooks.py:577 |  |  | 0.597 |
| walker |  | 3908 | 0 | python method at src/pluggy/_hooks.py:612 |  |  | 0.597 |
| walker |  | 3908 | 0 | python method at src/pluggy/_hooks.py:634 |  |  | 0.597 |
| walker |  | 3908 | 0 | python method at src/pluggy/_hooks.py:692 |  |  | 0.597 |
| walker |  | 3908 | 0 | python method at src/pluggy/_hooks.py:709 |  |  | 0.597 |
| walker |  | 3916 | 8 | python method at src/pluggy/_hooks.py:626 |  |  | 0.597 |
| walker |  | 3924 | 8 | python method at src/pluggy/_hooks.py:630 |  |  | 0.597 |
| walker |  | 3934 | 10 | python method doc at src/pluggy/_hooks.py:365 |  |  | 0.597 |
| walker |  | 3943 | 9 | python method body at src/pluggy/_hooks.py:626 body 628 |  |  | 0.597 |
| walker |  | 3974 | 31 | python method at src/pluggy/_hooks.py:543 |  |  | 0.597 |
| walker |  | 3984 | 10 | python method body at src/pluggy/_hooks.py:420 body 421 |  |  | 0.597 |
| walker |  | 3999 | 15 | python method doc at src/pluggy/_hooks.py:449 |  |  | 0.597 |
| walker |  | 4010 | 11 | python method body at src/pluggy/_hooks.py:449 body 451 |  |  | 0.597 |
| walker |  | 4025 | 15 | python method at src/pluggy/_hooks.py:618 |  |  | 0.597 |
| walker |  | 4043 | 18 | python method doc at src/pluggy/_hooks.py:438 |  |  | 0.597 |
| walker |  | 4055 | 12 | python method body at src/pluggy/_hooks.py:438 body 440 |  |  | 0.597 |
| walker |  | 4096 | 41 | python method at src/pluggy/_hooks.py:424 |  |  | 0.597 |
| walker |  | 4113 | 17 | python method at src/pluggy/_hooks.py:480 |  |  | 0.597 |
| walker |  | 4127 | 14 | python method body at src/pluggy/_hooks.py:88 body 89 |  |  | 0.597 |
| walker |  | 4141 | 14 | python method body at src/pluggy/_hooks.py:175 body 176 |  |  | 0.597 |
| walker |  | 4156 | 15 | python method body at src/pluggy/_hooks.py:477 body 478 |  |  | 0.597 |
| walker |  | 4208 | 52 | python method at src/pluggy/_hooks.py:516 |  |  | 0.597 |
| walker |  | 4225 | 17 | python method body at src/pluggy/_hooks.py:634 body 635 |  |  | 0.597 |
| walker |  | 4239 | 14 | python method doc at src/pluggy/_hooks.py:453 |  |  | 0.598 |
| walker |  | 4302 | 63 | python method at src/pluggy/_hooks.py:656 |  |  | 0.598 |
| walker |  | 4314 | 12 | python method doc at src/pluggy/_hooks.py:656 |  |  | 0.598 |
| walker |  | 4384 | 70 | python method at src/pluggy/_hooks.py:393 |  |  | 0.598 |
| walker |  | 4396 | 12 | python method doc at src/pluggy/_hooks.py:393 |  |  | 0.598 |
| walker |  | 4407 | 11 | python method body at src/pluggy/_hooks.py:630 body 632 |  |  | 0.598 |
| walker |  | 4428 | 21 | python method doc at src/pluggy/_hooks.py:577 |  |  | 0.598 |
| walker |  | 4473 | 45 | python method doc at src/pluggy/_hooks.py:543 |  |  | 0.598 |
| walker |  | 4552 | 79 | python decl doc at src/pluggy/_hooks.py:293 |  |  | 0.598 |
| walker |  | 4577 | 25 | python method body at src/pluggy/_hooks.py:692 body 693 |  |  | 0.598 |
| walker |  | 4659 | 82 | python method at src/pluggy/_hooks.py:91 |  |  | 0.598 |
| walker |  | 4670 | 11 | python method body at src/pluggy/_hooks.py:91 body 99 |  |  | 0.598 |
| ns | 4819 |  | 1024 | HookimplMarker.__call__: full option docs (wrapper/hookwrapper/optionalhook/tryfirst/trylast/specname) | 3.3 | 2.3 | 0.536 |
| walker |  | 5285 | 615 | python method sigs roster in src/pluggy/_manager.py |  |  | 0.590 |
| walker |  | 5285 | 0 | python method sigs in src/pluggy/_manager.py |  |  | 0.590 |
| walker |  | 5285 | 0 | python method sigs #1 in src/pluggy/_manager.py |  |  | 0.590 |
| walker |  | 5285 | 0 | python method sigs #2 in src/pluggy/_manager.py |  |  | 0.590 |
| walker |  | 5285 | 0 | python method at src/pluggy/_manager.py:59 |  |  | 0.590 |
| walker |  | 5285 | 0 | python method at src/pluggy/_manager.py:68 |  |  | 0.590 |
| walker |  | 5285 | 0 | python method at src/pluggy/_manager.py:76 |  |  | 0.590 |
| walker |  | 5285 | 0 | python method at src/pluggy/_manager.py:79 |  |  | 0.590 |
| walker |  | 5285 | 0 | python method at src/pluggy/_manager.py:125 |  |  | 0.590 |
| walker |  | 5285 | 0 | python method at src/pluggy/_manager.py:176 |  |  | 0.590 |
| walker |  | 5285 | 0 | python method at src/pluggy/_manager.py:233 |  |  | 0.590 |
| walker |  | 5285 | 0 | python method at src/pluggy/_manager.py:238 |  |  | 0.590 |
| walker |  | 5285 | 0 | python method at src/pluggy/_manager.py:242 |  |  | 0.590 |
| walker |  | 5285 | 0 | python method at src/pluggy/_manager.py:252 |  |  | 0.590 |
| walker |  | 5285 | 0 | python method at src/pluggy/_manager.py:296 |  |  | 0.590 |
| walker |  | 5285 | 0 | python method at src/pluggy/_manager.py:300 |  |  | 0.590 |
| walker |  | 5285 | 0 | python method at src/pluggy/_manager.py:304 |  |  | 0.590 |
| walker |  | 5285 | 0 | python method at src/pluggy/_manager.py:315 |  |  | 0.590 |
| walker |  | 5285 | 0 | python method at src/pluggy/_manager.py:319 |  |  | 0.590 |
| walker |  | 5285 | 0 | python method at src/pluggy/_manager.py:323 |  |  | 0.590 |
| walker |  | 5285 | 0 | python method at src/pluggy/_manager.py:331 |  |  | 0.590 |
| walker |  | 5285 | 0 | python method at src/pluggy/_manager.py:379 |  |  | 0.590 |
| walker |  | 5285 | 0 | python method at src/pluggy/_manager.py:395 |  |  | 0.590 |
| walker |  | 5285 | 0 | python method at src/pluggy/_manager.py:425 |  |  | 0.590 |
| walker |  | 5285 | 0 | python method at src/pluggy/_manager.py:430 |  |  | 0.590 |
| walker |  | 5285 | 0 | python method at src/pluggy/_manager.py:434 |  |  | 0.590 |
| walker |  | 5285 | 0 | python method at src/pluggy/_manager.py:487 |  |  | 0.590 |
| walker |  | 5293 | 8 | python method at src/pluggy/_manager.py:71 |  |  | 0.592 |
| walker |  | 5303 | 10 | python method at src/pluggy/_manager.py:100 |  |  | 0.592 |
| walker |  | 5331 | 28 | python method at src/pluggy/_manager.py:512 |  |  | 0.592 |
| walker |  | 5340 | 9 | python method body at src/pluggy/_manager.py:68 body 69 |  |  | 0.592 |
| walker |  | 5354 | 14 | python method doc at src/pluggy/_manager.py:300 |  |  | 0.592 |
| walker |  | 5385 | 31 | python method at src/pluggy/_manager.py:278 |  |  | 0.592 |
| walker |  | 5416 | 31 | python method at src/pluggy/_manager.py:450 |  |  | 0.592 |
| walker |  | 5431 | 15 | python method doc at src/pluggy/_manager.py:238 |  |  | 0.592 |
| walker |  | 5446 | 15 | python method doc at src/pluggy/_manager.py:296 |  |  | 0.592 |
| walker |  | 5481 | 35 | python method at src/pluggy/_manager.py:201 |  |  | 0.592 |
| walker |  | 5498 | 17 | python method doc at src/pluggy/_manager.py:319 |  |  | 0.592 |
| walker |  | 5516 | 18 | python method doc at src/pluggy/_manager.py:233 |  |  | 0.592 |
| walker |  | 5534 | 18 | python method doc at src/pluggy/_manager.py:315 |  |  | 0.592 |
| walker |  | 5546 | 12 | python method body at src/pluggy/_manager.py:315 body 317 |  |  | 0.592 |
| walker |  | 5559 | 13 | python method body at src/pluggy/_manager.py:76 body 77 |  |  | 0.592 |
| walker |  | 5572 | 13 | python method body at src/pluggy/_manager.py:319 body 321 |  |  | 0.592 |
| walker |  | 5585 | 13 | python method body at src/pluggy/_manager.py:425 body 428 |  |  | 0.592 |
| walker |  | 5606 | 21 | python method doc at src/pluggy/_manager.py:430 |  |  | 0.592 |
| walker |  | 5618 | 12 | python method body at src/pluggy/_manager.py:430 body 432 |  |  | 0.592 |
| walker |  | 5646 | 28 | python method doc at src/pluggy/_manager.py:425 |  |  | 0.592 |
| walker |  | 5675 | 29 | python method doc at src/pluggy/_manager.py:323 |  |  | 0.592 |
| walker |  | 5693 | 18 | python method body at src/pluggy/_manager.py:300 body 302 |  |  | 0.592 |
| walker |  | 5712 | 19 | python method body at src/pluggy/_manager.py:79 body 80 |  |  | 0.592 |
| walker |  | 5747 | 35 | python method doc at src/pluggy/_manager.py:242 |  |  | 0.592 |
| walker |  | 5767 | 20 | python method body at src/pluggy/_manager.py:71 body 73 |  |  | 0.592 |
| walker |  | 5789 | 22 | python method body at src/pluggy/_manager.py:238 body 240 |  |  | 0.592 |
| walker |  | 5811 | 22 | python method body at src/pluggy/_manager.py:296 body 298 |  |  | 0.592 |
| ns | 5840 |  | 1021 | _multicall(): full body -- the actual call loop | 3.4 | 2.1 | 0.532 |
| walker |  | 5853 | 42 | python method doc at src/pluggy/_manager.py:487 |  |  | 0.532 |
| walker |  | 5897 | 44 | python method doc at src/pluggy/_manager.py:379 |  |  | 0.532 |
| walker |  | 5959 | 62 | listing of 'testing' |  |  | 0.567 |
| walker |  | 6009 | 50 | python method doc at src/pluggy/_manager.py:512 |  |  | 0.567 |
| walker |  | 6098 | 89 | python method at src/pluggy/_hooks.py:101 |  |  | 0.567 |
| walker |  | 6112 | 14 | python method body at src/pluggy/_hooks.py:101 body 109 |  |  | 0.567 |
| ns | 6113 |  | 273 | PluggyWarning / PluggyTeardownRaisedWarning: full body | 3.5 | 2.2 | 0.553 |
| walker |  | 6173 | 61 | python method doc at src/pluggy/_result.py:80 |  |  | 0.553 |
| walker |  | 6270 | 97 | python method at src/pluggy/_hooks.py:111 |  |  | 0.555 |
| walker |  | 6367 | 97 | python method at src/pluggy/_hooks.py:178 |  |  | 0.555 |
| walker |  | 6378 | 11 | python method body at src/pluggy/_hooks.py:178 body 188 |  |  | 0.555 |
| walker |  | 6441 | 63 | python method doc at src/pluggy/_manager.py:252 |  |  | 0.555 |
| walker |  | 6504 | 63 | python method doc at src/pluggy/_result.py:91 |  |  | 0.555 |
| walker |  | 6535 | 31 | python method body at src/pluggy/_manager.py:59 body 60 |  |  | 0.555 |
| ns | 6545 |  | 432 | Test roster: test_pluginmanager.py (32 tests) | 4.1 |  | 0.538 |
| walker |  | 6600 | 65 | python method doc at src/pluggy/_manager.py:434 |  |  | 0.538 |
| ns | 6694 |  | 149 | Test roster: test_details.py (10 tests) | 4.2 |  | 0.533 |
| walker |  | 6704 | 104 | python method at src/pluggy/_hooks.py:190 |  |  | 0.533 |
| walker |  | 6718 | 14 | python method body at src/pluggy/_hooks.py:190 body 200 |  |  | 0.533 |
| walker |  | 6785 | 67 | python decl names surface in src/pluggy/_tracing.py |  |  | 0.536 |
| walker |  | 6785 | 0 | python decl at src/pluggy/_tracing.py:16 |  |  | 0.536 |
| walker |  | 6785 | 0 | python decl at src/pluggy/_tracing.py:59 |  |  | 0.536 |
| walker |  | 6800 | 15 | listing of 'docs/examples' |  |  | 0.550 |
| ns | 6913 |  | 219 | Test roster: test_hookcaller.py (15 tests) | 4.3 |  | 0.542 |
| walker |  | 7010 | 210 | python method sigs in src/pluggy/_tracing.py |  |  | 0.560 |
| walker |  | 7010 | 0 | python method at src/pluggy/_tracing.py:17 |  |  | 0.560 |
| walker |  | 7010 | 0 | python method at src/pluggy/_tracing.py:22 |  |  | 0.560 |
| walker |  | 7010 | 0 | python method at src/pluggy/_tracing.py:25 |  |  | 0.560 |
| walker |  | 7010 | 0 | python method at src/pluggy/_tracing.py:42 |  |  | 0.560 |
| walker |  | 7010 | 0 | python method at src/pluggy/_tracing.py:48 |  |  | 0.560 |
| walker |  | 7010 | 0 | python method at src/pluggy/_tracing.py:51 |  |  | 0.560 |
| walker |  | 7010 | 0 | python method at src/pluggy/_tracing.py:60 |  |  | 0.560 |
| walker |  | 7010 | 0 | python method at src/pluggy/_tracing.py:64 |  |  | 0.560 |
| walker |  | 7010 | 0 | python method at src/pluggy/_tracing.py:67 |  |  | 0.560 |
| walker |  | 7019 | 9 | python method body at src/pluggy/_tracing.py:48 body 49 |  |  | 0.560 |
| walker |  | 7031 | 12 | python method body at src/pluggy/_tracing.py:22 body 23 |  |  | 0.560 |
| walker |  | 7044 | 13 | python method body at src/pluggy/_tracing.py:64 body 65 |  |  | 0.560 |
| walker |  | 7061 | 17 | python method body at src/pluggy/_tracing.py:67 body 68 |  |  | 0.560 |
| walker |  | 7079 | 18 | python method body at src/pluggy/_tracing.py:60 body 61 |  |  | 0.560 |
| walker |  | 7083 | 4 | listing of '.claude' |  |  | 0.560 |
| ns | 7085 |  | 172 | Test roster: test_invocations.py (13 tests) | 4.4 |  | 0.553 |
| walker |  | 7195 | 112 | python method at src/pluggy/_hooks.py:202 |  |  | 0.554 |
| walker |  | 7272 | 77 | python method doc at src/pluggy/_hooks.py:499 |  |  | 0.554 |
| walker |  | 7280 | 8 | python method body at src/pluggy/_manager.py:242 body 250 |  |  | 0.554 |
| walker |  | 7288 | 8 | python method body at src/pluggy/_manager.py:323 body 329 |  |  | 0.554 |
| walker |  | 7368 | 80 | python decl names surface in src/pluggy/_callers.py |  |  | 0.560 |
| ns | 7372 |  | 287 | Test roster: test_multicall.py (21 tests) | 4.5 |  | 0.549 |
| walker |  | 7399 | 31 | python decl at src/pluggy/_callers.py:27 |  |  | 0.549 |
| walker |  | 7430 | 31 | python decl doc at src/pluggy/_callers.py:27 |  |  | 0.549 |
| walker |  | 7459 | 29 | python decl at src/pluggy/_callers.py:70 |  |  | 0.549 |
| walker |  | 7493 | 34 | python decl at src/pluggy/_callers.py:60 |  |  | 0.549 |
| walker |  | 7580 | 87 | python method doc at src/pluggy/_manager.py:201 |  |  | 0.549 |
| ns | 7620 |  | 248 | testing/conftest.py: full body (shared pm / he_pm fixtures) | 4.6 |  | 0.538 |
| walker |  | 7657 | 77 | python decl body at src/pluggy/_hooks.py:281 body 282 |  |  | 0.538 |
| walker |  | 7846 | 189 | python decl doc at src/pluggy/_manager.py:83 |  |  | 0.538 |
| ns | 7848 |  | 228 | docs/api_reference.rst: autodoc roster (which classes are documented, in order) | 5.1 |  | 0.531 |
| walker |  | 8035 | 189 | python decl doc at src/pluggy/_warnings.py:10 |  |  | 0.551 |
| walker |  | 8132 | 97 | python method doc at src/pluggy/_manager.py:304 |  |  | 0.551 |
| ns | 8153 |  | 305 | docs/index.rst: 'How does it work?' overview | 5.2 |  | 0.543 |
| walker |  | 8230 | 98 | python method doc at src/pluggy/_hooks.py:516 |  |  | 0.543 |
| ns | 8302 |  | 149 | pyproject.toml: every distinct [section] header (config-key roster) | 6.1 |  | 0.537 |
| walker |  | 8329 | 99 | python method doc at src/pluggy/_result.py:67 |  |  | 0.537 |
| walker |  | 8432 | 103 | python method doc at src/pluggy/_manager.py:395 |  |  | 0.537 |
| ns | 8496 |  | 194 | tox.ini: [tox] envlist + [testenv] | 6.2 |  | 0.531 |
| walker |  | 8648 | 216 | python class body at src/pluggy/_hooks.py:593 |  |  | 0.531 |
| ns | 8674 |  | 178 | tox.ini: [testenv:docs] + [pytest] | 6.3 | 6.2 | 0.526 |
| walker |  | 8707 | 59 | python decl at src/pluggy/_callers.py:82 |  |  | 0.526 |
| ns | 8786 |  | 112 | tox.ini: [testenv:release] | 6.4 | 6.2 | 0.522 |
| walker |  | 8860 | 153 | python decl names surface #1 in src/pluggy/_hooks.py |  |  | 0.522 |
| walker |  | 8884 | 24 | python decl at src/pluggy/_hooks.py:377 |  |  | 0.522 |
| walker |  | 8918 | 34 | python decl at src/pluggy/_hooks.py:33 |  |  | 0.522 |
| walker |  | 8928 | 10 | python method body at src/pluggy/_manager.py:233 body 235 |  |  | 0.522 |
| walker |  | 8939 | 11 | python method body at src/pluggy/_result.py:42 body 45 |  |  | 0.522 |
| ns | 8988 |  | 202 | .pre-commit-config.yaml: repo + hook-id roster | 6.5 |  | 0.517 |
| walker |  | 8995 | 56 | python decl doc at src/pluggy/_callers.py:82 |  |  | 0.517 |
| ns | 9039 |  | 51 | .coveragerc: [run] section (what coverage.py tracks) | 6.6 |  | 0.515 |
| ns | 9102 |  | 63 | MANIFEST.in: full (sdist inclusion rules) | 6.7 |  | 0.512 |
| walker |  | 9104 | 109 | python imports in src/pluggy/_result.py |  |  | 0.512 |
| walker |  | 9112 | 8 | listing of 'docs/examples/eggsample' |  |  | 0.522 |
| walker |  | 9132 | 20 | listing of 'docs/examples/eggsample/eggsample' |  |  | 0.544 |
| walker |  | 9142 | 10 | python imports in docs/examples/eggsample/eggsample/__init__.py |  |  | 0.544 |
| ns | 9212 |  | 110 | CHANGELOG.rst: pluggy 1.6.0 -- version header + deprecations | 7.1 |  | 0.541 |
| walker |  | 9277 | 135 | python method doc at src/pluggy/_manager.py:176 |  |  | 0.541 |
| walker |  | 9289 | 12 | python method body at src/pluggy/_manager.py:304 body 313 |  |  | 0.541 |
| walker |  | 9426 | 137 | python method doc at src/pluggy/_manager.py:125 |  |  | 0.541 |
| ns | 9443 |  | 231 | CHANGELOG.rst: pluggy 1.6.0 -- bug fixes | 7.2 | 7.1 | 0.536 |
| walker |  | 9572 | 146 | python method doc at src/pluggy/_manager.py:278 |  |  | 0.536 |
| walker |  | 9639 | 67 | python imports in src/pluggy/_tracing.py |  |  | 0.536 |
| ns | 9646 |  | 203 | changelog/README.rst: newsfragment type taxonomy + naming convention | 7.3 |  | 0.532 |
| walker |  | 9652 | 13 | python method body at src/pluggy/_manager.py:233 body 236 |  |  | 0.532 |
| ns | 9971 |  | 325 | RELEASING.rst: full release procedure | 7.4 |  | 0.522 |
